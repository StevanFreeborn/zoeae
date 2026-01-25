mod components;
mod constants;
mod file;
mod io;
mod key_bindings;
mod state;
mod handler;
mod message;

use iced::Theme;
use iced::widget::column;
use iced::window::icon;
use iced::{Element, Subscription, Task, event};
use iced::{keyboard, window};

use crate::message::Message;
use crate::state::State;

pub fn main() -> iced::Result {
    iced::application(boot, update, view)
        .subscription(subscription)
        .font(constants::CUSTOM_FONT_BYTES)
        .theme(theme)
        .settings(iced::Settings {
            default_font: constants::CUSTOM_FONT,
            ..Default::default()
        })
        .window(window::Settings {
            icon: Some(
                icon::from_file_data(constants::ICON_BYTES, None).expect("Failed to load icon"),
            ),
            ..window::Settings::default()
        })
        .run()
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Edit(action) => handler::edit(state, action),
        Message::SwitchTab(index) => handler::switch_tab(state, index),
        Message::LinkClicked(url) => handler::link_clicked(url),
        Message::FileActionSelected(action) => handler::file_action(state, action),
        Message::ViewActionSelected(action) => handler::view_action(state, action),
    }
}

fn view(state: &State) -> Element<'_, Message> {
    let current_file = state.active_file();

    column![
        components::tabs::view(state.files(), state.current_file_index()),
        components::action_bar::view(state.selected_file_action(), state.selected_view_action()),
        components::editor::view(current_file, state.mode(), state.font_size()),
        components::status_bar::view(current_file),
    ]
    .padding(10)
    .into()
}

fn boot() -> State {
    state::State::new()
}

fn subscription(_state: &State) -> Subscription<Message> {
    event::listen_with(|e, _status, _win| -> Option<Message> {
        match e {
            iced::Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                for vb in key_bindings::ALL {
                    if vb.should_handle(&key, &modifiers) {
                        return Some(vb.message());
                    }
                }

                None
            }
            _ => None,
        }
    })
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

