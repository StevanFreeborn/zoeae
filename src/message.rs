use std::{fmt::Display, path::PathBuf};

use iced::{widget::text_editor, window};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileAction {
  New,
  Save,
  SaveAs,
  Open,
  Close(Option<usize>),
}

impl FileAction {
  pub const ALL: &'static [FileAction] = &[
    FileAction::New,
    FileAction::Save,
    FileAction::SaveAs,
    FileAction::Open,
    FileAction::Close(None),
  ];
}

impl Display for FileAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FileAction::New => write!(f, "New file"),
      FileAction::Save => write!(f, "Save"),
      FileAction::SaveAs => write!(f, "Save as... "),
      FileAction::Open => write!(f, "Open"),
      FileAction::Close(_) => write!(f, "Close"),
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewAction {
  Increase,
  Decrease,
  Reset,
  TogglePreview,
}

impl ViewAction {
  pub const ALL: &'static [ViewAction] = &[
    ViewAction::Increase,
    ViewAction::Decrease,
    ViewAction::Reset,
    ViewAction::TogglePreview,
  ];
}

impl Display for ViewAction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ViewAction::Decrease => write!(f, "Decrease font"),
      ViewAction::Increase => write!(f, "Increase font"),
      ViewAction::Reset => write!(f, "Reset font"),
      ViewAction::TogglePreview => write!(f, "Toggle preview"),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Message {
  WindowOpened(window::Id),
  WindowClosed(window::Id),
  Edit(text_editor::Action),
  FileActionSelected(FileAction),
  ViewActionSelected(ViewAction),
  SwitchTab(usize),
  LinkClicked(String),
  FileOpened(Result<(PathBuf, String), String>),
  FileSaved(Result<PathBuf, String>),
}
