use core::panic;
use std::{fmt::Display, ops::Index};

/// Rust types to be used in the table
#[derive(Clone, Debug, PartialEq)]
pub enum TypeDefs {
    /// String type
    String,
    /// Char type
    Char,
    /// I8
    I8,
    /// I64
    I64,
    /// U64 type
    U64,
    /// Boolean type
    Bool,
    /// F32 type
    F32,
    /// F64 type
    F64,
    /// Array type
    Array(Box<TypeDefs>),
}

impl Display for TypeDefs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeDefs::String => write!(f, "String"),
            TypeDefs::Char => write!(f, "Char"),
            TypeDefs::I8 => write!(f, "I8"),
            TypeDefs::I64 => write!(f, "I64"),
            TypeDefs::U64 => write!(f, "U64"),
            TypeDefs::Bool => write!(f, "Bool"),
            TypeDefs::F32 => write!(f, "F32"),
            TypeDefs::F64 => write!(f, "F64"),
            TypeDefs::Array(t) => write!(f, "Array({})", t),
        }
    }
}

impl TypeDefs {
    /// Builds a type from base and second layer
    pub fn from_base_and_second_layer(base: u8, second_layer: u8) -> TypeDefs {
        match base {
            0 => TypeDefs::String,
            1 => TypeDefs::Char,
            2 => TypeDefs::I8,
            3 => TypeDefs::I64,
            4 => TypeDefs::U64,
            5 => TypeDefs::Bool,
            6 => TypeDefs::F32,
            7 => TypeDefs::F64,
            8 => TypeDefs::Array(Box::new(TypeDefs::from_base_and_second_layer(
                second_layer,
                0,
            ))),
            _ => panic!("Invalid base type"),
        }
    }

    /// Returns the id of the type
    pub fn get_base_and_second_layer(&self) -> [u8; 2] {
        match self {
            TypeDefs::String => [0, 0],
            TypeDefs::Char => [1, 0],
            TypeDefs::I8 => [2, 0],
            TypeDefs::I64 => [3, 0],
            TypeDefs::U64 => [4, 0],
            TypeDefs::Bool => [5, 0],
            TypeDefs::F32 => [6, 0],
            TypeDefs::F64 => [7, 0],
            TypeDefs::Array(t) => [8, t.get_base_and_second_layer()[0]],
        }
    }
}

/// Database types
#[derive(Clone, Debug)]
pub enum Types {
    /// String type
    String(String),
    /// Char type
    Char(char),
    /// I8
    I8(i8),
    /// I64
    I64(i64),
    /// U64 type
    U64(u64),
    /// Boolean type
    Bool(bool),
    /// F32 type
    F32(f32),
    /// F64 type
    F64(f64),
    /// Array type
    Array(Vec<Types>),
}

impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::String(e) => format!("\"{}\"", e).fmt(f),
            Types::Char(e) => format!("'{}'", e).fmt(f),
            Types::I8(e) => format!("{}", e).fmt(f),
            Types::I64(e) => format!("{}", e).fmt(f),
            Types::U64(e) => format!("{}", e).fmt(f),
            Types::Bool(e) => format!("{}", e).fmt(f),
            Types::F32(e) => format!("{}", e).fmt(f),
            Types::F64(e) => format!("{}", e).fmt(f),
            Types::Array(e) => format!(
                "[{}]",
                e.iter().map(|x| format!("{}", x)).collect::<String>()
            )
            .fmt(f),
        }
    }
}

impl Types {
    // Get defined type
    pub(crate) fn get_defination(&self) -> TypeDefs {
        match self {
            Types::String(_) => TypeDefs::String,
            Types::Char(_) => TypeDefs::Char,
            Types::I8(_) => TypeDefs::I8,
            Types::I64(_) => TypeDefs::I64,
            Types::U64(_) => TypeDefs::U64,
            Types::Bool(_) => TypeDefs::Bool,
            Types::F32(_) => TypeDefs::F32,
            Types::F64(_) => TypeDefs::F64,
            Types::Array(e) => TypeDefs::Array(Box::new(e[0].get_defination())),
        }
    }
}

impl Into<Types> for &str {
    fn into(self) -> Types {
        Types::String(self.to_string())
    }
}

impl Into<Types> for String {
    fn into(self) -> Types {
        Types::String(self)
    }
}

