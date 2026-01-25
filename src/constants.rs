use iced::Font;

pub const CUSTOM_FONT_BYTES: &[u8] = include_bytes!("./fonts/CaskaydiaCoveNFM-Regular.ttf");
pub const CUSTOM_FONT: Font = Font::with_name("CaskaydiaCove Nerd Font Mono");
pub const DEFAULT_EDITOR_FONT_SIZE: u32 = 16;
pub const MAX_EDITOR_FONT_SIZE: u32 = 80;
pub const MIN_EDITOR_FONT_SIZE: u32 = 12;
pub const ICON_BYTES: &[u8] = include_bytes!("./images/icon.ico");
