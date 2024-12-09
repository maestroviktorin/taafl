use iced::{self, window, Font, Settings};
use ui::*;

mod analyzer;
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

    iced::application("Синтаксический анализатор. Стародубцев Виктор. Вариант 20.", TaaflUIState::update, TaaflUIState::view)
        .centered()
        .settings(settings)
        .window(window_settings)
        .theme(TaaflUIState::theme)
        .run()
}

// region: dummy_analyzer

// pub fn dummy_analyze(input: &str) -> Result<Success, ParserError> {
//     if input == "Correct" {
//         let mut identifiers = HashSet::new();
//         identifiers.insert(("foo".to_owned(), "bar".to_owned()));

//         let mut constants = HashSet::new();
//         constants.insert((123, "baz".to_owned()));

//         Ok(Success {
//             identifiers,
//             constants,
//         })
//     } else {
//         Err(ParserError::SemanticError(
//             "\"Correct\" expected".to_owned(),
//             0,
//         ))
//     }
// }

// pub struct Success {
//     pub identifiers: HashSet<(String, String)>,
//     pub constants: HashSet<(usize, String)>,
// }

// #[derive(Debug)]
// pub enum ParserError {
//     UnexpectedToken(lexer::Token, usize),
//     ExpectedToken(String, usize),
//     SemanticError(String, usize),
// }

// endregion: dummy_analyzer
