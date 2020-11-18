use inkwell::values::AnyValueEnum;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Symbol<'a> {
    pub identifier: String,
    pub value: AnyValueEnum<'a>,
}

#[derive(Clone)]
pub struct SymbolTable<'a> {
    parent: Option<Rc<RefCell<SymbolTable<'a>>>>,
    symbols: HashMap<String, Symbol<'a>>,
}

impl<'a> SymbolTable<'a> {
    pub fn new() -> Self {
        Self {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn from_parent(parent: Rc<RefCell<Self>>) -> Self {
        Self {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    pub fn get(&self, identifier: &str) -> Option<Symbol<'a>> {
        match self.symbols.get(identifier) {
            Some(symbol) => Some(symbol.clone()),
            None => match self.parent.clone() {
                Some(parent) => parent.borrow().get(identifier),
                None => None,
            },
        }
    }

    pub fn add(&mut self, symbol: Symbol<'a>) {
        self.symbols.insert(symbol.identifier.clone(), symbol);
    }
}
