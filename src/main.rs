use dummy_analyze::dummy_analyze;
use iced::{
    self,
    widget::{button, column, container, row, text, text_input, Column},
    window, Font, Settings, Task,
};

mod dummy_analyze;

static COLUMN_SPACING: u16 = 10;

fn main() -> iced::Result {
    let settings: Settings = iced::settings::Settings {
        default_font: Font::MONOSPACE,
        ..Default::default()
    };

    let window_settings = window::Settings {
        size: iced::Size::new(600.0, 600.0),
        position: window::Position::Centered,
        resizable: false,
        ..Default::default()
    };

    iced::application("TAAFL", TaaflUIState::update, TaaflUIState::view)
        .centered()
        .settings(settings)
        .window(window_settings)
        .run()
}

#[derive(Debug, Default)]
struct TaaflUIState {
    content: String,
    syntax_output: String,
    _syntax_success: bool,
    _semantics_output: String,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputClear,
    TextInputChanged(String),
    TextInputSubmit,
    Analyze,
    Semantics,
}

impl TaaflUIState {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextInputChanged(content) => {
                self.content = content;
                self.syntax_output.clear();

                Task::none()
            }
            Message::TextInputClear => {
                self.content = String::new();

                Task::none()
            }
            Message::TextInputSubmit => {
                self.syntax_output = self.content.clone();

                Task::none()
            }
            Message::Analyze => {
                let result = dummy_analyze(&self.content);
                self._syntax_success = result.is_ok();

                match result {
                    Result::Ok(_success) => {
                        self.syntax_output =
                            self.content.clone() + "\nThis string belongs to the language.";
                        self._syntax_success = true;
                    }
                    Err(e) => {
                        self.syntax_output = format!("{:?}", e);
                    }
                }

                Task::none()
            }
            Message::Semantics => {
                let result = dummy_analyze(&self.content);

                if let Ok(success) = result {
                    self._semantics_output =
                        format!("{:?}\n{:?}", success.constants, success.identifiers)
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Column<Message> {
        let (text_input_widget, button_clear) = (
            text_input("Type something here...", self.content.as_ref())
                .on_input(Message::TextInputChanged)
                .on_submit(Message::TextInputSubmit),
            button("Clear").on_press(Message::TextInputClear),
        );

        let (button_input, button_analyze, button_semantics) = (
            button("Input").on_press(Message::TextInputSubmit),
            button("Analyze").on_press(Message::Analyze),
            button("Semantics").on_press_maybe(if self._syntax_success {
                Some(Message::Semantics)
            } else {
                None
            }),
        );

        let framed_syntax_output = container(text(self.syntax_output.clone()))
            .style(container::rounded_box)
            .center(250);

        let framed_semantics_output = container(text(self._semantics_output.clone()))
            .center(250)
            .style(container::rounded_box);

        Self::base_column("Automata")
            .push(row![button_input, button_analyze, button_semantics])
            .push(row![].push(text_input_widget).push(button_clear))
            .push(framed_syntax_output)
            .push(framed_semantics_output)
    }

    fn base_column(title: &str) -> Column<Message> {
        column![text(title).size(20)].spacing(COLUMN_SPACING)
    }
}