impl Into<Types> for char {
    fn into(self) -> Types {
        Types::Char(self)
    }
}

impl Into<Types> for i8 {
    fn into(self) -> Types {
        Types::I8(self)
    }
}

impl Into<Types> for i64 {
    fn into(self) -> Types {
        Types::I64(self)
    }
}

impl Into<Types> for u64 {
    fn into(self) -> Types {
        Types::U64(self)
    }
}

impl Into<Types> for bool {
    fn into(self) -> Types {
        Types::Bool(self)
    }
}

impl Into<Types> for f32 {
    fn into(self) -> Types {
        Types::F32(self)
    }
}

impl Into<Types> for f64 {
    fn into(self) -> Types {
        Types::F64(self)
    }
}

impl Into<Types> for Vec<String> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::String(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<char> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::Char(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<i8> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::I8(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<i64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::I64(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<u64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::U64(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<bool> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::Bool(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<f32> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::F32(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<f64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| Types::F64(c))
                .collect::<Vec<Types>>()
                .into_iter()
                .collect::<Vec<Types>>(),
        )
    }
}

impl Into<Types> for Vec<Types> {
    fn into(self) -> Types {
        Types::Array(self)
    }
}

impl From<Types> for String {
    fn from(c: Types) -> Self {
        match c {
            Types::String(x) => x,
            _ => panic!("Not a String type"),
        }
    }
}

impl From<Types> for char {
    fn from(c: Types) -> Self {
        match c {
            Types::Char(x) => x,
            _ => panic!("Not a char type"),
        }
    }
}

impl From<Types> for i8 {
    fn from(c: Types) -> Self {
        match c {
            Types::I8(x) => x,
            _ => panic!("Not an i8 type"),
        }
    }
}

impl From<Types> for i64 {
    fn from(c: Types) -> Self {
        match c {
            Types::I64(x) => x,
            _ => panic!("Not an i64 type"),
        }
    }
}

impl From<Types> for u64 {
    fn from(c: Types) -> Self {
        match c {
            Types::U64(x) => x,
            _ => panic!("Not an u64 type"),
        }
    }
}

impl From<Types> for bool {
    fn from(c: Types) -> Self {
        match c {
            Types::Bool(x) => x,
            _ => panic!("Not a bool type"),
        }
    }
}

impl From<Types> for f32 {
    fn from(c: Types) -> Self {
        match c {
            Types::F32(x) => x,
            _ => panic!("Not a f32 type"),
        }
    }
}

impl From<Types> for f64 {
    fn from(c: Types) -> Self {
        match c {
            Types::F64(x) => x,
            _ => panic!("Not a f64 type"),
        }
    }
}

impl From<Types> for Vec<String> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::String(y) => y,
                    _ => panic!("Not a String type"),
                })
                .collect::<Vec<String>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<char> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::Char(y) => y,
                    _ => panic!("Not a char type"),
                })
                .collect::<Vec<char>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<i8> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::I8(y) => y,
                    _ => panic!("Not a i8 type"),
                })
                .collect::<Vec<i8>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<i64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::I64(y) => y,
                    _ => panic!("Not a i64 type"),
                })
                .collect::<Vec<i64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<u64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::U64(y) => y,
                    _ => panic!("Not a u64 type"),
                })
                .collect::<Vec<u64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<bool> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::Bool(y) => y,
                    _ => panic!("Not a bool type"),
                })
                .collect::<Vec<bool>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<f32> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::F32(y) => y,
                    _ => panic!("Not a f32 type"),
                })
                .collect::<Vec<f32>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<f64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x
                .into_iter()
                .map(|f| match f {
                    Types::F64(y) => y,
                    _ => panic!("Not a f64 type"),
                })
                .collect::<Vec<f64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

/// Row of table
/// Key is header of the table
/// Value is the value of the row
#[derive(Clone)]
pub struct Entry {
    /// Key
    pub key: String,
    /// Value
    pub value: Types,
}

impl Entry {
    /// Get the value of the entry
    pub fn get<T>(&self) -> T
    where
        T: std::convert::From<Types>,
    {
        Into::into(self.value.clone())
    }
}

