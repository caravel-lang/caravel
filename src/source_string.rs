use crate::position::Span;

#[derive(Clone, Debug)]
pub struct SourceString {
  pub value: String,
  pub pos: Span,
}
