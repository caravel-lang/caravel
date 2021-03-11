use crate::lexer::token::Token;
use crate::position::source_position::SourceSpan;
use std::ops::{Add, Sub};

pub const DEFAULT_TOKEN_SPAN: TokenSpan = TokenSpan { start: 0, len: 0 };

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct TokenSpan {
  pub start: usize,
  pub len: usize,
}

impl TokenSpan {
  pub fn new(start: usize, len: usize) -> Self {
    Self { start, len }
  }

  pub fn as_source_span(&self, all_tokens: &Vec<Token>) -> SourceSpan {
    let first_tok = &all_tokens[self.start];
    let last_tok = &all_tokens[self.start + self.len - 1];

    SourceSpan::new(
      first_tok.pos.start_pos.clone(),
      match self.len == 1 {
        true => first_tok.pos.source_len,
        false => {
          last_tok.pos.start_pos.index + last_tok.pos.source_len - first_tok.pos.start_pos.index
        }
      },
    )
  }
}

impl Add for TokenSpan {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    let min = self.min(rhs.clone());
    let max = self.max(rhs);
    Self::new(min.start, max.start + max.len - min.start)
  }
}

impl Sub<usize> for TokenSpan {
  type Output = Self;
  fn sub(self, rhs: usize) -> Self {
    Self::new(self.start - rhs, self.len + rhs)
  }
}
