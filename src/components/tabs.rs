use crate::file::File;
use crate::message::{FileAction, Message};
use iced::{Background, Border, Padding, Theme, border};
use iced::{
  Element,
  widget::{button, container, row, scrollable, text},
};

pub fn view<'a>(files: &'a [File], active_index: usize) -> Element<'a, Message> {
  let tabs = files.iter().enumerate().map(|(index, file)| {
    let is_focused = index == active_index;

    let label_text = if file.needs_saving() {
      format!("● {}", file.display_name())
    } else {
      file.display_name().to_owned()
    };

    let label = text(label_text);
    let close_btn = button(text("x"))
      .padding(1)
      .on_press(Message::FileActionSelected(FileAction::Close(Some(index))));

    button(
      row![label, close_btn]
        .spacing(5)
        .align_y(iced::Alignment::Center),
    )
    .on_press(Message::SwitchTab(index))
    .padding(Padding {
      top: 5.0,
      bottom: 5.0,
      left: 10.0,
      right: 10.0,
    })
    .style(move |theme: &Theme, status: button::Status| {
      let base = button::primary(theme, status);

      let button_background = if is_focused {
        Some(Background::Color(theme.palette().primary))
      } else {
        Some(Background::Color(theme.palette().background))
      };

      let button_style = button::Style {
        background: button_background,
        border: Border {
          radius: border::radius(0).top_left(10).top_right(10),
          ..base.border
        },
        ..base
      };

      match status {
        button::Status::Hovered => button::Style {
          background: Some(Background::Color(theme.palette().primary)),
          ..button_style
        },
        _ => button_style,
      }
    })
    .into()
  });

  scrollable(container(row(tabs)).padding(Padding {
    top: 5.0,
    left: 5.0,
    right: 5.0,
    bottom: 0.0,
  }))
  .direction(scrollable::Direction::Horizontal(
    scrollable::Scrollbar::new().spacing(1),
  ))
  .style(|theme: &Theme, status: scrollable::Status| {
    let mut style = scrollable::default(theme, status);
    style.horizontal_rail.background = Some(Background::Color(theme.palette().primary));
    style
  })
  .into()
}
