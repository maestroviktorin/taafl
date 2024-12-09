use std::collections::HashSet;
use std::iter::Peekable;

/// Данный код реализует синтаксический анализатор части оператора присваивания
/// языка, сходного с фрагментом Modula-2.
/// Формат оператора:
/// <левая часть> := <правая часть>;
///
/// <левая часть> ::= <идентификатор> | <идентификатор>[<список индексов>]
/// <список индексов> ::= <индекс> | <список индексов>,<индекс>
/// <индекс> ::= <идентификатор> | <константа>
///
/// <правая часть> ::= <идентификатор> | <константа> | <правая часть><операция><правая часть>
/// <операция> ::= + | - | / | * | > | < | = | #
///
/// Идентификатор:
///   - начинается с буквы
///   - может содержать буквы и цифры
///   - длина не более 8 символов
///
/// Константа:
///   - положительное целое число в диапазоне [1..32767]
///
/// Требуется:
/// 1. Провести синтаксический анализ.
/// 2. Собрать списки идентификаторов и констант с указанием их ролей:
///    - идентификатор-индекс
///    - идентификатор-массив
///    - идентификатор-выражение
///    - константа-индекс
///    - константа-выражение
/// 3. В случае ошибок отобразить их с указанием места ошибки (курсор) и описания.
///
/// Дополнительно:
/// - В правой части не допускается использование идентификатора массива в качестве имени,
///   совпадающего с самим массивом слева (т.е. нельзя присвоить массив самому себе)
/// - Анализ остановится при первой ошибке.
/// - Регистр не учитывается.
/// - Пробелы между конструкциями могут быть произвольными или отсутствовать.

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Identifier(String),
    Constant(i32),
    LSquare,
    RSquare,
    Comma,
    Assign,
    Operation(char),
    Semicolon,
    End,
}

#[derive(Debug)]
enum Error {
    LexicalError(usize, String),
    SyntaxError(usize, String),
    SemanticError(usize, String),
}

