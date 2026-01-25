use iced::widget::{container, row, text};
use iced::{Element, Length};
use crate::file::File;

pub fn view(file: &File) -> Element<'_, crate::Message> {
    let path_text = text(file.path_summary());
    let cursor_text = text(file.position_summary());

    container(
        row![cursor_text, path_text]
            .spacing(20)
    )
    .width(Length::Fill)
    .padding(5)
    .into()
}
