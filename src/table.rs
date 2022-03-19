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
            1 => TypeDefs::String,
            2 => TypeDefs::Char,
            3 => TypeDefs::I8,
            4 => TypeDefs::I64,
            5 => TypeDefs::U64,
            6 => TypeDefs::Bool,
            7 => TypeDefs::F32,
            8 => TypeDefs::F64,
            9 => TypeDefs::Array(Box::new(TypeDefs::from_base_and_second_layer(
                second_layer,
                0,
            ))),
            _ => panic!("Invalid base type"),
        }
    }

    /// Returns the id of the type
    pub fn get_base_and_second_layer(&self) -> [u8; 2] {
        match self {
            TypeDefs::String => [1, 0],
            TypeDefs::Char => [2, 0],
            TypeDefs::I8 => [3, 0],
            TypeDefs::I64 => [4, 0],
            TypeDefs::U64 => [5, 0],
            TypeDefs::Bool => [6, 0],
            TypeDefs::F32 => [7, 0],
            TypeDefs::F64 => [8, 0],
            TypeDefs::Array(t) => [9, t.get_base_and_second_layer()[0]],
        }
    }
}

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

impl Types {
    // Get defined type
    pub fn get_defination(&self) -> TypeDefs {
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

#[derive(Clone)]
pub struct Entry {
    pub key: String,
    pub value: Types,
}

impl Entry {
    pub fn get<T>(&self) -> T
    where
        T: std::convert::From<Types>,
    {
        Into::into(self.value.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Table {
    pub(crate) name: String,
    pub(crate) headers: Vec<TableRow>,
    pub(crate) columns: Vec<Vec<Types>>,
}

pub struct Entries {
    pub entries: Vec<Entry>,
}

impl Index<usize> for Entries {
    type Output = Entry;
    fn index<'a>(&'a self, i: usize) -> &'a Entry {
        &self.entries[i]
    }
}

impl Entries {
    pub fn rows(&self, rows: Vec<String>) -> Self {
        Entries {
            entries: rows
                .iter()
                .map(|x| Entry {
                    key: x.to_string(),
                    value: Types::Char(x.chars().next().unwrap()),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableRow {
    /// Name of row
    pub(crate) key: String,
    /// Type of row
    pub(crate) rtype: TypeDefs,
    /// Nullable if true, default is false
    pub(crate) nullable: bool,
}

impl TableRow {
    /// Create a new table row
    /// ## Arguments
    /// * `name` - Name of row
    /// * `rtype` - Type of row
    /// ## Returns
    /// * [`TableRow`]
    pub fn new(key: String, rtype: TypeDefs) -> Self {
        TableRow {
            key,
            rtype,
            nullable: false,
        }
    }

    /// Set the nullable flag to true
    pub fn set_nullable(&mut self) -> &mut Self {
        self.nullable = true;
        self
    }
}

impl Table {
    ///Get Headers
    pub fn get_headers(&self) -> Vec<TableRow> {
        self.headers.clone()
    }

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
