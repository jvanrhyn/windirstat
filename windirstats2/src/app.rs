// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use crate::extension_stats::ExtensionStatsCollector;
use crate::file_item::FileItem;
use crate::scanner::Scanner;
use crate::treemap::TreeMap;
use crate::ui;
use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Scan result message
enum ScanResult {
    InProgress,
    Completed(Arc<FileItem>),
    Error(String),
}

/// Main application state
pub struct WinDirStatsApp {
    scanner: Scanner,
    root_item: Option<Arc<FileItem>>,
    selected_item: Option<Arc<FileItem>>,
    extension_stats: ExtensionStatsCollector,
    treemap: TreeMap,
    scan_result: Arc<Mutex<Option<ScanResult>>>,
    current_path: String,
    show_about: bool,
}

impl WinDirStatsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            scanner: Scanner::new(),
            root_item: None,
            selected_item: None,
            extension_stats: ExtensionStatsCollector::new(),
            treemap: TreeMap::new(),
            scan_result: Arc::new(Mutex::new(None)),
            current_path: String::new(),
            show_about: false,
        }
    }

    fn start_scan(&mut self, path: PathBuf) {
        self.current_path = path.to_string_lossy().to_string();
        self.root_item = None;
        self.selected_item = None;
        self.treemap.clear();
        
        let result = self.scan_result.clone();
        
        // Reset result
        if let Ok(mut res) = result.lock() {
            *res = Some(ScanResult::InProgress);
        }
        
        self.scanner.scan_async(path, move |scan_result| {
            if let Ok(mut res) = result.lock() {
                *res = match scan_result {
                    Ok(root) => Some(ScanResult::Completed(root)),
                    Err(e) => Some(ScanResult::Error(e.to_string())),
                };
            }
        }).ok();
    }

    fn render_menu(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Select Directory...").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.start_scan(path);
                        }
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
    }

    fn render_status(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.scanner.is_running() {
                    let progress = self.scanner.get_progress();
                    ui::render_progress(
                        ui,
                        progress.files_scanned,
                        progress.dirs_scanned,
                        progress.total_size,
                        &progress.current_path,
                    );
                    ui.spinner();
                } else if let Some(root) = &self.root_item {
                    ui.label(format!(
                        "Total: {} files, {} directories, {}",
                        root.files,
                        root.subdirs,
                        root.size_string()
                    ));
                } else {
                    ui.label("Ready");
                }
            });
        });
    }

    fn render_about_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("About WinDirStats2")
            .open(&mut self.show_about)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("WinDirStats2");
                    ui.label("Cross-Platform Disk Usage Analyzer");
                    ui.add_space(10.0);
                    ui.label("Version 0.1.0");
                    ui.add_space(10.0);
                    ui.label("A Rust reimplementation of WinDirStat");
                    ui.label("with cross-platform support");
                    ui.add_space(10.0);
                    ui.label("Copyright © WinDirStat Team");
                    ui.label("Licensed under GPL v2");
                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        // Will be handled by window close button
                    }
                });
            });
        // Check if window was closed
        if !self.show_about {
            // Window was closed by close button
        }
    }
}

impl eframe::App for WinDirStatsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request repaint if scanning
        if self.scanner.is_running() {
            ctx.request_repaint();
        }

        // Check for scan completion
        if let Ok(mut result) = self.scan_result.lock() {
            if let Some(scan_result) = result.take() {
                match scan_result {
                    ScanResult::Completed(root) => {
                        self.extension_stats.collect(&root);
                        self.root_item = Some(root);
                    }
                    ScanResult::Error(err) => {
                        eprintln!("Scan error: {}", err);
                    }
                    ScanResult::InProgress => {
                        // Put it back
                        *result = Some(ScanResult::InProgress);
                    }
                }
            }
        }

        self.render_menu(ctx);
        self.render_status(ctx);
        
        if self.show_about {
            self.render_about_dialog(ctx);
        }

        // Main content area with splitters
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.root_item.is_none() && !self.scanner.is_running() {
                ui.vertical_centered(|ui| {
                    ui.add_space(200.0);
                    ui.heading("Welcome to WinDirStats2");
                    ui.add_space(20.0);
                    ui.label("A cross-platform disk usage analyzer");
                    ui.add_space(20.0);
                    if ui.button("Select Directory to Scan").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.start_scan(path);
                        }
                    }
                });
                return;
            }

            // Split view: Tree on left, Treemap and Extension list on right
            egui::TopBottomPanel::top("tree_panel")
                .resizable(true)
                .default_height(300.0)
                .show_inside(ui, |ui| {
                    ui.heading("Directory Tree");
                    ui.separator();
                    ui::render_tree_view(ui, &self.root_item, &mut self.selected_item);
                });

            egui::SidePanel::right("extension_panel")
                .resizable(true)
                .default_width(300.0)
                .show_inside(ui, |ui| {
                    ui.heading("File Extensions");
                    ui.separator();
                    
                    let stats: Vec<_> = self
                        .extension_stats
                        .get_sorted_stats()
                        .iter()
                        .map(|s| {
                            (
                                s.extension.clone(),
                                s.file_count,
                                s.total_size,
                                s.percentage,
                                s.color,
                            )
                        })
                        .collect();
                    
                    ui::render_extension_view(ui, &stats);
                });

            // Treemap in center
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.heading("TreeMap Visualization");
                ui.separator();
                
                if let Some(root) = &self.root_item {
                    let available = ui.available_rect_before_wrap();
                    if self.treemap.is_empty() {
                        self.treemap.build(root, &self.extension_stats, available);
                    }
                    
                    if let Some(clicked_item) = self.treemap.render(ui) {
                        self.selected_item = Some(clicked_item);
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Scanning directory...");
                    });
                }
            });
        });
    }
}
