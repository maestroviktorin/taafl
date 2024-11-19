use iced::{self, window, Font, Settings};
use ui::*;

mod dummy_analyze;
mod ui;

fn main() -> iced::Result {
    let settings: Settings = iced::settings::Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    };

    let window_settings = window::Settings {
        size: iced::Size::new(WINDOW_WIDTH, WINDOW_HEIGHT),
        position: window::Position::Centered,
        resizable: false,
        ..Default::default()
    };

    iced::application("TAAFL", TaaflUIState::update, TaaflUIState::view)
        .centered()
        .settings(settings)
        .window(window_settings)
        .theme(TaaflUIState::theme)
        .run()
}
