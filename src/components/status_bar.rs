use crate::file::File;
use iced::widget::{container, row, text};
use iced::{Element, Length, Padding, Theme};

pub fn view(file: &File) -> Element<'_, crate::Message> {
  let path_text = text(file.path_summary()).size(12);
  let cursor_text = text(file.position_summary()).size(12);

  container(row![cursor_text, path_text].spacing(20))
    .style(|theme: &Theme| {
      let base = container::Style::default();

      container::Style {
        background: container::primary(theme).background,
        ..base
      }
    })
    .width(Length::Fill)
    .padding(Padding {
      left: 20.0,
      right: 20.0,
      top: 5.0,
      bottom: 5.0,
    })
    .into()
}