/// Table
#[derive(Clone, Debug)]
pub struct Table {
    pub(crate) name: String,
    pub(crate) headers: Vec<TableRow>,
    pub(crate) columns: Vec<Vec<Types>>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = String::new();
        let max_row_lengths = if self.columns.len() == 0 {
            self.headers
                .iter()
                .map(|x| format!(" {} - {}", x.key, x.rtype).len())
                .collect()
        } else {
            let mut rows: Vec<usize> = self
                .headers
                .iter()
                .map(|x| format!(" {} - {}", x.key, x.rtype).len())
                .collect();
            for row in self.columns.iter() {
                for (index, _) in self.headers.iter().enumerate() {
                    let row_len = format!("{:?}", row[index]).len() + 2;
                    if rows[index] < row_len {
                        rows[index] = row_len;
                    }
                }
            }
            rows
        };

        let mut header_line = String::from("|");

        for (index, header) in self.headers.iter().enumerate() {
            header_line += &format!(" {} - {}", header.key, header.rtype);

            for _ in 0..max_row_lengths[index] - format!(" {} - {}", header.key, header.rtype).len()
            {
                header_line += " ";
            }
            header_line += " |";
        }

        let mut spliter = String::from("| ");
        for _ in 0..header_line.len() - 4 {
            spliter += "-";
        }
        spliter += " |";
        lines += &format!("{}\n{}", header_line, spliter);

        for column in &self.columns {
            let mut row_line = String::from("|");

            for (index, _) in self.headers.iter().enumerate() {
                let mut row_len = String::new();
                let rtype = format!(" {}", column[index]);
                for i in 0..max_row_lengths[index] {
                    if i < rtype.len() {
                        row_len += &rtype.chars().nth(i).unwrap_or(' ').to_string();
                    } else {
                        row_len += " ";
                    }
                }
                row_line += &format!("{} |", row_len);
            }
            lines += &format!("\n{}", row_line);
        }
        write!(f, "{}", lines)
    }
}

/// A column in a table
///
/// ```
/// /// | name | age |
/// /// | --- | --- |
/// /// | John | 12 |
/// Entries {
///    entries: vec![
///       Entry {
///         key: "name".to_string(),
///         value: Types::String("John".to_string()),
///      },
///     Entry {
///        key: "age".to_string(),
///        value: Types::I32(12),
///    },
/// ```
pub struct Entries {
    /// Rows of the table with key and value
    pub entries: Vec<Entry>,
}

impl Display for Entries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut header_line = String::from("| ");
        let mut value_line = String::from("| ");
        for entry in &self.entries {
            let key_len = entry.key.len();
            let value_len = format!("{:?}", entry.value).len();
            let line_len = if key_len > value_len {
                key_len + 1
            } else {
                value_len + 1
            };
            header_line += &entry.key;

            if key_len < line_len {
                for _ in 0..(line_len - key_len) {
                    header_line += " ";
                }
                header_line += "| ";
            }

            value_line += &format!("{:?}", entry.value);

            if value_len < line_len {
                for _ in 0..(line_len - value_len) {
                    value_line += " ";
                }
                value_line += "| ";
            }
        }
        let spacer = "-".repeat(if header_line.len() < 5 {
            5
        } else {
            header_line.len() - 5
        });
        write!(f, "{}\n| {} |\n{}", header_line, spacer, value_line)
    }
}

impl Index<usize> for Entries {
    type Output = Entry;
    fn index<'a>(&'a self, i: usize) -> &'a Entry {
        &self.entries[i]
    }
}

impl Entries {
    /// Get the value of a column by key
    pub fn get(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|x| x.key == key).map(|x| x)
    }

    /// Get the value of a column by index
    pub fn get_at(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index).map(|x| x)
    }
}

/// A row in a table
#[derive(Debug, Clone)]
pub struct TableRow {
    /// Name of row
    pub(crate) key: String,
    /// Type of row
    pub(crate) rtype: TypeDefs,
}

impl TableRow {
    /// Create a new table row
    /// ## Arguments
    /// * `name` - Name of row
    /// * `rtype` - Type of row
    /// ## Returns
    /// * [`TableRow`]
    pub fn new(key: &str, rtype: TypeDefs) -> Self {
        TableRow {
            key: key.to_string(),
            rtype,
        }
    }
}

