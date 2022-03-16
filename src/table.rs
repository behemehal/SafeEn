/// Rust types to be used in the table
#[derive(Clone, Debug)]
pub enum TypeDefs {
    /// String type
    String,
    /// Char type
    Char,
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

impl TypeDefs {
    /// Builds a type from base and second layer
    pub fn from_base_and_second_layer(base: u8, second_layer: u8) -> TypeDefs {
        match base {
            1 => match second_layer {
                0 => TypeDefs::String,
                _ => panic!("Invalid second layer for String"),
            },
            2 => match second_layer {
                0 => TypeDefs::Char,
                _ => panic!("Invalid second layer for Char"),
            },
            3 => TypeDefs::I64,
            4 => TypeDefs::U64,
            5 => TypeDefs::Bool,
            6 => TypeDefs::F32,
            7 => TypeDefs::F64,
            8 => TypeDefs::Array(Box::new(TypeDefs::from_base_and_second_layer(second_layer, 0))),
            _ => panic!("Invalid base type"),
        }
    }
}


#[derive(Clone, Debug)]
pub enum Types {
    /// String type
    String(String),
    /// Char type
    Char(char),
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
                    column[i] = Types::from(value.clone());
                }
            }
        }
    }

    pub fn insert(&mut self, row: Vec<Types>) {
        if row.len() != self.headers.len() {
            panic!("Row length does not match table headers");
        }
        self.columns.push(row);
    }
}
