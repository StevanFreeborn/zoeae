use std::path::PathBuf;

use crate::io;
use crate::message::{FileAction, ViewAction};
use crate::{Message, state::State};
use iced::Task;
use iced::widget::text_editor;

pub fn edit(state: &mut State, action: text_editor::Action) -> Task<Message> {
  state.apply_edit(action);
  Task::none()
}

pub fn switch_tab(state: &mut State, index: usize) -> Task<Message> {
  state.switch_tab(index);
  Task::none()
}

pub fn link_clicked(url: String) -> Task<Message> {
  let _ = webbrowser::open(&url);
  Task::none()
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
    FileAction::Open => Task::perform(io::open_file(), Message::FileOpened),
    FileAction::Save => {
      let (current_path, content) = state.active_file_data();
      let path = current_path.cloned();
      Task::perform(io::save_file(path, content), Message::FileSaved)
    }
    FileAction::SaveAs => {
      let (_, content) = state.active_file_data();
      Task::perform(io::save_file(None, content), Message::FileSaved)
    }
  }
}

pub fn opened_file(state: &mut State, result: Result<(PathBuf, String), String>) -> Task<Message> {
  match result {
    Ok((path, content)) => {
      state.open_file(path, content);
    }
    Err(_error) => {}
  };

  Task::none()
}

pub fn saved_file(state: &mut State, result: Result<PathBuf, String>) -> Task<Message> {
  match result {
    Ok(path) => {
      state.set_active_file_path(path);
      state.set_active_file_save_status(false);
    }
    Err(_error) => {}
  };

  Task::none()
}

pub fn view_action(state: &mut State, action: ViewAction) -> Task<Message> {
  match action {
    ViewAction::Increase => state.increase_font(),
    ViewAction::Decrease => state.decrease_font(),
    ViewAction::Reset => state.reset_font(),
    ViewAction::TogglePreview => state.toggle_preview(),
    ViewAction::ToggleWordWrap => state.toggle_word_wrap(),
  }

  Task::none()
}
