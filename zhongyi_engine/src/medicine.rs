#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
use std::{
    fmt::{self, write},
    collections::HashSet,
    hash::{Hash, Hasher},
};
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Medicine {
    name: String,
    alias: String,
    description: String,
    attribute: String,
    keyword: HashSet<String>
}

impl Medicine {
    pub fn new(name: &str) -> Self {
        Medicine {
            name: String::from(name),
            alias: String::new(),
            description: String::new(),
            attribute: String::new(),
            keyword: HashSet::new()
        }
    }
}

impl Hash for Medicine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl fmt::Display for Medicine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "名称：{}", self.name)
    }
}

pub type Medicines = HashSet<Medicine>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_test() {
        let gancao = Medicine::new("甘草");
        assert_eq!(format!("{gancao}"), "名称：甘草");
    }
}
