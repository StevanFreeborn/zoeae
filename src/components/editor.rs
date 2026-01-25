use iced::{
  Element, Length, Theme, border, highlighter,
  widget::{container, markdown, row, text_editor},
};

use crate::{constants, file::File, message::Message, state::Mode};

pub fn view<'a>(file: &'a File, mode: Mode, font_size: u32) -> Element<'a, Message> {
  match mode {
    Mode::Edit => {
      let editor = text_editor(file.content())
        .highlight(
          file.extension().unwrap_or("txt"),
          highlighter::Theme::Base16Ocean,
        )
        .padding(10)
        .size(font_size)
        .height(Length::Fill)
        .style(|theme: &Theme, status: text_editor::Status| {
          let base = text_editor::default(theme, status);

          text_editor::Style {
            border: border::Border {
              width: 0.0,
              ..Default::default()
            },
            ..base
          }
        })
        .on_action(Message::Edit);

      container(row![editor]).height(Length::Fill).into()
    }
    Mode::Preview => {
      let mut style: markdown::Style = Theme::Dark.into();
      style.font = constants::CUSTOM_FONT;

      let settings = markdown::Settings::with_text_size(font_size, style);

      let markdown_preview = markdown::view(file.markdown(), settings).map(Message::LinkClicked);

      container(row![markdown_preview])
        .height(Length::Fill)
        .into()
    }
  }
}
