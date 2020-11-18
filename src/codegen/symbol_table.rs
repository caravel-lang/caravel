use inkwell::values::AnyValueEnum;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Symbol<'a> {
    pub identifier: String,
    pub value: AnyValueEnum<'a>,
}

#[derive(Clone)]
pub struct SymbolTable<'a> {
    parent: Option<Box<SymbolTable<'a>>>,
    symbols: HashMap<String, Symbol<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn new_child(self) -> Self {
        Self {
            parent: Some(Box::new(self)),
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Symbol<'a>> {
        match self.symbols.get(identifier) {
            Some(symbol) => Some(symbol.clone()),
            None => match self.parent.clone() {
                Some(parent) => parent.get(identifier),
                None => None,
            },
        }
    }

    pub fn add(&mut self, symbol: Symbol<'a>) {
        self.symbols.insert(symbol.identifier.clone(), symbol);
    }

    pub fn add_symbols(&mut self, symbols: HashMap<String, Symbol<'a>>) {
        self.symbols.extend(symbols);
    }
}
