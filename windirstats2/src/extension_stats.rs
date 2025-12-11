// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use crate::file_item::FileItem;
use std::collections::HashMap;
use std::sync::Arc;

/// Statistics for a file extension
#[derive(Debug, Clone, Default)]
pub struct ExtensionStats {
    pub extension: String,
    pub file_count: u64,
    pub total_size: u64,
    pub percentage: f32,
    pub color: [u8; 3], // RGB color for treemap
}

impl ExtensionStats {
    pub fn new(extension: String) -> Self {
        Self {
            extension,
            file_count: 0,
            total_size: 0,
            percentage: 0.0,
            color: [128, 128, 128],
        }
    }

    pub fn add_file(&mut self, size: u64) {
        self.file_count += 1;
        self.total_size += size;
    }
}

/// Collects and manages extension statistics
pub struct ExtensionStatsCollector {
    stats: HashMap<String, ExtensionStats>,
    total_size: u64,
}

impl ExtensionStatsCollector {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
            total_size: 0,
        }
    }

    /// Collect statistics from a file tree
    pub fn collect(&mut self, root: &Arc<FileItem>) {
        self.stats.clear();
        self.total_size = 0;
        self.collect_recursive(root);
        self.calculate_percentages();
        self.assign_colors();
    }

    fn collect_recursive(&mut self, item: &FileItem) {
        if !item.is_dir {
            let ext = item.extension.clone().unwrap_or_else(|| "<no extension>".to_string());
            let stats = self.stats.entry(ext.clone()).or_insert_with(|| ExtensionStats::new(ext));
            stats.add_file(item.size);
            self.total_size += item.size;
        }

        for child in &item.children {
            self.collect_recursive(child);
        }
    }

    fn calculate_percentages(&mut self) {
        if self.total_size > 0 {
            for stat in self.stats.values_mut() {
                stat.percentage = (stat.total_size as f64 / self.total_size as f64 * 100.0) as f32;
            }
        }
    }

    fn assign_colors(&mut self) {
        // Assign colors based on a color palette
        let palette = Self::generate_color_palette(self.stats.len());
        
        let mut sorted_exts: Vec<_> = self.stats.keys().cloned().collect();
        sorted_exts.sort_by(|a, b| {
            self.stats[b].total_size.cmp(&self.stats[a].total_size)
        });

        for (i, ext) in sorted_exts.iter().enumerate() {
            if let Some(stat) = self.stats.get_mut(ext) {
                stat.color = palette[i % palette.len()];
            }
        }
    }

    fn generate_color_palette(count: usize) -> Vec<[u8; 3]> {
        // Generate a visually distinct color palette
        let base_colors = vec![
            [255, 99, 71],   // Tomato
            [60, 179, 113],  // Medium Sea Green
            [30, 144, 255],  // Dodger Blue
            [255, 215, 0],   // Gold
            [218, 112, 214], // Orchid
            [255, 140, 0],   // Dark Orange
            [0, 255, 255],   // Cyan
            [255, 105, 180], // Hot Pink
            [124, 252, 0],   // Lawn Green
            [147, 112, 219], // Medium Purple
            [255, 69, 0],    // Orange Red
            [0, 206, 209],   // Dark Turquoise
            [255, 20, 147],  // Deep Pink
            [127, 255, 0],   // Chartreuse
            [72, 61, 139],   // Dark Slate Blue
        ];

        let mut palette = Vec::new();
        let needed = count.max(1);

        // Repeat and vary colors if we need more
        for i in 0..needed {
            let base = base_colors[i % base_colors.len()];
            let variation = ((i / base_colors.len()) as f32 * 0.3).min(0.9);
            
            palette.push([
                (base[0] as f32 * (1.0 - variation)) as u8,
                (base[1] as f32 * (1.0 - variation)) as u8,
                (base[2] as f32 * (1.0 - variation)) as u8,
            ]);
        }

        palette
    }

    /// Get sorted list of extension statistics
    pub fn get_sorted_stats(&self) -> Vec<ExtensionStats> {
        let mut stats: Vec<_> = self.stats.values().cloned().collect();
        stats.sort_by(|a, b| b.total_size.cmp(&a.total_size));
        stats
    }

    /// Get color for an extension
    pub fn get_color(&self, extension: &str) -> [u8; 3] {
        self.stats
            .get(extension)
            .map(|s| s.color)
            .unwrap_or([128, 128, 128])
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }
}

impl Default for ExtensionStatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_extension_stats() {
        let mut stats = ExtensionStats::new("txt".to_string());
        stats.add_file(1024);
        stats.add_file(2048);
        
        assert_eq!(stats.file_count, 2);
        assert_eq!(stats.total_size, 3072);
    }

    #[test]
    fn test_color_palette() {
        let palette = ExtensionStatsCollector::generate_color_palette(10);
        assert_eq!(palette.len(), 10);
    }
}
