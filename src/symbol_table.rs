use crate::types::Type;
use std::collections::HashMap;

pub struct SymbolTable {
  parent: Option<Box<Self>>,
  symbols: HashMap<String, Type>,
}

impl SymbolTable {
  pub fn new(parent: Option<Box<Self>>) -> Self {
    SymbolTable {
      parent,
      symbols: HashMap::new(),
    }
  }

  pub fn set(&mut self, identifier: &str, sym_type: Type) {
    self.symbols.insert(identifier.to_owned(), sym_type);
  }

  pub fn get(&self, identifier: &str) -> Option<Type> {
    if let Some(sym_type) = self.symbols.get(identifier) {
      return Some(sym_type.clone());
    };

    match &self.parent {
      Some(parent) => (*parent).get(identifier),
      None => None,
    }
  }

  pub fn has(&self, identifier: &str) -> bool {
    match self.get(identifier) {
      Some(_) => true,
      None => false,
    }
  }
}
