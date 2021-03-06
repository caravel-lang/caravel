pub fn is_ident_start(c: char) -> bool {
  c.is_ascii_alphabetic() || c == '_'
}

pub fn is_ident_body(c: char) -> bool {
  is_ident_start(c) || c.is_ascii_digit()
}

pub fn is_float_literal_body(c: char) -> bool {
  c.is_ascii_digit() || c == '_' || c == '.'
}