impl Table {
    ///Get Headers
    pub fn get_headers(&self) -> Vec<TableRow> {
        self.headers.clone()
    }

    /// Get all columns as a vector of Entries
    pub fn get_all(&self) -> Vec<Entries> {
        let mut all = Vec::new();
        for i in 0..self.columns.len() {
            let mut entries = Vec::new();
            for j in 0..self.columns[i].len() {
                entries.push(Entry {
                    key: self.headers[j].key.clone(),
                    value: self.columns[i][j].clone(),
                });
            }
            all.push(Entries { entries });
        }
        all
    }

    /// Get row by index
    /// ## Arguments
    /// * `index` - Index of row
    /// ## Returns
    /// [`Option<Entries>`]
    pub fn get_at(&self, index: usize) -> Option<Entries> {
        let column = self.columns.get(index)?;
        let mut entries = Vec::new();
        for i in 0..column.len() {
            entries.push(Entry {
                key: self.headers[i].key.clone(),
                value: column[i].clone(),
            });
        }
        Some(Entries { entries })
    }

    /// Get the value of a column by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// * `value` - Value to set
    /// ## Example
    /// ```rust
    /// use safe_en::Database;
    /// let mut db = Database::new();
    /// db.table("users").get_where(|entry| entry.key == "name" && entry.get() == "Ahmet");
    /// ```
    pub fn get_where<E: Fn(Entry) -> bool + Clone + Sized>(&self, filter: E) -> Entries {
        let mut entries = Vec::new();

        for column in self.columns.iter() {
            for (i, entry) in column.iter().enumerate() {
                if filter(Entry {
                    key: self.headers[i].key.clone(),
                    value: entry.clone(),
                }) {
                    entries.push(Entry {
                        key: self.headers[i].key.clone(),
                        value: entry.clone(),
                    });
                }
            }
        }
        Entries { entries }
    }

    /// Set the value of a column by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// * `value` - Value to set
    /// ## Example
    /// ```rust
    /// use safe_en::Database;
    /// let mut db = Database::new();
    /// db.table("users").set_where(|entry| entry.key == "name" && entry.get() == "Ahmet", "Mehmet".into());
    /// ```
    pub fn set_where<E: Fn(Entry) -> bool + Clone + Sized, T>(&mut self, filter: E, value: T)
    where
        Types: From<T>,
        T: Clone,
    {
        for column in self.columns.iter_mut() {
            for i in 0..column.len() {
                if filter(Entry {
                    key: self.headers[i].key.clone(),
                    value: column[i].clone(),
                }) {
                    if self.headers[i].rtype == Types::from(value.clone()).get_defination() {
                        column[i] = Types::from(value.clone());
                    } else {
                        panic!(
                            "Type mismatch, expected {}, got {}",
                            self.headers[i].rtype,
                            Types::from(value.clone()).get_defination()
                        );
                    }
                }
            }
        }
    }

    /// Insert data to table
    /// ## Arguments
    /// * `rows` - [`TableRow`]
    /// ## Example
    /// ```
    /// use safe_en::{table::{TableRow, TypeDefs, Types},Database};
    /// db.create_table(
    ///    "test",
    ///   vec![
    ///      TableRow::new("name", TypeDefs::String),
    ///     TableRow::new("age", TypeDefs::I64),
    /// ]).unwrap();
    ///
    /// db.table("test").unwrap()
    ///    .insert(vec![
    ///       "John".into(),
    ///      18_i64.into(),
    ///     ]).unwrap();
    /// ```
    /// ## Returns
    /// * [`Result<()>`]
    /// * [`Err<Vec<String>>`] for insert errors
    pub fn insert(&mut self, rows: Vec<Types>) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        if rows.len() != self.headers.len() {
            errors.push(format!(
                "Length mismatch, expected {}, got {} length of column",
                rows.len(),
                self.headers.len()
            ));
            return Err(errors);
        }
        let mut _rows = vec![];

        for i in 0..rows.len() {
            let header = &self.headers[i];

            if header.rtype == rows[i].get_defination() {
                _rows.push(rows[i].clone());
            } else {
                errors.push(format!(
                    "Type mismatch, expected {}, got {} on column {}",
                    header.rtype,
                    rows[i].get_defination(),
                    i
                ));
            }
        }

        self.columns.push(_rows);
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(())
        }
    }
}
