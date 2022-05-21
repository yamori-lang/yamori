use crate::token;

pub struct Lexer {
  input: Vec<char>,
  index: usize,
  read_index: usize,
  current_char: char,
}

fn is_letter(character: char) -> bool {
  'a' <= character && character <= 'z' || 'A' <= character && character <= 'Z' || character == '_'
}

fn is_digit(character: char) -> bool {
  // num
  '0' <= character && character <= '9'
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Self {
    Self {
      input: input,
      index: 0,
      read_index: 0,
      current_char: '\0',
    }
  }

  pub fn read_char(&mut self) {
    if self.read_index >= self.input.len() {
      // finish
      self.current_char = '\0';
    } else {
      self.current_char = self.input[self.read_index];
    }

    self.index = self.read_index;
    self.read_index += 1;
  }

  fn skip_whitespace(&mut self) {
    if self.current_char == ' '
      || self.current_char == '\t'
      || self.current_char == '\n'
      || self.current_char == '\r'
    {
      self.read_char()
    }
  }
}

impl Iterator for Lexer {
  type Item = token::Token;

  fn next(&mut self) -> Option<Self::Item> {
    let read_identifier = |lexer: &mut Lexer| -> Vec<char> {
      let index = lexer.index;

      while lexer.index < lexer.input.len() && is_letter(lexer.current_char) {
        lexer.read_char();
      }

      lexer.input[index..lexer.index].to_vec()
    };

    let read_number = |lexer: &mut Lexer| -> Vec<char> {
      let index = lexer.index;

      while lexer.index < lexer.input.len() && is_digit(lexer.current_char) {
        lexer.read_char();
      }

      lexer.input[index..lexer.index].to_vec()
    };

    self.skip_whitespace();

    let token: token::Token;

    match self.current_char {
      '{' => {
        token = token::Token::BraceL(self.current_char);
      }
      '\0' => {
        token = token::Token::EOF;
      }
      _ => {
        if is_letter(self.current_char) {
          let identifier: Vec<char> = read_identifier(self);

          match token::get_keyword_token(&identifier) {
            Ok(keyword_token) => {
              return Some(keyword_token);
            }
            Err(_err) => {
              return Some(token::Token::Identifier(identifier));
            }
          }
        } else if is_digit(self.current_char) {
          let identifier: Vec<char> = read_number(self);

          return Some(token::Token::Integer(identifier));
        } else {
          return None;
        }
      }
    }

    self.read_char();

    Some(token)
  }
}
