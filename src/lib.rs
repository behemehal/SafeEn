#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_root_url = "https://docs.rs/safe_en/1.5.5")]
//!# SafeEn
//!Local database solution with clean and strict data integrity.
//!
//!## Usage
//!
//! ```
//! use safe_en::{
//!     table::{TableRow, TypeDefs},
//!     Database,
//! };
//! let mut db = Database::new();
//! db.create_table(
//!     "users",
//!     vec![
//!         TableRow::new("id", TypeDefs::I64),
//!         TableRow::new("email", TypeDefs::String),
//!     ],
//! )
//! .unwrap();
//!
//!
//! let id = 1_i64;;
//! let email = "ahmet@mail.com";
//!
//! db.table("users").unwrap().insert(vec![id.into(), email.into()]).unwrap();
//! ```
//! You can find more examples [here](https://github.com/behemehal/SafeEn/tree/main/examples)

/// Formatter for tables and types
use core::fmt;
/// FileSystem utilities for saving and loading database
use std::{fs::File, io::Write};
/// Database types
use table::{Table, TableRow, TypeDefs, Types};
/// Database table
pub mod table;
/// Database utils
pub mod utils;

/// Integrity error
#[derive(Debug, Clone)]
pub struct LoadError;

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load db from file")
    }
}

/// Database struct
pub struct Database {
    /// Database name
    name: String,
    /// Database size
    size: usize,
    /// Database tables
    tables: Vec<table::Table>,
}

impl Database {
    /// Creates a new database
    pub fn new() -> Self {
        Database {
            tables: Vec::new(),
            name: "".to_string(),
            size: 0,
        }
    }

    /// Loads a database from a file
    /// ## Errors
    /// Returns a `LoadError` if integrity checks fail
    /// ## Parameters
    /// * `path` - The path to the file
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// let db = Database::load("db.sfn");
    /// ```
    pub fn load(path: &str) -> Result<Self, LoadError> {
        let mut db = Database::new();
        match db.load_file(path) {
            Ok(_) => Ok(db),
            Err(_) => Err(LoadError),
        }
    }

    ///Sets name of the database
    /// ## Parameters
    /// * `name` - The name of the database
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// let mut db = Database::new();
    /// db.set_name("users");
    /// ```
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    ///Returns name of the database
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// let mut db = Database::new();
    /// db.set_name("users");
    /// assert_eq!(db.get_name(), "users");
    /// ```
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[deprecated(since = "1.5.3")]
    ///Returns size of the database
    pub fn get_size(&self) -> usize {
        self.size
    }

    ///Returns number of tables in the database
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("id", TypeDefs::I64),
    ///   TableRow::new("email", TypeDefs::String),
    /// ]).unwrap();
    /// assert_eq!(db.get_table_count(), 1);
    /// ```
    pub fn get_table_count(&self) -> usize {
        self.tables.len()
    }