struct Lexer<'a> {
    input: &'a [u8],
    pos: usize,
    length: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let bytes = input.as_bytes();
        Self {
            input: bytes,
            pos: 0,
            length: bytes.len(),
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.pos < self.length {
            Some(self.input[self.pos] as char)
        } else {
            None
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.pos < self.length {
            let c = self.input[self.pos] as char;
            self.pos += 1;
            Some(c)
        } else {
            None
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn lex_number(&mut self) -> Result<(usize, Token), Error> {
        let start_pos = self.pos;
        let mut num_str = String::new();
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        if let Ok(n) = num_str.parse::<i32>() {
            if n < 1 || n > 32767 {
                return Err(Error::SemanticError(
                    start_pos,
                    format!("Константа вне диапазона [1..32767]: {}", n),
                ));
            }
            Ok((start_pos, Token::Constant(n)))
        } else {
            Err(Error::LexicalError(
                start_pos,
                format!("Невозможно преобразовать в число: {}", num_str),
            ))
        }
    }

    fn lex_identifier(&mut self, first_char: char) -> Result<(usize, Token), Error> {
        let start_pos = self.pos - 1;
        let mut ident = String::new();
        ident.push(first_char);
        while let Some(c) = self.peek_char() {
            if c.is_ascii_alphanumeric() {
                ident.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        let ident = ident.to_uppercase();
        if ident.len() > 8 {
            return Err(Error::SemanticError(
                start_pos,
                format!("Идентификатор слишком длинный: {}", ident),
            ));
        }
        Ok((start_pos, Token::Identifier(ident)))
    }

    fn next_token(&mut self) -> Result<(usize, Token), Error> {
        self.skip_spaces();
        let start_pos = self.pos;
        match self.next_char() {
            Some(c) => {
                if c.is_ascii_alphabetic() {
                    self.lex_identifier(c)
                } else if c.is_ascii_digit() {
                    self.pos -= 1; // вернуть символ для lex_number
                    let number = self.lex_number();

                    if let Some(after) = self.peek_char() {
                        if after.is_ascii_alphabetic() {
                            Err(Error::SyntaxError(
                                start_pos,
                                "Идентификатор не может начинаться с цифры".to_string(),
                            ))
                        } else {
                            number
                        }
                    } else {
                        number
                    }
                } else {
                    match c {
                        '[' => Ok((start_pos, Token::LSquare)),
                        ']' => Ok((start_pos, Token::RSquare)),
                        ',' => Ok((start_pos, Token::Comma)),
                        ':' => {
                            if let Some('=') = self.peek_char() {
                                self.pos += 1;
                                Ok((start_pos, Token::Assign))
                            } else {
                                Err(Error::SyntaxError(
                                    start_pos,
                                    "Ожидался '=' после ':'".to_string(),
                                ))
                            }
                        }
                        ';' => Ok((start_pos, Token::Semicolon)),
                        '+' | '-' | '*' | '/' | '>' | '<' | '=' | '#' => {
                            Ok((start_pos, Token::Operation(c)))
                        }
                        _ => {
                            // Прочие символы - ошибка
                            Err(Error::SyntaxError(
                                start_pos,
                                format!("Недопустимый символ: '{}'", c),
                            ))
                        }
                    }
                }
            }
            None => Ok((start_pos, Token::End)),
        }
    }

    fn tokenize(mut self) -> Result<Vec<(usize, Token)>, Error> {
        let mut tokens = Vec::new();
        loop {
            let (pos, token) = self.next_token()?;
            if token == Token::End {
                break;
            }
            tokens.push((pos, token));
        }

        println!("{:?}", tokens);
        Ok(tokens)
    }
}

struct Parser {
    tokens: Peekable<std::vec::IntoIter<(usize, Token)>>,
    current_pos: usize,
    input_str: String,
    /// Для семантического анализа:
    /// Списки идентификаторов и констант, разбитые по ролям
    ids_array: HashSet<String>,
    ids_index: HashSet<String>,
    ids_expr: HashSet<String>,
    const_index: HashSet<i32>,
    const_expr: HashSet<i32>,

    /// Имя массива в левой части
    left_array_name: Option<String>,
}

impl Parser {
    fn new(tokens: Vec<(usize, Token)>, input_str: String) -> Self {
        Parser {
            tokens: tokens.into_iter().peekable(),
            current_pos: 0,
            input_str,
            ids_array: HashSet::new(),
            ids_index: HashSet::new(),
            ids_expr: HashSet::new(),
            const_index: HashSet::new(),
            const_expr: HashSet::new(),
            left_array_name: None,
        }
    }

    fn peek(&mut self) -> Option<&(usize, Token)> {
        self.tokens.peek()
    }

    fn next_token(&mut self) -> Option<(usize, Token)> {
        let pair = self.tokens.next();
        if let Some((pos, _t)) = pair.clone() {
            self.current_pos = pos
        } else {
            self.current_pos = self.input_str.len() - 1;
        };
        pair
    }

    fn expect(
        &mut self,
        expected: &[Token],
        error_message_some: String,
        error_message_none: String,
    ) -> Result<Token, Error> {
        if let Some((_, t)) = self.next_token() {
            if expected.contains(&t) {
                Ok(t)
            } else {
                let pos = self.get_current_position();
                Err(Error::SyntaxError(pos, error_message_some))
            }
        } else {
            self.next_token();
            let pos = self.get_current_position();
            Err(Error::SyntaxError(pos, error_message_none))
        }
    }

    fn get_current_position(&self) -> usize {
        self.current_pos
    }

    fn parse(&mut self) -> Result<(), Error> {
        // <левая часть> := <правая часть>;
        self.parse_left_part()?;

        self.expect(
            &[Token::Assign],
            "Ожидалось ':='".to_string(),
            "Ожидалось ':=', но достигнут конец".to_string(),
        )?;
        self.parse_right_part()?;
        self.expect(
            &[Token::Semicolon, Token::Operation('+')],
            "Ожидалось либо ';', либо операция".to_string(),
            "Ожидалось ';', но достигнут конец".to_string(),
        )?;

        if let Some(_) = self.next_token() {
            Err(Error::SyntaxError(
                self.get_current_position(),
                "После ';' ничего не ожидается".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn parse_left_part(&mut self) -> Result<(), Error> {
        // <левая часть> ::= <идентификатор> | <идентификатор>[<список индексов>]
        let ident = self.parse_identifier()?;
        // Считаем, что это потенциально имя массива
        // Но если не будет индексов - это просто одиночный идентификатор
        if let Some((_, Token::LSquare)) = self.peek() {
            // Тогда это массив
            self.next_token();
            self.ids_array.insert(ident.clone());
            self.left_array_name = Some(ident.clone());

            // Список индексов
            self.parse_index_list()?;
            self.expect(
                &[Token::RSquare],
                "Ожидалось ']'".to_string(),
                "Ожидалось ']', но достигнут конец".to_string(),
            )?;
        } else {
            self.left_array_name = None;
            self.ids_expr.insert(ident);
        }

        Ok(())
    }

    fn parse_index_list(&mut self) -> Result<(), Error> {
        // <список индексов> ::= <индекс> | <список индексов>,<индекс>
        self.parse_index()?;
        while let Some((_, Token::Comma)) = self.peek() {
            self.next_token();
            self.parse_index()?;
        }
        Ok(())
    }

    fn parse_index(&mut self) -> Result<(), Error> {
        // <индекс> ::= <идентификатор> | <константа>
        if let Some(t) = self.peek() {
            match t {
                (_, Token::Identifier(_)) => {
                    let ident = self.parse_identifier()?;
                    self.ids_index.insert(ident);
                }
                (_, Token::Constant(_)) => {
                    let c = self.parse_constant()?;
                    self.const_index.insert(c);
                }
                _ => {
                    self.next_token();
                    let pos = self.get_current_position();
                    return Err(Error::SyntaxError(
                        pos,
                        "Ожидался идентификатор или константа в индексе".to_string(),
                    ));
                }
            }
        } else {
            let pos = self.get_current_position();
            return Err(Error::SyntaxError(
                pos,
                "Ожидался индекс, но достигнут конец".to_string(),
            ));
        }
        Ok(())
    }

    fn parse_right_part(&mut self) -> Result<(), Error> {
        // <правая часть> ::= <идентификатор> | <константа> | <правая часть><операция><правая часть>
        self.parse_term()?;

        while let Some((_, Token::Operation(_))) = self.peek() {
            self.next_token();
            self.parse_term()?;
        }

        Ok(())
    }

    fn parse_term(&mut self) -> Result<(), Error> {
        // <term> ::= <идентификатор> | <константа>
        match self.peek() {
            Some((_, Token::Identifier(_))) => {
                let ident = self.parse_identifier()?;
                let pos = self.get_current_position();
                let left_array_name = &self.left_array_name.clone();

                // Проверяем семантику: нельзя использовать идентификатор массива (т.е. такой же, как слева) в правой части
                if let Some(arr) = left_array_name {
                    if ident == *arr {
                        self.next_token();
                        return Err(Error::SemanticError(
                            pos,
                            "Нельзя использовать массив в правой части".to_string(),
                        ));
                    }
                }
                self.ids_expr.insert(ident);
            }
            Some((_, Token::Constant(_))) => {
                let c = self.parse_constant()?;
                self.const_expr.insert(c);
            }
            _ => {
                self.next_token();
                return Err(Error::SyntaxError(
                    self.get_current_position(),
                    "Ожидался идентификатор или константа в правой части".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn parse_identifier(&mut self) -> Result<String, Error> {
        if let Some((_, Token::Identifier(s))) = self.next_token() {
            Ok(s)
        } else {
            let pos = self.get_current_position();
            Err(Error::SyntaxError(
                pos,
                "Ожидался идентификатор".to_string(),
            ))
        }
    }

    fn parse_constant(&mut self) -> Result<i32, Error> {
        if let Some((_, Token::Constant(c))) = self.next_token() {
            Ok(c)
        } else {
            let pos = self.get_current_position();
            Err(Error::SyntaxError(pos, "Ожидалась константа".to_string()))
        }
    }

    fn finish(self) -> (Option<String>, Option<String>) {
        // Формируем строки вывода
        // Идентификаторы: могут быть в индексах, массивах, выражениях
        // Константы: индекс, выражение

        if !self.ids_array.is_empty()
            || !self.ids_index.is_empty()
            || !self.ids_expr.is_empty()
            || !self.const_index.is_empty()
            || !self.const_expr.is_empty()
        {
            let mut ids = String::new();
            let mut consts = String::new();

            if !self.ids_array.is_empty() {
                for id in &self.ids_array {
                    ids.push_str(&format!("{} - идентификатор-массив\n", id));
                }
            }
            if !self.ids_index.is_empty() {
                for id in &self.ids_index {
                    ids.push_str(&format!("{} - идентификатор-индекс\n", id));
                }
            }
            if !self.ids_expr.is_empty() {
                for id in &self.ids_expr {
                    ids.push_str(&format!("{} - идентификатор-выражение\n", id));
                }
            }

            if !self.const_index.is_empty() {
                for c in &self.const_index {
                    consts.push_str(&format!("{} - константа-индекс\n", c));
                }
            }
            if !self.const_expr.is_empty() {
                for c in &self.const_expr {
                    consts.push_str(&format!("{} - константа-выражение\n", c));
                }
            }

            return (Some(ids), Some(consts));
        }

        (None, None)
    }
}

/// Анализирует строку входного кода, возвращая результаты синтаксического/семантического анализа.
///
/// Возвращает:
/// - Ok((Some(ids_str), Some(consts_str))): при успешном разборе, строки со списками идентификаторов и констант.
/// - Ok((None, None)): если нет идентификаторов и констант (теоретически не должно быть в данном языке).
/// - Err(err_str): при ошибке, строка с сообщением и указанием позиции.
pub fn analyze_line(input: &str) -> Result<(Option<String>, Option<String>), String> {
    let lexer = Lexer::new(input);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => return Err(format_error(e, input)),
    };

    let mut parser = Parser::new(tokens, input.to_string());
    match parser.parse() {
        Ok(_) => {
            // Успешно
            let (ids, consts) = parser.finish();
            Ok((ids, consts))
        }
        Err(e) => Err(format_error(e, input)),
    }
}

fn format_error(err: Error, input: &str) -> String {
    match err {
        Error::LexicalError(pos, msg) => {
            format_error_with_cursor(input, pos, &format!("Лексическая ошибка: {}", msg))
        }
        Error::SyntaxError(pos, msg) => {
            format_error_with_cursor(input, pos, &format!("Синтаксическая ошибка: {}", msg))
        }
        Error::SemanticError(pos, msg) => {
            format_error_with_cursor(input, pos, &format!("Семантическая ошибка: {}", msg))
        }
    }
}

fn format_error_with_cursor(input: &str, pos: usize, msg: &str) -> String {
    let mut cursor_pos = pos;
    if cursor_pos > input.len() {
        cursor_pos = input.len();
    }
    let mut result = String::new();
    result.push_str(input);
    result.push('\n');
    for _ in 0..cursor_pos {
        result.push(' ');
    }
    result.push('^');
    result.push('\n');
    result.push_str(msg);
    result
}

// ----------------------
// Пример использования:

// fn main() {
//     let input = "ABC [ 1, I, LF, 25] := ABC1 + 135 - LF * DKL1 / ZP + KP;";
//     match analyze_line(input) {
//         Ok((ids, consts)) => {
//             if let Some(ids) = ids {
//                 println!("Список идентификаторов:\n{}", ids);
//             }
//             if let Some(consts) = consts {
//                 println!("Список констант:\n{}", consts);
//             }
//         }
//         Err(e) => println!("{}", e),
//     }
// }
