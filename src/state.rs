use std::path::PathBuf;

use iced::widget::text_editor;
use iced::window;

use crate::message::{FileAction, ViewAction};
use crate::{constants, file};

#[derive(Default, Copy, Clone)]
pub enum Mode {
  #[default]
  Edit,
  Preview,
}

#[derive(Default)]
pub struct State {
  window_id: Option<window::Id>,
  mode: Mode,
  files: Vec<file::File>,
  current_file: usize,
  editor_font_size: u32,
  selected_file_action: Option<FileAction>,
  selected_view_action: Option<ViewAction>,
}

impl State {
  pub fn new(window_id: window::Id) -> Self {
    let default_file = file::File::default();

    Self {
      window_id: Some(window_id),
      files: vec![default_file],
      current_file: 0,
      editor_font_size: constants::DEFAULT_EDITOR_FONT_SIZE,
      ..Default::default()
    }
  }

  pub fn set_active_file_save_status(&mut self, status: bool) {
    self.files[self.current_file].set_needs_saving(status);
  }

  pub fn apply_edit(&mut self, action: text_editor::Action) {
    self.files[self.current_file].content_mut().perform(action.clone());
    
    if matches!(action, text_editor::Action::Edit(_)) {
      self.files[self.current_file].set_needs_saving(true);
    }
  }

  pub fn new_file(&mut self) {
    self.files.push(file::File::default());
    self.current_file = self.files.len() - 1;
  }

  pub fn close_file(&mut self, index: Option<usize>) -> bool {
    let index = index.unwrap_or(self.current_file);

    if self.files.len() <= 1 {
      return true;
    }

    self.files.remove(index);

    if self.current_file >= self.files.len() {
      self.current_file = self.files.len().saturating_sub(1);
    } else if index < self.current_file {
      self.current_file -= 1;
    }

    false
  }

  pub fn open_file(&mut self, path: PathBuf, content: String) {
    if let Some(index) = self.files.iter().position(|f| f.path() == Some(&path)) {
      self.files[index].set_content(&content);
      self.current_file = index;
      return;
    }

    let opened_file = file::File::from(&content, Some(path));

    self.files.push(opened_file);
    self.current_file = self.files.len() - 1;
  }

  pub fn switch_tab(&mut self, index: usize) {
    if index < self.files.len() {
      self.current_file = index;
    }
  }

  pub fn active_file_data(&self) -> (Option<&PathBuf>, String) {
    let file = &self.files[self.current_file];
    (file.path(), file.content().text())
  }

  pub fn set_active_file_path(&mut self, path: PathBuf) {
    if let Some(file) = self.files.get_mut(self.current_file) {
      file.set_path(Some(path));
    }
  }

  pub fn increase_font(&mut self) {
    if self.editor_font_size < constants::MAX_EDITOR_FONT_SIZE {
      self.editor_font_size += 2;
    }
  }

  pub fn decrease_font(&mut self) {
    if self.editor_font_size > constants::MIN_EDITOR_FONT_SIZE {
      self.editor_font_size -= 2;
    }
  }

  pub fn reset_font(&mut self) {
    self.editor_font_size = constants::DEFAULT_EDITOR_FONT_SIZE;
  }

  pub fn toggle_preview(&mut self) {
    match self.mode {
      Mode::Edit => {
        self.files[self.current_file].update_markdown();
        self.mode = Mode::Preview;
      }
      Mode::Preview => self.mode = Mode::Edit,
    }
  }

  pub fn files(&self) -> &[file::File] {
    &self.files
  }

  pub fn current_file_index(&self) -> usize {
    self.current_file
  }

  pub fn active_file(&self) -> &file::File {
    &self.files[self.current_file]
  }

  pub fn mode(&self) -> Mode {
    self.mode
  }

  pub fn window_id(&self) -> Option<window::Id> {
    self.window_id
  }

  pub fn font_size(&self) -> u32 {
    self.editor_font_size
  }

  pub fn selected_file_action(&self) -> Option<FileAction> {
    self.selected_file_action
  }

  pub fn selected_view_action(&self) -> Option<ViewAction> {
    self.selected_view_action
  }

  pub fn set_window_id(&mut self, id: window::Id) {
    self.window_id = Some(id)
  }
}
