mod file;
mod tag;

use std::path::PathBuf;
use std::{
    fs::{self, DirEntry},
    usize,
};

pub struct Collection {
    pub directories: Vec<PathBuf>,
}

impl Default for Collection {
    fn default() -> Self {
        Collection::new()
    }
}

impl Collection {
    pub fn new() -> Collection {
        Collection {
            directories: vec![],
        }
    }

    pub fn dir_count(&self) -> usize {
        self.directories.len()
    }

    pub fn file_count(&self) -> usize {
        self.files().len()
    }

    pub fn files(&self) -> Vec<DirEntry> {
        let mut files: Vec<DirEntry> = vec![];

        for directory in &self.directories {
            if let Ok(entry) = fs::read_dir(directory) {
                entry.flatten().for_each(|file| {
                    files.push(file);
                });
            }
        }

        files
    }

    pub fn add_directory(&mut self, directory: PathBuf) {
        self.directories.push(directory);
    }
}
