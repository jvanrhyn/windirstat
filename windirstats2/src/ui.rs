// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use crate::file_item::{FileItem, format_size};
use egui::{Color32, RichText, ScrollArea, Ui};
use std::sync::Arc;

/// Render the directory tree view
pub fn render_tree_view(
    ui: &mut Ui,
    root: &Option<Arc<FileItem>>,
    selected: &mut Option<Arc<FileItem>>,
) {
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if let Some(root) = root {
                render_tree_item(ui, root, selected, 0);
            } else {
                ui.label("No directory scanned. Select a directory to begin.");
            }
        });
}

fn render_tree_item(
    ui: &mut Ui,
    item: &Arc<FileItem>,
    selected: &mut Option<Arc<FileItem>>,
    depth: usize,
) {
    let indent = (depth * 20) as f32;
    
    ui.horizontal(|ui| {
        ui.add_space(indent);
        
        let is_selected = selected
            .as_ref()
            .map(|s| Arc::ptr_eq(s, item))
            .unwrap_or(false);
        
        let icon = if item.is_dir {
            if item.children.is_empty() { "📁" } else { "📂" }
        } else {
            "📄"
        };
        
        let label = format!(
            "{} {} ({}, {:.1}%)",
            icon,
            item.name,
            item.size_string(),
            item.percent
        );
        
        let text = if is_selected {
            RichText::new(label).color(Color32::YELLOW)
        } else {
            RichText::new(label)
        };
        
        if ui.selectable_label(is_selected, text).clicked() {
            *selected = Some(item.clone());
        }
    });
    
    // Render children for directories
    if item.is_dir && !item.children.is_empty() {
        for child in &item.children {
            render_tree_item(ui, child, selected, depth + 1);
        }
    }
}

/// Render the extension statistics view
pub fn render_extension_view(
    ui: &mut Ui,
    stats: &[(String, u64, u64, f32, [u8; 3])],
) {
    ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if stats.is_empty() {
                ui.label("No file statistics available.");
                return;
            }
            
            // Header
            ui.horizontal(|ui| {
                ui.label(RichText::new("Extension").strong());
                ui.separator();
                ui.label(RichText::new("Files").strong());
                ui.separator();
                ui.label(RichText::new("Size").strong());
                ui.separator();
                ui.label(RichText::new("Percent").strong());
            });
            
            ui.separator();
            
            // Data rows
            for (ext, count, size, percent, color) in stats {
                ui.horizontal(|ui| {
                    // Color indicator
                    let color = Color32::from_rgb(color[0], color[1], color[2]);
                    ui.colored_label(color, "■");
                    
                    ui.label(ext);
                    ui.separator();
                    ui.label(format!("{}", count));
                    ui.separator();
                    ui.label(format_size(*size));
                    ui.separator();
                    ui.label(format!("{:.2}%", percent));
                });
            }
        });
}

/// Render scan progress
pub fn render_progress(ui: &mut Ui, files: u64, dirs: u64, size: u64, current_path: &str) {
    ui.horizontal(|ui| {
        ui.label(format!("Files: {}", files));
        ui.separator();
        ui.label(format!("Directories: {}", dirs));
        ui.separator();
        ui.label(format!("Total Size: {}", format_size(size)));
    });
    
    if !current_path.is_empty() {
        ui.label(format!("Scanning: {}", current_path));
    }
}
