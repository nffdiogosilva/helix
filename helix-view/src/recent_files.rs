use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

pub const DEFAULT_MAX_ENTRIES: usize = 200;

pub fn recent_files_path() -> PathBuf {
    helix_loader::cache_dir().join("recent-files.json")
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RecentFileEntry {
    pub path: PathBuf,
    pub last_opened: SystemTime,
}

#[derive(Debug)]
pub struct RecentFiles {
    entries: VecDeque<RecentFileEntry>,
    max_entries: usize,
}

impl RecentFiles {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_entries,
        }
    }

    pub fn push(&mut self, path: PathBuf) {
        self.entries.retain(|e| e.path != path);
        self.entries.push_front(RecentFileEntry {
            path,
            last_opened: SystemTime::now(),
        });
        while self.entries.len() > self.max_entries {
            self.entries.pop_back();
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &RecentFileEntry> {
        self.entries.iter()
    }

    pub fn load(path: &Path) -> Self {
        let data = match fs::read_to_string(path) {
            Ok(d) => d,
            Err(err) => {
                log::warn!("Failed to read recent files from {:?}: {}", path, err);
                return Self::new(DEFAULT_MAX_ENTRIES);
            }
        };

        let entries: VecDeque<RecentFileEntry> = match serde_json::from_str(&data) {
            Ok(e) => e,
            Err(err) => {
                log::warn!("Failed to parse recent files from {:?}: {}", path, err);
                return Self::new(DEFAULT_MAX_ENTRIES);
            }
        };

        Self {
            entries,
            max_entries: DEFAULT_MAX_ENTRIES,
        }
    }

    pub fn save(&self, path: &Path) {
        let data = match serde_json::to_string(&self.entries) {
            Ok(d) => d,
            Err(err) => {
                log::warn!("Failed to serialize recent files: {}", err);
                return;
            }
        };

        if let Err(err) = fs::write(path, data) {
            log::warn!("Failed to write recent files to {:?}: {}", path, err);
        }
    }
}
