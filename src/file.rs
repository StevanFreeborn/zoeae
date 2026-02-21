use std::{
  ffi,
  path::{Path, PathBuf},
};

use iced::widget::{markdown, text_editor};

pub struct File {
  needs_saving: bool,
  content: text_editor::Content,
  path: Option<PathBuf>,
  markdown: Vec<markdown::Item>,
}

impl Default for File {
  fn default() -> Self {
    File {
      needs_saving: false,
      content: text_editor::Content::new(),
      path: None,
      markdown: Vec::new(),
    }
  }
}

impl File {
  pub fn from(content: &str, path: Option<PathBuf>) -> Self {
    let text_editor_content = text_editor::Content::with_text(content);
    let markdown = markdown::parse(content).collect();

    File {
      needs_saving: false,
      content: text_editor_content,
      path,
      markdown,
    }
  }

  pub fn content(&self) -> &text_editor::Content {
    &self.content
  }

  pub fn content_mut(&mut self) -> &mut text_editor::Content {
    &mut self.content
  }

  pub fn set_content(&mut self, content: &str) {
    self.content = text_editor::Content::with_text(content);
  }

  pub fn markdown(&self) -> Vec<&markdown::Item> {
    self.markdown.iter().collect()
  }

  pub fn update_markdown(&mut self) {
    self.markdown = markdown::parse(&self.content.text()).collect();
  }

  pub fn path(&self) -> Option<&PathBuf> {
    self.path.as_ref()
  }

  pub fn set_path(&mut self, path: Option<PathBuf>) {
    self.path = path;
  }

  pub fn extension(&self) -> Option<&str> {
    self
      .path
      .as_deref()
      .and_then(Path::extension)
      .and_then(ffi::OsStr::to_str)
  }

  pub fn display_name(&self) -> &str {
    self
      .path
      .as_deref()
      .and_then(|p| p.file_name())
      .and_then(|n| n.to_str())
      .unwrap_or("New file")
  }

  pub fn position_summary(&self) -> String {
    let pos = self.content.cursor().position;
    format!("Ln {}, Col {}", pos.line, pos.column)
  }

  pub fn path_summary(&self) -> String {
    self
      .path
      .as_deref()
      .map(|p| p.to_string_lossy().to_string())
      .unwrap_or_default()
  }
  
  pub fn needs_saving(&self) ->  bool {
      self.needs_saving
  }

  pub fn set_needs_saving(&mut self, state: bool) {
      self.needs_saving = state
  }
}
