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

use crate::database::{Database, MEDICINE_DATABASE};

/// Medicine 药材
///     - 名称
///     - 别名
///     - 描述
///     - 特性
///     - 禁忌
///     - 关键词
#[derive(Clone, Serialize, Deserialize, Debug, Eq)]
pub struct Medicine {
    pub name: String,
    pub alias: String,
    pub summary: String,
    pub attribute: String,
    pub taboo: String,
    pub keyword: String
}

impl Medicine {
    pub fn new(name: &str) -> Self {
        Medicine {
            name: String::from(name),
            alias: String::new(),
            summary: String::new(),
            attribute: String::new(),
            taboo: String::new(),
            keyword: String::new()
        }
    }

    pub fn create(name: &str) -> Self {
        let mut medicine = Self::new(name);
        let stdin = io::stdin();

        print!("药材别名：");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut medicine.alias);
        print!("概述信息：");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut medicine.summary);
        print!("属性信息：");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut medicine.attribute);
        print!("禁忌要求：");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut medicine.taboo);
        print!("关键词：");
        io::stdout().flush().unwrap();
        let _ = stdin.read_line(&mut medicine.keyword);

        medicine
    }

    pub fn show(&self) {
        println!("
名称：{}
别名：{}
概述：{}
功效：{}
禁忌：{}
关键词：{}
                 ", self.name, self.alias, self.summary, self.attribute, self.taboo, self.keyword);
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Medicines(pub HashMap<String, Medicine>);

impl Medicines {
    pub fn get_instance() -> Arc<Mutex<Self>> {
        static mut MEDICINES: Option<Arc<Mutex<Medicines>>> = None;

        unsafe {
            MEDICINES.get_or_insert_with(|| {
                Arc::new(Mutex::new(
                        Medicines(toml::from_str(&Self::get_database().lock().unwrap().data()).unwrap_or(HashMap::new()))
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

    pub fn insert(medicine: Medicine) -> Option<Medicine> {
        let _medicines = Self::get_instance();
        let res = _medicines.lock().unwrap().0.insert(medicine.name.to_string(), medicine);
        res
    }

    pub fn search(name: &str) -> Option<Medicine> {
        let _medicines = Self::get_instance();
        let res = if let Some(res) = _medicines.lock().unwrap().0.get(name) {
            Some(res.clone())
        } else {
            None
        };
        res
    }

    pub fn delete(name: &str) -> Option<Medicine> {
        let mut _medicines = Self::get_instance();
        let res = _medicines.lock().unwrap().0.remove(name);
        res
    }

    pub fn flush() -> std::io::Result<()> {
        let _medicines = Self::get_instance();
        let mut _database = Self::get_database();
        _database.lock().unwrap().update(toml::to_string(&_medicines.lock().unwrap().0).unwrap());
        let res = _database.lock().unwrap().write();
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
        let mut _medicine = Medicine::new("甘草");
        _medicine.attribute = String::from("补脾益气、润肺止咳、清热解毒和调和诸药");
        // insert
        assert_eq!(Medicines::insert(_medicine.clone()), None);
        assert_eq!(Medicines::insert(_medicine.clone()), Some(_medicine.clone()));
        // search
        assert_eq!(Medicines::search("甘草"), Some(_medicine.clone()));
        // delete
        assert_eq!(Medicines::delete("甘草"), Some(_medicine));
        assert_eq!(Medicines::delete("甘草"), None);
    }
}
