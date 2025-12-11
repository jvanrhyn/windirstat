// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

/// Represents a file or directory in the file system
#[derive(Debug, Clone)]
pub struct FileItem {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub items: u64, // Number of items (files + subdirectories)
    pub files: u64, // Number of files
    pub subdirs: u64, // Number of subdirectories
    pub is_dir: bool,
    pub extension: Option<String>,
    pub modified: Option<SystemTime>,
    pub children: Vec<Arc<FileItem>>,
    pub percent: f32, // Percentage of parent
}

impl FileItem {
    /// Create a new file item
    pub fn new(path: PathBuf, name: String, is_dir: bool) -> Self {
        let extension = if !is_dir {
            path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
        } else {
            None
        };

        Self {
            path,
            name,
            size: 0,
            items: 0,
            files: 0,
            subdirs: 0,
            is_dir,
            extension,
            modified: None,
            children: Vec::new(),
            percent: 0.0,
        }
    }

    /// Create a file item from metadata
    pub fn from_path(path: &Path, size: u64, is_dir: bool, modified: Option<SystemTime>) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let extension = if !is_dir {
            path.extension()
                .and_then(|e| e.to_str())
                .map(|s| s.to_lowercase())
        } else {
            None
        };

        Self {
            path: path.to_path_buf(),
            name,
            size,
            items: if is_dir { 0 } else { 1 },
            files: if is_dir { 0 } else { 1 },
            subdirs: 0,
            is_dir,
            extension,
            modified,
            children: Vec::new(),
            percent: 0.0,
        }
    }

    /// Add a child item and update statistics
    pub fn add_child(&mut self, child: Arc<FileItem>) {
        self.size += child.size;
        self.items += child.items;
        self.files += child.files;
        self.subdirs += child.subdirs;
        if child.is_dir {
            self.subdirs += 1;
        }
        self.children.push(child);
    }

    /// Sort children by size (descending)
    pub fn sort_children(&mut self) {
        self.children.sort_by(|a, b| b.size.cmp(&a.size));
        for child in &mut self.children {
            Arc::get_mut(child).map(|c| c.sort_children());
        }
    }

    /// Calculate percentage of total for this item and all children
    pub fn calculate_percentages(&mut self, total: u64) {
        if total > 0 {
            self.percent = (self.size as f64 / total as f64 * 100.0) as f32;
        }
        for child in &mut self.children {
            Arc::get_mut(child).map(|c| c.calculate_percentages(total));
        }
    }

    /// Get display size as human-readable string
    pub fn size_string(&self) -> String {
        format_size(self.size)
    }

    /// Get the full path as a string
    pub fn path_string(&self) -> String {
        self.path.to_string_lossy().to_string()
    }
}

/// Format bytes as human-readable size
pub fn format_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let exponent = (bytes_f.log(1024.0).floor() as usize).min(UNITS.len() - 1);
    let value = bytes_f / 1024_f64.powi(exponent as i32);
    
    if exponent == 0 {
        format!("{} {}", bytes, UNITS[exponent])
    } else {
        format!("{:.2} {}", value, UNITS[exponent])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(1023), "1023 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_file_item_creation() {
        let item = FileItem::new(
            PathBuf::from("/test/file.txt"),
            "file.txt".to_string(),
            false,
        );
        assert_eq!(item.name, "file.txt");
        assert_eq!(item.extension, Some("txt".to_string()));
        assert!(!item.is_dir);
    }
}
