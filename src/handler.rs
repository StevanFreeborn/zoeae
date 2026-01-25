use std::path::PathBuf;

use crate::io;
use crate::message::{FileAction, ViewAction};
use crate::{Message, state::State};
use iced::Task;
use iced::widget::text_editor;
use rfd::FileDialog;

pub fn edit(state: &mut State, action: text_editor::Action) -> Task<Message> {
    state.apply_edit(action);
    Task::none()
}

pub fn switch_tab(state: &mut State, index: usize) -> Task<Message> {
    state.switch_tab(index);
    Task::none()
}

pub fn link_clicked(url: String) -> Task<Message> {
    println!("Opening link: {}", url);
    Task::none()
}

fn save_file(path: Option<&PathBuf>, text: String) -> Option<PathBuf> {
    let mut save_path = path.cloned();

    if path.is_none() {
        save_path = FileDialog::new().set_directory("/").save_file();
    }

    match save_path {
        Some(p) => {
            let sp = p.clone();
            io::save_file_to_disk(sp, text);
            Some(p)
        }
        None => None,
    }
}

fn save_file_as(text: String) -> Option<PathBuf> {
    let files = FileDialog::new().set_directory("/").save_file();

    match files {
        Some(path) => {
            io::save_file_to_disk(path.clone(), text);
            Some(path)
        }
        None => None,
    }
}

fn open_file() -> (Option<PathBuf>, String) {
    let file = FileDialog::new().set_directory("/").pick_file();

    match file {
        Some(path) => {
            let content = io::load_file_from_disk(path.clone());
            (Some(path), content)
        }
        None => (None, String::new()),
    }
}

pub fn file_action(state: &mut State, action: FileAction) -> Task<Message> {
    match action {
        FileAction::New => {
            state.new_file();
            Task::none()
        }
        FileAction::Close(index) => {
            if state.close_file(index) {
                iced::exit()
            } else {
                Task::none()
            }
        }
        FileAction::Open => {
            let (path, content) = open_file();
            
            if let Some(p) = path {
                state.open_file(p, content);
            }

            Task::none()
        }
        FileAction::Save => {
            let (current_path, content) = state.active_file_data();

            if let Some(path) = save_file(current_path, content) {
                state.set_active_file_path(path);
            }

            Task::none()
        }
        FileAction::SaveAs => {
            let (_, content) = state.active_file_data();

            if let Some(path) = save_file_as(content) {
                state.set_active_file_path(path);
            }

            Task::none()
        }
    }
}

pub fn view_action(state: &mut State, action: ViewAction) -> Task<Message> {
    match action {
        ViewAction::Increase => state.increase_font(),
        ViewAction::Decrease => state.decrease_font(),
        ViewAction::Reset => state.reset_font(),
        ViewAction::TogglePreview => state.toggle_preview(),
    }

    Task::none()
}
