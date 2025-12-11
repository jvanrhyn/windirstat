// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use crate::file_item::FileItem;
use crate::extension_stats::ExtensionStatsCollector;
use egui::{Color32, Pos2, Rect, Sense, Ui, Vec2};
use std::sync::Arc;

/// Rectangle in the treemap with associated file item
#[derive(Debug, Clone)]
pub struct TreeMapRect {
    pub rect: Rect,
    pub item: Arc<FileItem>,
    pub color: Color32,
}

/// Treemap visualization
pub struct TreeMap {
    rectangles: Vec<TreeMapRect>,
    selected_item: Option<Arc<FileItem>>,
}

impl TreeMap {
    pub fn new() -> Self {
        Self {
            rectangles: Vec::new(),
            selected_item: None,
        }
    }

    /// Check if treemap is empty
    pub fn is_empty(&self) -> bool {
        self.rectangles.is_empty()
    }

    /// Build treemap layout from file tree
    pub fn build(
        &mut self,
        root: &Arc<FileItem>,
        extension_stats: &ExtensionStatsCollector,
        available_rect: Rect,
    ) {
        self.rectangles.clear();
        
        if root.size > 0 {
            self.build_recursive(root, available_rect, extension_stats);
        }
    }

    fn build_recursive(
        &mut self,
        item: &Arc<FileItem>,
        rect: Rect,
        extension_stats: &ExtensionStatsCollector,
    ) {
        let width = rect.width();
        let height = rect.height();

        // Don't render very small rectangles
        if width < 2.0 || height < 2.0 {
            return;
        }

        // If this is a file, render it
        if !item.is_dir {
            let color = self.get_item_color(item, extension_stats);
            self.rectangles.push(TreeMapRect {
                rect,
                item: item.clone(),
                color,
            });
            return;
        }

        // For directories, recursively layout children
        if item.children.is_empty() {
            return;
        }

        // Calculate layout using squarified treemap algorithm
        let mut children = item.children.clone();
        children.sort_by(|a, b| b.size.cmp(&a.size));

        self.squarify(&children, rect, extension_stats);
    }

    fn squarify(
        &mut self,
        items: &[Arc<FileItem>],
        rect: Rect,
        extension_stats: &ExtensionStatsCollector,
    ) {
        if items.is_empty() {
            return;
        }

        let total_size: u64 = items.iter().map(|i| i.size).sum();
        if total_size == 0 {
            return;
        }

        let area = rect.width() * rect.height();
        let mut remaining_rect = rect;
        let mut remaining_items = items.to_vec();

        while !remaining_items.is_empty() {
            let row = self.get_row(&remaining_items, &remaining_rect, area, total_size);
            let row_size: u64 = row.iter().map(|i| i.size).sum();
            
            let (new_remaining, _used_rect) = 
                self.layout_row(&row, remaining_rect, row_size, total_size, extension_stats);
            
            remaining_rect = new_remaining;
            remaining_items.drain(0..row.len());
        }
    }

    fn get_row(
        &self,
        items: &[Arc<FileItem>],
        rect: &Rect,
        _area: f32,
        total_size: u64,
    ) -> Vec<Arc<FileItem>> {
        if items.is_empty() {
            return Vec::new();
        }

        let mut row = vec![items[0].clone()];
        let width = rect.width();
        let height = rect.height();
        
        if width < 1.0 || height < 1.0 {
            return row;
        }

        let vertical = height > width;
        
        for i in 1..items.len().min(100) {
            let test_row: Vec<_> = items[0..=i].to_vec();
            let row_size: u64 = test_row.iter().map(|item| item.size).sum();
            
            if row_size == 0 {
                break;
            }

            let ratio = row_size as f64 / total_size as f64;
            let length = if vertical { width } else { height };
            
            if ratio * length as f64 > 1.0 {
                row = test_row;
            } else {
                break;
            }
        }

        row
    }

    fn layout_row(
        &mut self,
        row: &[Arc<FileItem>],
        rect: Rect,
        row_size: u64,
        total_size: u64,
        extension_stats: &ExtensionStatsCollector,
    ) -> (Rect, Rect) {
        let width = rect.width();
        let height = rect.height();
        let vertical = height > width;

        let ratio = row_size as f32 / total_size as f32;
        let breadth = if vertical { width * ratio } else { height * ratio };

        let mut offset = 0.0;

        for item in row {
            let item_ratio = item.size as f32 / row_size as f32;
            let length = if vertical { height * item_ratio } else { width * item_ratio };

            let item_rect = if vertical {
                Rect::from_min_size(
                    Pos2::new(rect.min.x, rect.min.y + offset),
                    Vec2::new(breadth, length),
                )
            } else {
                Rect::from_min_size(
                    Pos2::new(rect.min.x + offset, rect.min.y),
                    Vec2::new(length, breadth),
                )
            };

            self.build_recursive(item, item_rect, extension_stats);
            offset += length;
        }

        // Return remaining rectangle
        let remaining = if vertical {
            Rect::from_min_size(
                Pos2::new(rect.min.x + breadth, rect.min.y),
                Vec2::new(width - breadth, height),
            )
        } else {
            Rect::from_min_size(
                Pos2::new(rect.min.x, rect.min.y + breadth),
                Vec2::new(width, height - breadth),
            )
        };

        let _used = if vertical {
            Rect::from_min_size(rect.min, Vec2::new(breadth, height))
        } else {
            Rect::from_min_size(rect.min, Vec2::new(width, breadth))
        };

        (remaining, _used)
    }

    fn get_item_color(
        &self,
        item: &FileItem,
        extension_stats: &ExtensionStatsCollector,
    ) -> Color32 {
        let ext = item.extension.as_deref().unwrap_or("<no extension>");
        let color = extension_stats.get_color(ext);
        Color32::from_rgb(color[0], color[1], color[2])
    }

    /// Render the treemap
    pub fn render(&mut self, ui: &mut Ui) -> Option<Arc<FileItem>> {
        let available = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available, Sense::click());
        
        let painter = ui.painter();
        
        // Draw all rectangles
        for tree_rect in &self.rectangles {
            let mut color = tree_rect.color;
            
            // Highlight selected item
            if let Some(ref selected) = self.selected_item {
                if Arc::ptr_eq(&tree_rect.item, selected) {
                    color = Color32::from_rgb(255, 255, 0); // Yellow highlight
                }
            }
            
            painter.rect_filled(tree_rect.rect, 0.0, color);
            painter.rect_stroke(tree_rect.rect, 0.0, (1.0, Color32::from_gray(100)));
        }

        // Handle clicks
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                for tree_rect in &self.rectangles {
                    if tree_rect.rect.contains(pos) {
                        self.selected_item = Some(tree_rect.item.clone());
                        return Some(tree_rect.item.clone());
                    }
                }
            }
        }

        // Show tooltip on hover
        if let Some(pos) = response.hover_pos() {
            for tree_rect in &self.rectangles {
                if tree_rect.rect.contains(pos) {
                    response.on_hover_ui(|ui| {
                        ui.label(format!("Path: {}", tree_rect.item.path_string()));
                        ui.label(format!("Size: {}", tree_rect.item.size_string()));
                        if let Some(ext) = &tree_rect.item.extension {
                            ui.label(format!("Type: .{}", ext));
                        }
                    });
                    break;
                }
            }
        }

        None
    }

    pub fn clear(&mut self) {
        self.rectangles.clear();
        self.selected_item = None;
    }
}

impl Default for TreeMap {
    fn default() -> Self {
        Self::new()
    }
}
