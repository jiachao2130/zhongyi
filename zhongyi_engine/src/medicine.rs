#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
use std::{
    fmt::{self, write},
    collections::HashSet,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};
use toml;
use serde_derive::{Serialize, Deserialize};

use crate::database::{Database, MEDICINE_DATABASE};

#[derive(Clone, Serialize, Deserialize, Debug, Eq)]
pub struct Medicine {
    name: String,
    alias: String,
    description: String,
    attribute: String,
    taboo: String,
    keyword: HashSet<String>
}

impl Medicine {
    pub fn new(name: &str) -> Self {
        Medicine {
            name: String::from(name),
            alias: String::new(),
            description: String::new(),
            attribute: String::new(),
            taboo: String::new(),
            keyword: HashSet::new()
        }
    }
}

impl Hash for Medicine {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Medicine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl fmt::Display for Medicine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "名称：{}", self.name)
    }
}

//pub type Medicines = HashSet<Medicine>;
pub struct Medicines(HashSet<Medicine>);

impl Medicines {
    pub fn new() -> Self {
        Medicines(HashSet::new())
    }

    pub fn get_instance() -> Arc<Mutex<Self>> {
        static mut MEDICINES: Option<Arc<Mutex<Medicines>>> = None;

        unsafe {
            MEDICINES.get_or_insert_with(|| {
                Arc::new(Mutex::new(
                        Medicines(toml::from_str(&Self::get_database().lock().unwrap().data()).unwrap_or(HashSet::new()))
                        ))
            }).clone()
        }
    }

    pub fn get_database() -> Arc<Mutex<Database>> {
        static mut DATABASE: Option<Arc<Mutex<Database>>>= None;

        unsafe {
            DATABASE.get_or_insert_with(|| {
                Arc::new(Mutex::new(Database::load(MEDICINE_DATABASE).unwrap()))
            }).clone()
        }
    }

    pub fn insert(&self, medicine: Medicine) -> bool {
        let _medicines = Self::get_instance();
        let res = _medicines.lock().unwrap().0.insert(medicine);
        res
    }

    pub fn search(&self, name: &str) -> Option<Medicine> {
        let _medicines = Self::get_instance();
        let res = if let Some(res) = _medicines.lock().unwrap().0.get(&Medicine::new(name)) {
            Some(res.clone())
        } else {
            None
        };
        res
    }

    pub fn delete(&self, name: &str) -> bool {
        let mut _medicines = Self::get_instance();
        let res = _medicines.lock().unwrap().0.remove(&Medicine::new(name));
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn medicine_test() {
        let gancao = Medicine::new("甘草");
        assert_eq!(format!("{gancao}"), "名称：甘草");
    }

    #[test]
    fn medicines_test() {
        // prepare
        let mut _medicines = Medicines::new();
        let mut _medicine = Medicine::new("甘草");
        _medicine.attribute = String::from("补脾益气、润肺止咳、清热解毒和调和诸药");
        // insert
        assert_eq!(_medicines.insert(_medicine.clone()), true);
        assert_eq!(_medicines.insert(_medicine.clone()), false);
        // search
        assert_eq!(_medicines.search("甘草"), Some(_medicine.clone()));
        // delete
        assert_eq!(_medicines.delete("甘草"), true);
        assert_eq!(_medicines.delete("甘草"), false);
    }
}
