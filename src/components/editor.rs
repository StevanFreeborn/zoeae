use iced::{
  Background, Element, Length, Theme,
  border::{self},
  highlighter,
  widget::{container, markdown, responsive, row, scrollable, text::Wrapping, text_editor},
};

use crate::{constants, file::File, message::Message, state::Mode};

pub fn view<'a>(
  file: &'a File,
  mode: Mode,
  font_size: u32,
  is_word_wrap_on: bool,
) -> Element<'a, Message> {
  match mode {
    Mode::Edit => {
      let wrapping = if is_word_wrap_on {
        Wrapping::WordOrGlyph
      } else {
        Wrapping::None
      };

      let create_editor = move || {
        text_editor(file.content())
          .wrapping(wrapping)
          .highlight(
            file.extension().unwrap_or("txt"),
            highlighter::Theme::Base16Ocean,
          )
          .padding(10)
          .size(font_size)
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
          .on_action(Message::Edit)
      };

      let create_scrollable = |content: Element<'a, Message>| {
        scrollable(content)
          .auto_scroll(true)
          .direction(scrollable::Direction::Both {
            vertical: scrollable::Scrollbar::default(),
            horizontal: scrollable::Scrollbar::default(),
          })
          .style(|theme: &Theme, status: scrollable::Status| {
            let mut style = scrollable::default(theme, status);
            style.horizontal_rail.background = Some(Background::Color(theme.palette().background));
            style.horizontal_rail.scroller.background = Background::Color(theme.palette().primary);
            style.vertical_rail.background = Some(Background::Color(theme.palette().background));
            style.vertical_rail.scroller.background = Background::Color(theme.palette().primary);
            style
          })
          .anchor_bottom()
          .height(Length::Fill)
      };

      if is_word_wrap_on {
        responsive(move |size| {
          let editor = create_editor();
          create_scrollable(container(editor).width(size.width).into()).into()
        })
        .into()
      } else {
        let editor = create_editor();
        create_scrollable(container(editor).height(Length::Fill).into())
          .width(Length::Fill)
          .into()
      }
    }
    Mode::Preview => {
      let mut style: markdown::Style = Theme::Dark.into();
      style.font = constants::CUSTOM_FONT;

      let settings = markdown::Settings::with_text_size(font_size, style);
      let markdown_preview = markdown::view(file.markdown(), settings).map(Message::LinkClicked);

      scrollable(row![markdown_preview].padding(10))
        .auto_scroll(true)
        .direction(scrollable::Direction::Both {
          vertical: scrollable::Scrollbar::new().spacing(1),
          horizontal: scrollable::Scrollbar::new().spacing(1),
        })
        .style(|theme: &Theme, status: scrollable::Status| {
          let mut style = scrollable::default(theme, status);
          style.horizontal_rail.background = Some(Background::Color(theme.palette().background));
          style.horizontal_rail.scroller.background = Background::Color(theme.palette().primary);
          style.vertical_rail.background = Some(Background::Color(theme.palette().background));
          style.vertical_rail.scroller.background = Background::Color(theme.palette().primary);
          style
        })
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
  }
}
