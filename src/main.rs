mod file;

use std::path::PathBuf;

use iced::padding::bottom;
use iced::widget::text_editor::{Binding, KeyPress};
use iced::widget::{
    button, column, container, markdown, pick_list, row, scrollable, text, text_editor,
};
use iced::window::icon;
use iced::{Background, Border, Element, Task, border};
use iced::{Font, Length, Theme};
use iced::{keyboard, window};
use rfd::FileDialog;

const CUSTOM_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font Mono");
const DEFAULT_EDITOR_FONT_SIZE: u32 = 16;
const MAX_EDITOR_FONT_SIZE: u32 = 80;
const MIN_EDITOR_FONT_SIZE: u32 = 12;

// TODO: Need to handle that when user
// cancels file action it is causing
// the currently selected path to be
// emptied out

// TODO: Need to reconsider the way
// we are holding on to files. Maybe
// vec is not best

// TODO: Need to handle having more tabs
// then can be displayed in the container
// given it's current width

// TODO: We need to style the button tab
// according to which is currently focused
// in the editor

// TODO: Handle creating new files

// TODO: Opening another window
struct File {
    content: text_editor::Content,
    path: Option<PathBuf>,
    markdown: Vec<markdown::Item>,
}

impl Default for File {
    fn default() -> Self {
        File {
            content: text_editor::Content::new(),
            path: None,
            markdown: Vec::new(),
        }
    }
}

#[derive(Default)]
enum Mode {
    #[default]
    Edit,
    Preview,
}

#[derive(Default)]
struct State {
    mode: Mode,
    files: Vec<File>,
    current_file: usize,
    editor_font_size: u32,
    selected_file_action: Option<FileAction>,
    selected_view_action: Option<ViewAction>,
}

// fn get_current_file(mut files: &HashMap<Uuid, File>, current_file: &Uuid) -> &mut File {
//     files
//         .get_mut(current_file)
//         .expect("current file id not present in files map")
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    New,
    Save,
    SaveAs,
    Open,
    Close(Option<usize>),
}

impl FileAction {
    const ALL: &'static [FileAction] = &[
        FileAction::New,
        FileAction::Save,
        FileAction::SaveAs,
        FileAction::Open,
        FileAction::Close(None),
    ];
}

impl std::fmt::Display for FileAction {
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
enum ViewAction {
    Increase,
    Decrease,
    Reset,
    TogglePreview,
}

impl ViewAction {
    const ALL: &'static [ViewAction] = &[
        ViewAction::Increase,
        ViewAction::Decrease,
        ViewAction::Reset,
        ViewAction::TogglePreview,
    ];
}

impl std::fmt::Display for ViewAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ViewAction::Decrease => write!(f, "Decrease font"),
            ViewAction::Increase => write!(f, "Increase font"),
            ViewAction::Reset => write!(f, "Reset font"),
            ViewAction::TogglePreview => write!(f, "Preview"),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileActionSelected(FileAction),
    ViewActionSelected(ViewAction),
    SwitchTab(usize),
    LinkClicked(String),
}

// TODO: Can we at least make
// theme configurable
// and then maybe even define
// complete custom and/or use
// existing as starter
fn theme(_state: &State) -> Theme {
    Theme::Dark
}

