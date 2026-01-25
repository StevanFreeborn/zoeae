use crate::file::File;
use crate::message::{FileAction, Message};
use iced::{Background, Border, Theme, border};
use iced::{
    Element,
    widget::{button, container, row, scrollable, text},
};

pub fn view<'a>(files: &'a [File], active_index: usize) -> Element<'a, Message> {
    let tabs = files.iter().enumerate().map(|(index, file)| {
        let is_focused = index == active_index;

        let label = text(file.display_name());
        let close_btn =
            button(text("x")).on_press(Message::FileActionSelected(FileAction::Close(Some(index))));

        button(row![label, close_btn].spacing(5))
            .on_press(Message::SwitchTab(index))
            .padding(5)
            .style(move |theme: &Theme, status| {
                let base = button::primary(theme, status);
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
            .into()
    });

    scrollable(container(row(tabs).spacing(2))).into()
}
