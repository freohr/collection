use ratatui::widgets::ListState;

use crate::collection::Collection;
use std::{path::PathBuf, usize};

#[derive()]
pub enum CurrentScreen {
    Main,
    Add,
    Exit,
}

pub enum CurrentBlock {
    Files,
    Directories,
}

pub struct App {
    collection: Collection,
    pub dir_input: String,
    pub current_screen: CurrentScreen,
    pub current_block: CurrentBlock,
    pub selected_file: ListState,
    pub selected_dir: ListState,
}

impl App {
    pub fn new() -> App {
        App {
            collection: Collection::new(),
            dir_input: String::new(),
            current_screen: CurrentScreen::Main,
            current_block: CurrentBlock::Files,
            selected_file: ListState::default(),
            selected_dir: ListState::default(),
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
        if self.collection.dir_count() == 1 {
            App::next_element(&mut self.selected_file, self.collection.file_count());
            App::next_element(&mut self.selected_dir, 1);
        }
    }

    fn next_element(state: &mut ListState, len: usize) {
        if len == 0 {
            return;
        }

        let i = match state.selected() {
            Some(i) => (i + 1).clamp(0, len - 1),
            None => 0,
        };
        state.select(Some(i));
    }

    pub(crate) fn next(&mut self) {
        match self.current_block {
            CurrentBlock::Files => {
                App::next_element(&mut self.selected_file, self.collection.file_count())
            }
            CurrentBlock::Directories => {
                App::next_element(&mut self.selected_dir, self.collection.dir_count())
            }
        }
    }

    fn previous_element(state: &mut ListState, len: usize) {
        if len == 0 {
            return;
        }

        let i = match state.selected() {
            Some(0) | None => 0,
            Some(i) => (i - 1).clamp(0, len - 1),
        };
        state.select(Some(i));
    }

    pub(crate) fn previous(&mut self) {
        match self.current_block {
            CurrentBlock::Files => {
                App::previous_element(&mut self.selected_file, self.collection.file_count());
            }
            CurrentBlock::Directories => {
                App::previous_element(&mut self.selected_dir, self.collection.dir_count());
            }
        }
    }
}
