use crate::collection::Collection;
use std::path::PathBuf;

#[derive()]
pub enum CurrentScreen {
    Main,
    Add,
    Exit,
}

pub struct App {
    collection: Collection,
    pub dir_input: String,
    pub current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> App {
        App {
            collection: Collection::new(),
            dir_input: String::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn add_directory(&mut self, directory: String) {
        self.collection.add_directory(PathBuf::from(directory));
    }

    pub fn file_list(&self) -> Vec<std::fs::DirEntry> {
        self.collection.files()
    }

    pub fn dir_list(&self) -> &Vec<PathBuf> {
        &self.collection.directories
    }

    pub fn print_collection(&self) {
        todo!()
    }

    pub(crate) fn save_directory(&mut self) {
        self.add_directory(self.dir_input.clone());
        self.dir_input.clear();
    }
}