    /// Get query
    /// ## Parameters
    /// * `table` - The name of the table
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///   TableRow::new("id", TypeDefs::I64),
    ///  TableRow::new("email", TypeDefs::String),
    /// ]).unwrap();
    /// assert_eq!(db.table("users").unwrap().get_name(), "users");
    /// ```
    pub fn table(&mut self, table_name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|x| x.name == table_name)
    }

    /// Creates table
    /// ## Parameters
    /// * `name` - Table name
    /// * `rows` - Table rows
    /// ## Example
    /// ```
    /// use safe_en::{
    ///    table::{TableRow, TypeDefs},
    ///   Database,
    /// };
    /// let mut db = Database::new();
    /// db.create_table(
    ///    "users",
    ///    vec![
    ///      TableRow::new("id", TypeDefs::I64),
    ///      TableRow::new("email", TypeDefs::String),
    ///    ]).unwrap();
    /// ```
    pub fn create_table(&mut self, table_name: &str, rows: Vec<TableRow>) -> Result<(), ()> {
        let table = table::Table {
            name: table_name.to_owned(),
            headers: rows,
            columns: vec![],
        };
        if self.tables.iter().find(|x| x.name == table_name).is_some() {
            return Err(());
        } else {
            self.tables.push(table);
            Ok(())
        }
    }

    /// Load database from file
    /// ## Parameters
    /// * `path` - The path to the file
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// let db = Database::load("db.sfn");
    /// ```
    fn load_file(&mut self, path: &str) -> Result<(), LoadError> {
        let mut file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };
        let db_name: String = utils::read_data(&mut file, TypeDefs::String).into();
        let table_len: u64 = utils::read_data(&mut file, TypeDefs::U64).into();
        self.set_name(&db_name);
        for _ in 0..table_len {
            let table_name: String = utils::read_data(&mut file, TypeDefs::String).into();
            let table_headers_len: u64 = utils::read_data(&mut file, TypeDefs::U64).into();

            let mut table_rows: Vec<TableRow> = Vec::new();

            for _ in 0..table_headers_len {
                let table_header: String = utils::read_data(&mut file, TypeDefs::String).into();
                let base_header_type: i8 = utils::read_one(&mut file);
                let second_header_type: i8 = utils::read_one(&mut file);
                let row = TableRow::new(
                    &table_header,
                    TypeDefs::from_base_and_second_layer(
                        base_header_type as u8,
                        second_header_type as u8,
                    ),
                );
                table_rows.push(row);
            }

            //Create table from collected rows
            match self.create_table(&table_name, table_rows.clone()) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            };

            let table_rows_len: u64 = utils::read_data(&mut file, TypeDefs::U64).into();

            for _ in 0..table_rows_len {
                let mut tables = vec![];
                for table_row in &table_rows {
                    let row_value = utils::read_data(&mut file, table_row.rtype.clone());
                    tables.push(row_value);
                }
                match self.table(&table_name) {
                    Some(it) => match it.insert(tables.clone()) {
                        Ok(_) => (),
                        Err(_) => return Err(LoadError),
                    },
                    None => return Err(LoadError),
                }
            }
        }
        Ok(())
    }

    /// Saves database to file
    /// ## Parameters
    /// * `path` - The path to the file
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// let mut db = Database::new();
    /// db.save("db.sfn");
    /// ```
    pub fn save(&self, path: &str) {
        let mut bytes = vec![];

        utils::extend_bytes_from_raw_type(&mut bytes, &utils::type_to_bytes(self.name.clone()));
        utils::extend_bytes_from_raw_type(
            &mut bytes,
            &utils::type_to_bytes(self.tables.len() as u64),
        );

        for table in self.tables.iter() {
            utils::extend_bytes_from_raw_type(
                &mut bytes,
                &utils::type_to_bytes(table.name.clone()),
            );
            utils::extend_bytes_from_raw_type(
                &mut bytes,
                &utils::type_to_bytes(table.headers.len() as u64),
            );

            for header in table.headers.iter() {
                utils::extend_bytes_from_raw_type(
                    &mut bytes,
                    &utils::type_to_bytes(header.key.clone()),
                );
                bytes.extend(header.rtype.get_base_and_second_layer());
            }

            utils::extend_bytes_from_raw_type(
                &mut bytes,
                &utils::type_to_bytes(table.columns.len() as u64),
            );

            for row in table.columns.iter() {
                for _data in row.iter() {
                    let data = match _data.clone() {
                        Types::String(e) => utils::type_to_bytes(e),
                        Types::Char(e) => utils::type_to_bytes(e),
                        Types::I8(e) => utils::type_to_bytes(e),
                        Types::I64(e) => utils::type_to_bytes(e),
                        Types::U64(e) => utils::type_to_bytes(e),
                        Types::Bool(e) => utils::type_to_bytes(e),
                        Types::F32(e) => utils::type_to_bytes(e),
                        Types::F64(e) => utils::type_to_bytes(e),
                        Types::Array(e) => utils::type_to_bytes(e),
                    };
                    utils::extend_bytes_from_raw_type(&mut bytes, &data);
                }
            }
        }

        let mut file = match File::create(path) {
            Ok(it) => it,
            Err(_) => return,
        };

        match file.write_all(&bytes) {
            Ok(it) => it,
            Err(_) => return,
        };
    }
}