fn view(state: &State) -> Element<'_, Message> {
    let mut tab_row = row![];

    for (file_index, file) in state.files.iter().enumerate() {
        let file_name = if let Some(p) = &file.path {
            p.file_name()
                .expect("unable to get file name")
                .to_str()
                .expect("unable to get file name")
        } else {
            "New file"
        };

        let tab_button_text = text(file_name).wrapping(text::Wrapping::None);
        let delete_button = button(text("x")).on_press(Message::FileActionSelected(
            FileAction::Close(Some(file_index)),
        ));

        let tab_button = button(row![tab_button_text, delete_button])
            .style(move |theme: &Theme, status| {
                let base = button::primary(theme, status);
                let is_focused = state.current_file == file_index;
                let button_background = if is_focused {
                    base.background
                } else {
                    Some(Background::Color(theme.palette().background))
                };

                button::Style {
                    background: button_background,
                    border: Border {
                        radius: border::radius(0).top_left(10).top_right(10),
                        ..base.border
                    },
                    ..base
                }
            })
            .on_press(Message::SwitchTab(file_index));

        tab_row = tab_row.push(tab_button);
    }

    let tabs = scrollable(container(tab_row).padding(bottom(10))).direction(
        scrollable::Direction::Horizontal(scrollable::Scrollbar::new()),
    );

    let file_menu = pick_list(
        FileAction::ALL,
        state.selected_file_action,
        Message::FileActionSelected,
    )
    .placeholder("File");

    let view_menu = pick_list(
        ViewAction::ALL,
        state.selected_view_action,
        Message::ViewActionSelected,
    )
    .placeholder("View");

    let action_bar = container(row![file_menu, view_menu].spacing(5));

    let current_file = &state.files[state.current_file];

    let editor = text_editor(&current_file.content)
        .size(state.editor_font_size)
        .height(Length::Fill)
        .on_action(Message::Edit)
        .key_binding(|key_press: KeyPress| {
            let n = keyboard::Key::Character("n".into());
            let s = keyboard::Key::Character("s".into());
            let o = keyboard::Key::Character("o".into());
            let w = keyboard::Key::Character("w".into());
            let minus = keyboard::Key::Character("-".into());
            let equals = keyboard::Key::Character("=".into());
            let zero = keyboard::Key::Character("0".into());

            let is_new = key_press.modifiers.command() && key_press.key == n;
            let is_save = key_press.modifiers.command() && key_press.key == s;
            let is_save_as =
                key_press.modifiers.command() && key_press.modifiers.shift() && key_press.key == s;
            let is_open = key_press.modifiers.command() && key_press.key == o;
            let is_close = key_press.modifiers.command() && key_press.key == w;

            let is_increase_font = key_press.modifiers.command() && key_press.key == equals;
            let is_decrease_font = key_press.modifiers.command() && key_press.key == minus;
            let is_reset_font = key_press.modifiers.command() && key_press.key == zero;

            if is_close {
                return Some(Binding::Custom(Message::FileActionSelected(
                    FileAction::Close(Some(state.current_file)),
                )));
            }

            if is_reset_font {
                return Some(Binding::Custom(Message::ViewActionSelected(
                    ViewAction::Reset,
                )));
            }

            if is_increase_font {
                return Some(Binding::Custom(Message::ViewActionSelected(
                    ViewAction::Increase,
                )));
            }

            if is_decrease_font {
                return Some(Binding::Custom(Message::ViewActionSelected(
                    ViewAction::Decrease,
                )));
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

            if is_new {
                return Some(Binding::Custom(Message::FileActionSelected(
                    FileAction::New,
                )));
            }

            text_editor::Binding::from_key_press(key_press)
        });

    let mut markdown_styles: markdown::Style = Theme::Dark.into();
    markdown_styles.font = CUSTOM_FONT;
    
    let markdown_settings = markdown::Settings::with_text_size(state.editor_font_size, markdown_styles);
    let markdown_preview =
        markdown::view(&current_file.markdown, markdown_settings).map(Message::LinkClicked);

    let preview_container = container(row![markdown_preview]).height(Length::Fill);
    let editor_container = container(row![editor]).height(Length::Fill);

    let cursor_position = current_file.content.cursor().position;

    let cursor_display_text = format!(
        "Ln {}, Col {}",
        cursor_position.line, cursor_position.column
    );
    let cursor_text = text(cursor_display_text);

    let file_path_display_text = match &current_file.path {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::new(),
    };

    let file_path_text = text(file_path_display_text);

    let status_bar = container(row![file_path_text, cursor_text].spacing(10));

    let main = match state.mode {
        Mode::Edit => editor_container,
        Mode::Preview => preview_container,
    };

    container(column![tabs, action_bar, main, status_bar])
        .padding(10)
        .into()
}

