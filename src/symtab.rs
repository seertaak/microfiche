use std::{collections::HashMap, io::Empty};

use crate::metaparser::{Directive, EXEC_DIRECTIVE, NOTE_DIRECTIVE};

#[derive(Clone)]
pub enum SymbolTableBinding {
    Directive(Directive),
    Data(String),
    Module(SymbolTable),
}

#[derive(Clone, Default)]
pub struct SymbolTable {
    bindings: HashMap<String, SymbolTableBinding>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn root() -> Self {
        Self {
            bindings: HashMap::from_iter([NOTE_DIRECTIVE, EXEC_DIRECTIVE].iter().map(
                |directive| {
                    (
                        directive.name.to_string(),
                        SymbolTableBinding::Directive(directive.clone()),
                    )
                },
            )),
        }
    }

    pub fn bind(&mut self, name: &str) {
        unimplemented!()
    }

    pub fn unbind(&mut self, name: &str) {
        unimplemented!()
    }

    pub fn is_bound(&self, name: &str) -> bool {
        unimplemented!()
    }

    pub fn lookup(&self, name: &str) -> Option<SymbolTableBinding> {
        if self.bindings.contains_key(name) {
            Option::Some(self.bindings[name].clone())
        } else {
            Option::None
        }
    }
}
