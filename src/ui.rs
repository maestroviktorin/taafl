use crate::dummy_analyze::dummy_analyze;
use iced::{
    self,
    widget::{button, column, container, row, scrollable, text, text_input, Column},
    Length::Fill,
    Task, Theme,
};

pub static WINDOW_WIDTH: f32 = 750.0;
pub static WINDOW_HEIGHT: f32 = 550.0;
pub static COLUMN_SPACING: u16 = 10;
// pub static OUTPUT_WIDTH: f32 = ...;
pub static OUTPUT_HEIGHT: f32 = 200.0;

#[derive(Debug, Default)]
pub struct TaaflUIState {
    content: String,
    syntax_output: String,
    _syntax_success: bool,
    _semantics_output: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    TextInputClear,
    TextInputChanged(String),
    TextInputSubmit,
    Analyze,
    Semantics,
}

impl TaaflUIState {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextInputChanged(content) => {
                self.content = content;
                self.syntax_output.clear();

                Task::none()
            }
            Message::TextInputClear => {
                self.content = String::new();
                self.syntax_output = String::new();
                self._semantics_output = String::new();
                self._syntax_success = false;

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

    pub fn view(&self) -> Column<Message> {
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

        let framed_syntax_output = container(scrollable(text(self.syntax_output.clone())))
            .style(container::rounded_box)
            .width(Fill)
            .height(OUTPUT_HEIGHT);
        let framed_semantics_output = container(scrollable(text(self._semantics_output.clone())))
            .style(container::rounded_box)
            .width(Fill)
            .height(OUTPUT_HEIGHT);

        Self::base_column("Automata")
            .push(row![button_input, button_analyze, button_semantics].spacing(COLUMN_SPACING / 3))
            .push(row![].push(text_input_widget).push(button_clear))
            .push(
                column![]
                    .push(framed_syntax_output)
                    .push(framed_semantics_output)
                    .spacing(COLUMN_SPACING)
                    .align_x(iced::Alignment::Center),
            )
    }

    pub fn theme(&self) -> Theme {
        Theme::Ferra
    }

    fn base_column(title: &str) -> Column<Message> {
        column![text(title).size(20)]
            .spacing(COLUMN_SPACING)
            .padding(10)
    }
}
