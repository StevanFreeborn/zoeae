mod file;

use std::path::PathBuf;

use iced::Element;
use iced::widget::text_editor::{Binding, KeyPress};
use iced::widget::{column, container, pick_list, row, text, text_editor};
use iced::window::icon;
use iced::{Font, Length, Theme};
use iced::{keyboard, window};
use rfd::FileDialog;

const CUSTOM_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font Mono");
const MAX_EDITOR_FONT_SIZE: u32 = 80;
const MIN_EDITOR_FONT_SIZE: u32 = 12;

#[derive(Default)]
struct State {
    file_path: Option<PathBuf>,
    content: text_editor::Content,
    editor_font_size: u32,
    selected_file_action: Option<FileAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    Save,
    SaveAs,
    Open,
}

impl FileAction {
    const ALL: &'static [FileAction] = &[FileAction::SaveAs, FileAction::Open, FileAction::Save];
}

impl std::fmt::Display for FileAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileAction::Save => write!(f, "Save"),
            FileAction::SaveAs => write!(f, "Save As... "),
            FileAction::Open => write!(f, "Open"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileActionSelected(FileAction),
    IncreaseFont,
    DecreaseFont,
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

fn view(state: &State) -> Element<'_, Message> {
    // TODO: Expose the zooming functionality
    // through a menu

    let file_menu = pick_list(
        FileAction::ALL,
        state.selected_file_action,
        Message::FileActionSelected,
    )
    .placeholder("File");

    let action_bar = container(row![file_menu]);

    let editor = text_editor(&state.content)
        .size(state.editor_font_size)
        .height(Length::Fill)
        .on_action(Message::Edit)
        .key_binding(|key_press: KeyPress| {
            let s = keyboard::Key::Character("s".into());
            let o = keyboard::Key::Character("o".into());
            let minus = keyboard::Key::Character("-".into());
            let equals = keyboard::Key::Character("=".into());


            let is_open = key_press.modifiers.command() && key_press.key == o;
            let is_save = key_press.modifiers.command() && key_press.key == s;
            let is_save_as =
                key_press.modifiers.command() && key_press.modifiers.shift() && key_press.key == s;

            let is_increase_font = key_press.modifiers.command() && key_press.modifiers.shift() && key_press.key == equals;
            let is_decrease_font = key_press.modifiers.command() && key_press.key == minus;
            
            if is_increase_font {
                return Some(Binding::Custom(Message::IncreaseFont));
            }

            if is_decrease_font {
                return Some(Binding::Custom(Message::DecreaseFont));
            }

            if is_save_as {
                return Some(Binding::Custom(Message::FileActionSelected(
                    FileAction::SaveAs,
                )));
            }

            if is_save {
                return Some(Binding::Custom(Message::FileActionSelected(
                    FileAction::Save,
                )));
            }

            if is_open {
                return Some(Binding::Custom(Message::FileActionSelected(
                    FileAction::Open,
                )));
            }

            text_editor::Binding::from_key_press(key_press)
        });

    let editor_container = container(row![editor]).height(Length::Fill);

    let cursor_position = state.content.cursor().position;
    let cursor_display_text = format!(
        "Ln {}, Col {}",
        cursor_position.line, cursor_position.column
    );
    let cursor_text = text(cursor_display_text);

    let file_path_display_text = match &state.file_path {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::new(),
    };

    let file_path_text = text(file_path_display_text);

    let status_bar = container(row![file_path_text, cursor_text].spacing(10));

    container(column![action_bar, editor_container, status_bar].spacing(10))
        .padding(10)
        .into()
}

fn save_file(path: Option<PathBuf>, text: String) -> Option<PathBuf> {
    let mut save_path = path.clone();

    if path == None {
        save_path = FileDialog::new().set_directory("/").save_file();
    }

    match save_path {
        Some(p) => {
            let sp = p.clone();
            file::save_file_to_disk(sp, text);
            Some(p)
        }
        None => None,
    }
}

fn save_file_as(text: String) -> Option<PathBuf> {
    let files = FileDialog::new().set_directory("/").save_file();

    match files {
        Some(path) => {
            file::save_file_to_disk(path.clone(), text);
            Some(path)
        }
        None => None,
    }
}

fn open_file() -> (PathBuf, String) {
    let file = FileDialog::new().set_directory("/").pick_file();

    match file {
        Some(path) => {
            let content = file::load_file_from_disk(path.clone());
            (path, content)
        }
        None => (PathBuf::new(), String::new()),
    }
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }
        Message::FileActionSelected(action) => {
            state.selected_file_action = None;

            match action {
                FileAction::SaveAs => state.file_path = save_file_as(state.content.text()),
                FileAction::Open => {
                    let (path, content) = open_file();
                    state.content = text_editor::Content::with_text(&content);
                    state.file_path = Some(path);
                }
                FileAction::Save => {
                    state.file_path = save_file(state.file_path.clone(), state.content.text());
                }
            }
        }
        Message::IncreaseFont => {
            if state.editor_font_size >= MAX_EDITOR_FONT_SIZE {
                return;
            }

            state.editor_font_size += 2;
        },
        Message::DecreaseFont => {
            if state.editor_font_size <= MIN_EDITOR_FONT_SIZE {
                return;
            }

            state.editor_font_size -= 2;
        }
    }
}

fn boot() -> State {
    State {
        editor_font_size: 16,
        ..Default::default()
    }
}

pub fn main() -> iced::Result {
    iced::application(boot, update, view)
        .font(include_bytes!("./fonts/CaskaydiaCoveNFM-Regular.ttf"))
        .theme(theme)
        .settings(iced::Settings {
            default_font: CUSTOM_FONT,
            ..Default::default()
        })
        .window(window::Settings {
            icon: Some(
                icon::from_file_data(include_bytes!("./images/icon.ico"), None)
                    .expect("Failed to load icon"),
            ),
            ..window::Settings::default()
        })
        .run()
}
