use crate::token;

pub struct Lexer {
  input: Vec<char>,
  index: usize,
  read_index: usize,
  // Represents the current character. If the input
  // string was empty, or if the read index is out of
  // bounds, it will be [`None`].
  current_char: Option<char>,
}

fn is_letter(character: char) -> bool {
  'a' <= character && character <= 'z' || 'A' <= character && character <= 'Z' || character == '_'
}

fn is_digit(character: char) -> bool {
  '0' <= character && character <= '9'
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Self {
    let current_char = if input.is_empty() {
      None
    } else {
      Some(input[0])
    };
    Self {
      input,
      index: 0,
      read_index: 0,
      current_char,
    }
  }

  pub fn read_char(&mut self) {
    if self.read_index >= self.input.len() {
      self.current_char = None;
    } else {
      self.current_char = match self.input.get(self.read_index) {
        Some(value) => Some(value.clone()),
        None => None,
      };
    }

    self.index = self.read_index;
    self.read_index += 1;
  }

  fn is_whitespace(&mut self) -> bool {
    if self.current_char.is_none() {
      return false;
    }

    let current_char = self.current_char.unwrap();

    current_char == ' ' || current_char == '\t' || current_char == '\n' || current_char == '\r'
  }

  fn is_eof(&self) -> bool {
    self.input.is_empty() || self.index == self.input.len() - 1
  }
}

impl Iterator for Lexer {
  type Item = token::Token;

  fn next(&mut self) -> Option<Self::Item> {
    if self.current_char.is_none() {
      return None;
    }

    let read_identifier = |lexer: &mut Lexer| -> String {
      let index = lexer.index;

      while lexer.index < lexer.input.len() && is_letter(lexer.current_char.unwrap()) {
        lexer.read_char();
      }

      lexer.input[index..lexer.index]
        .to_vec()
        .iter()
        .cloned()
        .collect::<String>()
    };

    let read_number = |lexer: &mut Lexer| -> Vec<char> {
      let index = lexer.index;

      while lexer.index < lexer.input.len() && is_digit(lexer.current_char.unwrap()) {
        lexer.read_char();
      }

      lexer.input[index..lexer.index].to_vec()
    };

    // TODO: What if it's EOF + whitespace?
    while self.is_whitespace() && self.is_eof() {
      self.read_char()
    }

    // TODO: Is it okay to use '?' here?

    let token: token::Token = match self.current_char? {
      '{' => token::Token::SymbolBraceL,
      '}' => token::Token::SymbolBraceR,
      '(' => token::Token::SymbolParenthesesL,
      ')' => token::Token::SymbolParenthesesR,
      '~' => token::Token::SymbolTilde,
      _ => {
        if is_letter(self.current_char.unwrap()) {
          let identifier = read_identifier(self);

          match token::get_keyword_or_type_token(identifier.as_str()) {
            Ok(keyword_token) => {
              return Some(keyword_token);
            }
            Err(_) => {
              return Some(token::Token::Identifier(identifier));
            }
          }
        } else if is_digit(self.current_char.unwrap()) {
          return Some(token::Token::Integer(read_number(self)));
        } else {
          return None;
        }
      }
    };

    self.read_char();

    Some(token)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn lexer_is_letter() {
    assert_eq!(true, is_letter('a'));
    assert_eq!(true, is_letter('z'));
    assert_eq!(true, is_letter('_'));
    assert_eq!(false, is_letter('0'));
    assert_eq!(false, is_letter('1'));
    assert_eq!(false, is_letter('!'));
  }

  #[test]
  fn lexer_is_digit() {
    assert_eq!(false, is_digit('a'));
    assert_eq!(false, is_digit('z'));
    assert_eq!(false, is_digit('_'));
    assert_eq!(true, is_digit('0'));
    assert_eq!(true, is_digit('1'));
    assert_eq!(false, is_digit('!'));
  }

  #[test]
  fn lexer_proper_initial_values() {
    let lexer = Lexer::new(vec!['a']);

    assert_eq!(lexer.input.len(), 1);
    assert_eq!(lexer.input[0], 'a');
    assert_eq!(lexer.index, 0);
    assert_eq!(lexer.read_index, 0);
    assert_eq!(lexer.current_char, Some('a'));
  }

  #[test]
  fn lexer_next_identifier() {
    let mut lexer = Lexer::new(vec!['a']);

    lexer.read_char();

    assert_eq!(
      Some(token::Token::Identifier(String::from("a"))),
      lexer.next()
    );
  }

  #[test]
  fn lexer_next_eof() {
    let mut lexer = Lexer::new(vec!['a']);

    lexer.read_char();
    lexer.next();
    assert_eq!(None, lexer.next());
  }

  #[test]
  fn lexer_next_none() {
    let mut lexer = Lexer::new(vec!['?']);

    lexer.read_char();

    assert_eq!(None, lexer.next());
  }

  #[test]
  fn lexer_read_char_single() {
    let mut lexer = Lexer::new(vec!['a']);

    lexer.read_char();
    assert_eq!(lexer.index, 0);
    assert_eq!(lexer.read_index, 1);
    assert_eq!(lexer.current_char, Some('a'));
  }

  #[test]
  fn lexer_read_char_overflow() {
    let mut lexer = Lexer::new(vec!['a']);

    lexer.read_char();
    lexer.read_char();
    assert_eq!(lexer.index, 1);
    assert_eq!(lexer.read_index, 2);
    assert_eq!(lexer.current_char, None);
  }

  #[test]
  fn lexer_is_whitespace() {
    let mut lexer = Lexer::new(vec![' ']);

    lexer.read_char();
    assert_eq!(true, lexer.is_whitespace());
  }

  #[test]
  fn lexer_is_whitespace_ignore_non_whitespace() {
    let mut lexer = Lexer::new(vec!['a']);

    lexer.read_char();
    assert_eq!(false, lexer.is_whitespace());
  }
}