fn save_file(path: Option<PathBuf>, text: String) -> Option<PathBuf> {
    let mut save_path = path.clone();

    if path.is_none() {
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

fn open_file() -> (Option<PathBuf>, String) {
    let file = FileDialog::new().set_directory("/").pick_file();

    match file {
        Some(path) => {
            let content = file::load_file_from_disk(path.clone());
            (Some(path), content)
        }
        None => (None, String::new()),
    }
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Edit(action) => {
            // TODO: Probably shouldn't parse every time we edited
            let current_file = &mut state.files[state.current_file];
            current_file.content.perform(action);
            current_file.markdown = markdown::parse(&current_file.content.text()).collect();
        }
        Message::FileActionSelected(action) => {
            state.selected_file_action = None;

            match action {
                FileAction::SaveAs => {
                    let current_file = &mut state.files[state.current_file];
                    let path = save_file_as(current_file.content.text());
                    current_file.path = path;
                }
                FileAction::Open => {
                    let (path, content) = open_file();

                    if let Some(opened_path) = &path {
                        for (file_index, file) in state.files.iter_mut().enumerate() {
                            if let Some(existing_path) = &file.path
                                && opened_path == existing_path
                            {
                                file.content = text_editor::Content::with_text(&content);
                                state.current_file = file_index;
                                return Task::none();
                            }
                        }

                        let opened_file = File {
                            path,
                            content: text_editor::Content::with_text(&content),
                            markdown: markdown::parse(&content).collect(),
                        };

                        state.files.push(opened_file);
                        state.current_file = state.files.len() - 1;
                    }
                }
                FileAction::Save => {
                    let current_file = &mut state.files[state.current_file];
                    let path = save_file(current_file.path.clone(), current_file.content.text());
                    current_file.path = path;
                }
                FileAction::New => {
                    let default_file = File::default();

                    state.files.push(default_file);
                    state.current_file = state.files.len() - 1;
                }
                FileAction::Close(idx) => {
                    let idx_to_close = match idx {
                        Some(i) => i,
                        None => state.current_file,
                    };

                    if state.files.len() == 1 {
                        return iced::exit();
                    }

                    if state.files.len() - 1 == idx_to_close {
                        state.current_file = state.files.len() - 2;
                        state.files.remove(idx_to_close);
                        return Task::none();
                    }

                    state.current_file -= 1;
                    state.files.remove(idx_to_close);
                }
            }
        }
        Message::ViewActionSelected(action) => {
            state.selected_view_action = None;

            match action {
                ViewAction::Increase => {
                    if state.editor_font_size >= MAX_EDITOR_FONT_SIZE {
                        return Task::none();
                    }

                    state.editor_font_size += 2;
                }
                ViewAction::Decrease => {
                    if state.editor_font_size <= MIN_EDITOR_FONT_SIZE {
                        return Task::none();
                    }

                    state.editor_font_size -= 2;
                }
                ViewAction::Reset => {
                    state.editor_font_size = DEFAULT_EDITOR_FONT_SIZE;
                }
                // TODO: Carry previous mode when toggling
                // so that if we already in preview we can
                // just put the user back where they were
                ViewAction::TogglePreview => match state.mode {
                    Mode::Edit => state.mode = Mode::Preview,
                    Mode::Preview => state.mode = Mode::Edit,
                },
            }
        }
        Message::SwitchTab(file_id) => {
            state.current_file = file_id;
        }
        Message::LinkClicked(link) => {
            print!("Link clicked: {}", link);
        }
    }

    Task::none()
}

fn boot() -> State {
    let default_file = File::default();

    let files = vec![default_file];

    State {
        files,
        current_file: 0,
        editor_font_size: DEFAULT_EDITOR_FONT_SIZE,
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
