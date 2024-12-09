use crate::analyzer::analyze_line;
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
                self._semantics_output.clear();
                self._syntax_success = false;

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
                if !self.content.is_empty() {
                    match analyze_line(&self.content) {
                        Ok((ids, consts)) => {
                            if ids.is_some() && consts.is_some() {
                                self._syntax_success = true;
                                self.syntax_output =
                                    self.content.clone() + "\n" + "Строка принадлежит языку.";
                            }
                        }
                        Err(e) => {
                            self.syntax_output = e;
                        }
                    }
                } else {
                    self.syntax_output = "Введите хоть что-нибудь (o_O)".to_string();
                }

                Task::none()
            }
            Message::Semantics => {
                if let Ok((Some(ids), Some(consts))) = analyze_line(&self.content) {
                    self._semantics_output = ids + "\n" + consts.as_ref();
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Column<Message> {
        let (text_input_widget, button_clear) = (
            scrollable(
                text_input("Напишите здесь что-нибудь... 🤓", self.content.as_ref())
                    .on_input(Message::TextInputChanged)
                    .on_submit(Message::TextInputSubmit),
            ),
            button("Очистить").on_press(Message::TextInputClear),
        );

        let (button_input, button_analyze, button_semantics) = (
            button("Ввод").on_press(Message::TextInputSubmit),
            button("Анализ").on_press(Message::Analyze),
            button("Семантика").on_press_maybe(if self._syntax_success {
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

        Self::base_column("Оператор присваивания языка Modula-2")
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
