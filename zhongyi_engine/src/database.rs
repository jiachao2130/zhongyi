#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
use std::{
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
    collections::HashSet,
};

use crate::medicine::Medicine;

// 药材
const MEDICINE_DATABASE: &str = "Medicines";
// 处方
const PRESCRIPT_DATABASE: &str = "Prescripts";
// 辩证
const DIALECTIC_DATABASE: &str = "Dialectics";
const DATABASE_PATH: &str = "database";

#[derive(Debug)]
pub struct Database {
    pub name: String,
    file: File,
    pub data: String,
    update: bool
}

impl Database {
    pub fn create(name: &str) -> std::io::Result<Self> {
        Database::create_database_path()?;
        let _database = Path::new(DATABASE_PATH).join(name);
        let file = OpenOptions::new().write(true).create_new(true).open(_database)?;
        Ok(Self {
            name: name.to_string(),
            file: file,
            data: String::new(),
            update: false
        })
    }

    pub fn connect(name: &str) -> std::io::Result<Self> {
        let _database = Path::new(DATABASE_PATH).join(name);
        let file = OpenOptions::new().read(true).write(true).create_new(true).open(_database)?;
        Ok(Self {
            name: name.to_string(),
            file: file,
            data: String::new(),
            update: false
        })
    }

    pub fn read(&mut self) -> std::io::Result<()> {
        self.file.read_to_string(&mut self.data)?;
        Ok(())
    }

    pub fn write(&mut self) ->std::io::Result<()> {
        if !self.update {
            return Ok(())
        }
        self.file.write(self.data.as_bytes())?;
        self.file.flush()
    }

    fn create_database_path() -> std::io::Result<()> {
        let _database_path = Path::new(DATABASE_PATH);
        if !_database_path.is_dir() {
            fs::create_dir(DATABASE_PATH)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn demo() {
        let medicine_database = Path::new(DATABASE_PATH).join(MEDICINE_DATABASE);
        assert_eq!(medicine_database, PathBuf::from("database/Medicines"));
        match fs::create_dir(DATABASE_PATH) {
            Ok(_) => {},
            Err(e) => { assert_eq!("File exists (os error 17)", e.to_string()) }
        }
    }
}
