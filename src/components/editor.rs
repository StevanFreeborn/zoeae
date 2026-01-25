use iced::{
    Element, Length, Theme,
    widget::{container, markdown, row, text_editor},
};

use crate::{constants, file::File, message::Message, state::Mode};

pub fn view<'a>(file: &'a File, mode: Mode, font_size: u32) -> Element<'a, Message> {
    match mode {
        Mode::Edit => {
            let editor = text_editor(file.content())
                .size(font_size)
                .height(Length::Fill)
                .on_action(Message::Edit);

            container(row![editor]).height(Length::Fill).into()
        }
        Mode::Preview => {
            let mut style: markdown::Style = Theme::Dark.into();
            style.font = constants::CUSTOM_FONT;

            let settings = markdown::Settings::with_text_size(font_size, style);

            let markdown_preview =
                markdown::view(file.markdown(), settings).map(Message::LinkClicked);

            container(row![markdown_preview])
                .height(Length::Fill)
                .into()
        }
    }
}
