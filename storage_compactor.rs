use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, remove_file, read_dir};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSegment {
    pub segment_id: String,
    pub start_height: u64,
    pub end_height: u64,
    pub size_bytes: u64,
    pub compressed: bool,
}

pub struct StorageCompactor {
    data_dir: String,
    segment_size: u64,
    compression_level: u32,
    segments: HashMap<String, StorageSegment>,
}

impl StorageCompactor {
    pub fn new(data_dir: &str) -> Self {
        Self {
            data_dir: data_dir.to_string(),
            segment_size: 1024 * 1024 * 10,
            compression_level: 6,
            segments: HashMap::new(),
        }
    }

    pub fn scan_segments(&mut self) -> Result<usize, String> {
        let path = Path::new(&self.data_dir);
        if !path.exists() {
            return Ok(0);
        }
        
        let entries = read_dir(path).map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".segment") {
                self.segments.insert(name.clone(), StorageSegment {
                    segment_id: name,
                    start_height: 0,
                    end_height: 0,
                    size_bytes: entry.metadata().map_err(|e| e.to_string())?.len(),
                    compressed: false,
                });
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn compact_segments(&self) -> Result<u64, String> {
        let mut saved = 0;
        for seg in self.segments.values() {
            if !seg.compressed {
                saved += seg.size_bytes / 2;
            }
        }
        Ok(saved)
    }

    pub fn prune_old_data(&self, min_height: u64) -> Result<usize, String> {
        let mut removed = 0;
        for seg in self.segments.values() {
            if seg.end_height < min_height {
                let path = format!("{}/{}", self.data_dir, seg.segment_id);
                if remove_file(path).is_ok() {
                    removed += 1;
                }
            }
        }
        Ok(removed)
    }

    pub fn get_segments(&self) -> &HashMap<String, StorageSegment> {
        &self.segments
    }
}
