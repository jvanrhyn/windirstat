// WinDirStats2 - Cross-Platform Disk Usage Analyzer
// Copyright © WinDirStat Team
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// at your option any later version.

use crate::file_item::FileItem;
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use walkdir::WalkDir;

/// Progress information for scanning
#[derive(Debug, Clone, Default)]
pub struct ScanProgress {
    pub files_scanned: u64,
    pub dirs_scanned: u64,
    pub total_size: u64,
    pub current_path: String,
    pub is_complete: bool,
}

/// Scanner for analyzing directory structure
pub struct Scanner {
    running: Arc<AtomicBool>,
    progress: Arc<Mutex<ScanProgress>>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            progress: Arc::new(Mutex::new(ScanProgress::default())),
        }
    }

    /// Start scanning a directory in a background thread
    pub fn scan_async<F>(
        &mut self,
        path: PathBuf,
        callback: F,
    ) -> Result<()>
    where
        F: FnOnce(Result<Arc<FileItem>>) + Send + 'static,
    {
        if self.running.load(Ordering::SeqCst) {
            anyhow::bail!("Scanner is already running");
        }

        self.running.store(true, Ordering::SeqCst);
        let running = self.running.clone();
        let progress = self.progress.clone();

        thread::spawn(move || {
            let result = Self::scan_directory(&path, &running, &progress);
            running.store(false, Ordering::SeqCst);
            
            if let Ok(ref mut prog) = progress.lock() {
                prog.is_complete = true;
            }
            
            callback(result);
        });

        Ok(())
    }

    /// Check if scanner is currently running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Stop the current scan
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Get current progress
    pub fn get_progress(&self) -> ScanProgress {
        self.progress.lock().unwrap().clone()
    }

    /// Scan a directory and build the file tree
    fn scan_directory(
        path: &Path,
        running: &Arc<AtomicBool>,
        progress: &Arc<Mutex<ScanProgress>>,
    ) -> Result<Arc<FileItem>> {
        // Create root item
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        let mut root = FileItem::new(path.to_path_buf(), name, true);

        // Build directory tree structure
        let mut dir_map: HashMap<PathBuf, Vec<Arc<FileItem>>> = HashMap::new();
        
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !running.load(Ordering::SeqCst) {
                anyhow::bail!("Scan cancelled");
            }

            let entry_path = entry.path();
            if entry_path == path {
                continue; // Skip root
            }

            let metadata = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let is_dir = metadata.is_dir();
            let size = if is_dir { 0 } else { metadata.len() };
            let modified = metadata.modified().ok();

            let item = Arc::new(FileItem::from_path(entry_path, size, is_dir, modified));

            // Update progress
            if let Ok(mut prog) = progress.lock() {
                if is_dir {
                    prog.dirs_scanned += 1;
                } else {
                    prog.files_scanned += 1;
                    prog.total_size += size;
                }
                prog.current_path = entry_path.to_string_lossy().to_string();
            }

            // Add to parent's children
            if let Some(parent) = entry_path.parent() {
                dir_map.entry(parent.to_path_buf())
                    .or_default()
                    .push(item);
            }
        }

        // Build tree from bottom up
        Self::build_tree(&mut root, &mut dir_map)?;
        
        // Sort children by size
        root.sort_children();
        
        // Calculate percentages
        let total = root.size;
        root.calculate_percentages(total);

        Ok(Arc::new(root))
    }

    /// Build tree structure from flat directory map
    fn build_tree(
        root: &mut FileItem,
        dir_map: &mut HashMap<PathBuf, Vec<Arc<FileItem>>>,
    ) -> Result<()> {
        if let Some(children) = dir_map.remove(&root.path) {
            for child in children {
                // Recursively build subtree for directories
                if child.is_dir {
                    if let Some(mut_child) = Arc::get_mut(&mut child.clone()) {
                        Self::build_tree(mut_child, dir_map)?;
                    }
                }
                root.add_child(child);
            }
        }
        Ok(())
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    #[test]
    fn test_scan_empty_dir() {
        let temp_dir = std::env::temp_dir().join("windirstats2_test");
        fs::create_dir_all(&temp_dir).unwrap();
        
        let running = Arc::new(AtomicBool::new(true));
        let progress = Arc::new(Mutex::new(ScanProgress::default()));
        
        let result = Scanner::scan_directory(&temp_dir, &running, &progress);
        assert!(result.is_ok());
        
        fs::remove_dir_all(&temp_dir).ok();
    }
}
