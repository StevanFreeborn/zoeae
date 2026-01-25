use iced::{
  Background, Element, Length, Padding, Theme,
  widget::{container, pick_list, row},
};

use crate::message::{FileAction, Message, ViewAction};

pub fn view(
  selected_file_action: Option<FileAction>,
  selected_view_action: Option<ViewAction>,
) -> Element<'static, Message> {
  let pick_list_padding = Padding {
    left: 4.0,
    right: 4.0,
    top: 2.0,
    bottom: 2.0,
  };

  let file_menu = pick_list(
    FileAction::ALL,
    selected_file_action,
    Message::FileActionSelected,
  )
  .padding(pick_list_padding)
  .style(pick_list_style)
  .placeholder("File");

  let view_menu = pick_list(
    ViewAction::ALL,
    selected_view_action,
    Message::ViewActionSelected,
  )
  .padding(pick_list_padding)
  .style(pick_list_style)
  .placeholder("View");

  container(row![file_menu, view_menu].spacing(10))
    .style(|theme: &Theme| {
      let base = container::Style::default();

      container::Style {
        background: container::primary(theme).background,
        ..base
      }
    })
    .padding(5)
    .width(Length::Fill)
    .into()
}

fn pick_list_style(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
  let base = pick_list::default(theme, status);

  pick_list::Style {
    placeholder_color: base.text_color,
    background: Background::Color(theme.palette().primary),
    ..base
  }
}
