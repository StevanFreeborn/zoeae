use iced::keyboard::{self, Key, Modifiers};

use crate::message::{FileAction, Message, ViewAction};

pub struct Keybinding {
  key: &'static str,
  modifiers: Modifiers,
  message: Message,
}

impl Keybinding {
  pub fn should_handle(&self, key_pressed: &Key, modifiers: &Modifiers) -> bool {
    let key = keyboard::Key::Character(self.key.into());
    modifiers.contains(self.modifiers) && key_pressed == &key
  }

  pub fn message(&self) -> Message {
    self.message.clone()
  }
}

pub const ALL: &[Keybinding] = &[
  Keybinding {
    key: "o",
    modifiers: Modifiers::CTRL,
    message: Message::FileActionSelected(FileAction::Open),
  },
  Keybinding {
    key: "n",
    modifiers: Modifiers::CTRL,
    message: Message::FileActionSelected(FileAction::New),
  },
  Keybinding {
    key: "s",
    modifiers: Modifiers::CTRL.union(Modifiers::SHIFT),
    message: Message::FileActionSelected(FileAction::SaveAs),
  },
  Keybinding {
    key: "s",
    modifiers: Modifiers::CTRL,
    message: Message::FileActionSelected(FileAction::Save),
  },
  Keybinding {
    key: "w",
    modifiers: Modifiers::CTRL,
    message: Message::FileActionSelected(FileAction::Close(None)),
  },
  Keybinding {
    key: "p",
    modifiers: Modifiers::CTRL,
    message: Message::ViewActionSelected(ViewAction::TogglePreview),
  },
  Keybinding {
    key: "=",
    modifiers: Modifiers::CTRL,
    message: Message::ViewActionSelected(ViewAction::Increase),
  },
  Keybinding {
    key: "-",
    modifiers: Modifiers::CTRL,
    message: Message::ViewActionSelected(ViewAction::Decrease),
  },
  Keybinding {
    key: "0",
    modifiers: Modifiers::CTRL,
    message: Message::ViewActionSelected(ViewAction::Reset),
  },
  Keybinding {
    key: "z",
    modifiers: Modifiers::ALT,
    message: Message::ViewActionSelected(ViewAction::ToggleWordWrap),
  },
];
