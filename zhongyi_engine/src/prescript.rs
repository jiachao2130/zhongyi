#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
use std::{
    fmt::{self, write},
    collections::HashMap,
    hash::{Hash, Hasher},
    io::{self, Write},
    sync::{Arc, Mutex},
};
use toml;
use serde_derive::{Serialize, Deserialize};

use crate::medicine::Medicine;
use crate::unit::Quantum;
use crate::database::Database;

/// 药方：
///     - 名称
///     - 方剂组成
///     - 制作，使用方法
///
#[derive(Clone, Serialize, Deserialize, Debug, Eq)]
pub struct Prescript {
    pub name: String,
    pub prescript: HashMap<Medicine, Quantum>,
    pub usage: String
}

impl Prescript {
    pub fn new(name: &str) -> Self {
        Prescript {
            name: name.to_string(),
            prescript: HashMap::new(),
            usage: String::new()
        }
    }

    pub fn create(name: &str) -> Self {
        let mut prescript = Prescript::new(name);
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        let read_from_stdin = |field: &mut String| {
            let _ = stdin.read_line(field);
            field.pop();
        };

        prescript
    }
}

impl Hash for Prescript {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Prescript {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Prescript {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "名称：{}", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prescripts(HashMap<String, Prescript>);

impl Prescripts {
    fn new() -> Self {
        Prescripts(HashMap::new())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prescript_test() {
        assert!(true);
    }
}
