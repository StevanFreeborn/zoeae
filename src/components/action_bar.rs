use iced::{Element, Length, widget::{container, pick_list, row}};

use crate::message::{FileAction, Message, ViewAction};

pub fn view(
    selected_file_action: Option<FileAction>,
    selected_view_action: Option<ViewAction>,
) -> Element<'static, Message> {
    
    let file_menu = pick_list(
        FileAction::ALL,
        selected_file_action,
        Message::FileActionSelected,
    )
    .placeholder("File");

    let view_menu = pick_list(
        ViewAction::ALL,
        selected_view_action,
        Message::ViewActionSelected,
    )
    .placeholder("View");

    container(
        row![file_menu, view_menu].spacing(10)
    )
    .width(Length::Fill)
    .into()
}
