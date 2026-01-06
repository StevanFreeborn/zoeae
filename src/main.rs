use iced::Element;
use iced::widget::{column, pick_list, text_editor};
use rfd::FileDialog;

#[derive(Default)]
struct State {
    content: text_editor::Content,
    selected_file_action: Option<FileAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileAction {
    SaveAs,
}

impl FileAction {
    const ALL: &'static [FileAction] = &[FileAction::SaveAs];
}

impl std::fmt::Display for FileAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileAction::SaveAs => write!(f, "Save As... "),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileActionSelected(FileAction),
}

fn view(state: &State) -> Element<'_, Message> {
    let file_menu = pick_list(
        FileAction::ALL,
        state.selected_file_action,
        Message::FileActionSelected,
    )
    .placeholder("File");

    let editor = text_editor(&state.content)
        .placeholder("Type something here...")
        .on_action(Message::Edit);

    column![file_menu, editor].into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Edit(action) => {
            state.content.perform(action);
        }

        Message::FileActionSelected(action) => {
            state.selected_file_action = None;

            match action {
                FileAction::SaveAs => {
                    let files = FileDialog::new().set_directory("/").save_file();
                    println!("Selected file: {:?}", files);
                }
            }
        }
    }
}

pub fn main() -> iced::Result {
    iced::run(update, view)
}
