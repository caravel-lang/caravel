use std::convert::TryFrom;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Type {
  Float,
  Void,
}

impl TryFrom<String> for Type {
  type Error = &'static str;

  fn try_from(str: String) -> Result<Self, Self::Error> {
    match &str[..] {
      "float" => Ok(Type::Float),
      "void" => Ok(Type::Void),
      _ => Err("Unexpected type"),
    }
  }
}
