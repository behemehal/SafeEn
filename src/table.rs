use core::{fmt::Display, ops::Index};

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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
    /// Clean way of creating array type
    /// # Example
    /// ```
    /// use safe_en::table::TypeDefs;
    /// let array_type = TypeDefs::Array(Box::new(TypeDefs::I64));
    /// let second_array_type = TypeDefs::array_of(TypeDefs::I64);
    /// assert_eq!(array_type, second_array_type);
    /// ```
    pub fn array_of(t: TypeDefs) -> TypeDefs {
        TypeDefs::Array(Box::new(t))
    }

    /// Get inner type of array
    /// # Example
    /// ```
    /// use safe_en::table::TypeDefs;
    /// let array_type = TypeDefs::array_of(TypeDefs::I64);
    /// assert_eq!(array_type.inner_type(), Some(TypeDefs::I64));
    /// ```
    pub fn inner_type(&self) -> Option<TypeDefs> {
        match self {
            TypeDefs::Array(t) => Some(*t.clone()),
            _ => None,
        }
    }

    /// Builds a type from base and second layer
    pub(crate) fn from_base_and_second_layer(base: u8, second_layer: u8) -> TypeDefs {
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
    pub(crate) fn get_base_and_second_layer(&self) -> [u8; 2] {
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
#[derive(Clone, Debug, PartialEq)]
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
    Array(Vec<SafeType>),
}

impl Display for Types {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Types::String(e) => format!("\"{}\"", e).fmt(f),
            Types::Char(e) => format!("'{}'", e).fmt(f),
            Types::I8(e) => format!("{}_i8", e).fmt(f),
            Types::I64(e) => format!("{}_i64", e).fmt(f),
            Types::U64(e) => format!("{}_u64", e).fmt(f),
            Types::Bool(e) => format!("{}_bool", e).fmt(f),
            Types::F32(e) => format!("{}_f32", e).fmt(f),
            Types::F64(e) => format!("{}_f64", e).fmt(f),
            Types::Array(e) => format!(
                "[{}]",
                e.iter()
                    .map(|x| format!("{}", x.get_type()))
                    .collect::<Vec<String>>()
                    .join(",")
            )
            .fmt(f),
        }
    }
}

///REP
#[derive(Clone, Debug, PartialEq)]
pub struct SafeType {
    ///REP
    pub type_id: TypeDefs,
    ///REP
    pub rtype: Types,
}

impl SafeType {
    ///Get the type
    /// ## Returns
    /// [`T `]
    /// ## Example
    /// ```
    /// use safe_en::table::{SafeType, TypeDefs, Types};
    /// let mut safe_type = SafeType::build("Hello".into(), TypeDefs::String);
    /// assert_eq!(safe_type.get::<String>(), "Hello".to_string());
    /// ```
    pub fn get<T>(&self) -> T
    where
        T: core::convert::From<Types>,
    {
        Into::into(self.rtype.clone())
    }

    ///Get type as [`Types`]
    /// ## Example
    /// ```
    /// use safe_en::table::{SafeType, TypeDefs, Types};
    /// let mut safe_type = SafeType::build("Hello".into(), TypeDefs::String);
    /// assert_eq!(safe_type.get_type(), Types::String("Hello".to_string()));
    /// ```
    pub fn get_type(&self) -> Types {
        self.rtype.clone()
    }

    ///Get type definition
    /// ## Example
    /// ```
    /// use safe_en::table::{SafeType, TypeDefs, Types};
    /// let mut safe_type = SafeType::build("Hello".into(), TypeDefs::String);
    /// assert_eq!(safe_type.get_type_def(), TypeDefs::String);
    /// ```
    pub fn get_type_def(&self) -> TypeDefs {
        self.type_id.clone()
    }

    ///Build a new safe type
    /// ## Returns
    /// [`SafeType`] builded
    /// ## Example
    /// ```
    /// use safe_en::table::{SafeType, TypeDefs, Types};
    /// let mut safe_type = SafeType::build("Hello".into(), TypeDefs::String);
    /// assert_eq!(safe_type.get::<String>(), "Hello".to_string());
    /// ```
    pub fn build(rtype: Types, type_id: TypeDefs) -> SafeType {
        SafeType { type_id, rtype }
    }
}

impl Types {
    /// Returns true if type is string
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let s = Types::String("Hello".to_string());
    /// assert_eq!(s.is_string(), true);
    /// ```
    pub fn is_string(&self) -> bool {
        match self {
            Types::String(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is char
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::Char('a');
    /// assert_eq!(t.is_char(), true);
    /// ```
    pub fn is_char(&self) -> bool {
        match self {
            Types::Char(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is i8
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::I8(1);
    /// assert_eq!(t.is_i8(), true);
    /// ```
    pub fn is_i8(&self) -> bool {
        match self {
            Types::I8(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is i64
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::I64(1);
    /// assert_eq!(t.is_i64(), true);
    /// ```
    pub fn is_i64(&self) -> bool {
        match self {
            Types::I64(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is u64
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::U64(1);
    /// assert_eq!(t.is_u64(), true);
    /// ```
    pub fn is_u64(&self) -> bool {
        match self {
            Types::U64(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is bool
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::Bool(true);
    /// assert_eq!(t.is_bool(), true);
    /// ```
    pub fn is_bool(&self) -> bool {
        match self {
            Types::Bool(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is f32
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::F32(1.0);
    /// assert_eq!(t.is_f32(), true);
    /// ```
    pub fn is_f32(&self) -> bool {
        match self {
            Types::F32(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is f64
    /// # Example
    /// ```
    /// use safe_en::table::Types;
    /// let t = Types::F64(1.0);
    /// assert_eq!(t.is_f64(), true);
    /// ```
    pub fn is_f64(&self) -> bool {
        match self {
            Types::F64(_) => true,
            _ => false,
        }
    }

    /// Returns true if type is array
    /// # Example
    /// ```
    /// use safe_en::table::{Types};
    /// let t = Types::Array( vec![] );
    /// assert_eq!(t.is_array(), true);
    /// ```
    pub fn is_array(&self) -> bool {
        match self {
            Types::Array(_) => true,
            _ => false,
        }
    }

    /// Convert to string
    pub fn to_string(&self) -> String {
        match self {
            Types::String(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to char
    pub fn to_char(&self) -> char {
        match self {
            Types::Char(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to i8
    pub fn to_i8(&self) -> i8 {
        match self {
            Types::I8(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to i64
    pub fn to_i64(&self) -> i64 {
        match self {
            Types::I64(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to u64
    pub fn to_u64(&self) -> u64 {
        match self {
            Types::U64(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to bool
    pub fn to_bool(&self) -> bool {
        match self {
            Types::Bool(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to f32
    pub fn to_f32(&self) -> f32 {
        match self {
            Types::F32(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }

    /// Convert to f64
    pub fn to_f64(&self) -> f64 {
        match self {
            Types::F64(e) => e.clone(),
            _ => panic!("Invalid type"),
        }
    }
}

impl Into<SafeType> for &str {
    fn into(self) -> SafeType {
        SafeType::build(Types::String(self.to_string()), TypeDefs::String)
    }
}

impl Into<SafeType> for String {
    fn into(self) -> SafeType {
        SafeType::build(Types::String(self), TypeDefs::String)
    }
}

impl Into<SafeType> for char {
    fn into(self) -> SafeType {
        SafeType::build(Types::Char(self), TypeDefs::Char)
    }
}

impl Into<SafeType> for i8 {
    fn into(self) -> SafeType {
        SafeType::build(Types::I8(self), TypeDefs::I8)
    }
}

impl Into<SafeType> for i64 {
    fn into(self) -> SafeType {
        SafeType::build(Types::I64(self), TypeDefs::I64)
    }
}

impl Into<SafeType> for u64 {
    fn into(self) -> SafeType {
        SafeType::build(Types::U64(self), TypeDefs::U64)
    }
}

impl Into<SafeType> for bool {
    fn into(self) -> SafeType {
        SafeType::build(Types::Bool(self), TypeDefs::Bool)
    }
}

impl Into<SafeType> for f32 {
    fn into(self) -> SafeType {
        SafeType::build(Types::F32(self), TypeDefs::F32)
    }
}

impl Into<SafeType> for f64 {
    fn into(self) -> SafeType {
        SafeType::build(Types::F64(self), TypeDefs::F64)
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
                .map(|c| SafeType::build(Types::String(c), TypeDefs::String))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<char> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::Char(c), TypeDefs::Char))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<i8> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::I8(c), TypeDefs::I8))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<i64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::I64(c), TypeDefs::I64))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<u64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::U64(c), TypeDefs::U64))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<bool> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::Bool(c), TypeDefs::Bool))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<f32> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::F32(c), TypeDefs::F32))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<Types> for Vec<f64> {
    fn into(self) -> Types {
        Types::Array(
            self.into_iter()
                .map(|c| SafeType::build(Types::F64(c), TypeDefs::F64))
                .collect::<Vec<SafeType>>(),
        )
    }
}

impl Into<SafeType> for Vec<&str> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::String(c.to_string()), TypeDefs::String))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::String)),
        )
    }
}

impl Into<SafeType> for Vec<String> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::String(c), TypeDefs::String))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::String)),
        )
    }
}

impl Into<SafeType> for Vec<char> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::Char(c), TypeDefs::Char))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::Char)),
        )
    }
}

impl Into<SafeType> for Vec<i8> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::I8(c), TypeDefs::I8))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::I8)),
        )
    }
}

impl Into<SafeType> for Vec<i64> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::I64(c), TypeDefs::I64))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::I64)),
        )
    }
}

impl Into<SafeType> for Vec<u64> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::U64(c), TypeDefs::U64))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::U64)),
        )
    }
}

impl Into<SafeType> for Vec<bool> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::Bool(c), TypeDefs::Bool))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::Bool)),
        )
    }
}

impl Into<SafeType> for Vec<f32> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::F32(c), TypeDefs::F32))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::F32)),
        )
    }
}

impl Into<SafeType> for Vec<f64> {
    fn into(self) -> SafeType {
        SafeType::build(
            Types::Array(
                self.into_iter()
                    .map(|c| SafeType::build(Types::F64(c), TypeDefs::F64))
                    .collect::<Vec<SafeType>>(),
            ),
            TypeDefs::Array(Box::new(TypeDefs::F64)),
        )
    }
}

impl Into<Types> for Vec<SafeType> {
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
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<String>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<char> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<char>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<i8> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<i8>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<i64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<i64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<u64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<u64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<bool> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<bool>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<f32> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<f32>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

impl From<Types> for Vec<f64> {
    fn from(c: Types) -> Self {
        match c {
            Types::Array(x) => x.into_iter().map(|f| f.get()).collect::<Vec<f64>>(),
            _ => panic!("Not a vec type"),
        }
    }
}

/// Row of table
/// Key is header of the table
/// Value is the value of the row
#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    /// Key
    pub key: String,
    /// Value
    pub value: SafeType,
}

impl Entry {
    /// Get the value of the entry
    pub fn get<T>(&self) -> T
    where
        T: From<Types>,
    {
        self.value.get()
    }
}

/// Table
#[derive(Clone, Debug)]
pub struct Table {
    pub(crate) name: String,
    pub(crate) headers: Vec<TableRow>,
    pub(crate) columns: Vec<Vec<SafeType>>,
}

impl Display for Table {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut lines = String::new();
        let max_row_lengths = if self.columns.len() == 0 {
            self.headers
                .iter()
                .map(|x| format!(" {} ", x.key).len())
                .collect()
        } else {
            let mut rows: Vec<usize> = self
                .headers
                .iter()
                .map(|x| format!(" {} ", x.key).len())
                .collect();
            for row in self.columns.iter() {
                for (index, _) in self.headers.iter().enumerate() {
                    let row_len = format!("{}", row[index].get_type()).len() + 2;
                    if rows[index] < row_len {
                        rows[index] = row_len;
                    }
                }
            }
            rows
        };

        let mut header_line = String::from("|");

        for (index, header) in self.headers.iter().enumerate() {
            header_line += &format!(" {} ", header.key);

            for _ in 0..max_row_lengths[index] - format!(" {} ", header.key).len() {
                header_line += " ";
            }
            header_line += " |";
        }

        let mut spliter = String::new();
        for i in header_line.split("") {
            spliter += if i == "|" {
                "|"
            } else if i == " " {
                " "
            } else if i != "" {
                "-"
            } else {
                ""
            };
        }
        lines += &format!("{}\n{}", header_line, spliter);

        for column in &self.columns {
            let mut row_line = String::from("|");

            for (index, _) in self.headers.iter().enumerate() {
                let mut row_len = String::new();
                let rtype = format!(" {}", column[index].get_type());
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
/// use safe_en::table::{Types, Entry, Entries, SafeType, TypeDefs};
/// Entries {
///    entries: vec![
///        Entry {
///          key: "name".to_string(),
///          value: SafeType { type_id: TypeDefs::String, rtype: Types::String("John".to_string()) },
///        },
///        Entry {
///          key: "age".to_string(),
///          value: SafeType { type_id: TypeDefs::I64, rtype: Types::I64(12) },
///        },
///     ],
/// };
/// ```
#[derive(Clone)]
pub struct Entries {
    /// Rows of the table with key and value
    pub entries: Vec<Entry>,
}

impl Display for Entries {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut header_line = String::from("| ");
        let mut spacer_line = String::from("| ");
        let mut val_line = String::from("| ");

        let longest_val_len = self
            .entries
            .iter()
            .map(|x| format!("{}", x.value.get_type()).len())
            .max()
            .unwrap();
        let longest_key_len = self
            .entries
            .iter()
            .map(|x| format!("{}", x.key).len())
            .max()
            .unwrap();

        let entry_len = if longest_val_len > longest_key_len {
            longest_val_len
        } else {
            longest_key_len
        };

        for entry in &self.entries {
            let value = format!("{}", entry.value.get_type());

            header_line += &entry.key;

            for i in 0..(entry_len) {
                if i < entry.key.len() {
                    spacer_line += "-";
                } else {
                    spacer_line += " ";
                }

                if i < value.len() {
                    val_line += &value.chars().nth(i).unwrap_or(' ').to_string();
                } else {
                    val_line += " ";
                }

                if header_line.len() < spacer_line.len() {
                    header_line += " ";
                }
            }
            header_line += " | ";
            spacer_line += " | ";
            val_line += " | ";
        }

        write!(f, "{}\n{}\n{}", header_line, spacer_line, val_line)
    }
}

impl Index<usize> for Entries {
    type Output = Entry;
    fn index<'a>(&'a self, i: usize) -> &'a Entry {
        &self.entries[i]
    }
}

/// Row query is a query tool for filtering rows
pub struct RowQuery {
    entry: Option<Entry>,
}

impl RowQuery {
    /// Check row query exists
    pub fn exists(&self) -> bool {
        self.entry.is_some()
    }

    /// If type is array get its length
    ///
    /// ## Returns
    /// This function returns -1 if the type is not an array
    ///
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("school", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("students", TypeDefs::array_of(TypeDefs::String)),
    /// ]);
    /// db.table_unwrap("school").insert(vec![
    ///     "IstinyeAnadolu".into(),
    ///     vec!["Ahmet", "Hasan", "Huseyin"].into(),
    /// ]);
    /// let first_column = &db.table_unwrap("school").get_where(|x| x.row("students").size() > 2 )[0];
    ///  assert_eq!(first_column.row("students").size(), 3);
    /// ```
    pub fn size(&self) -> isize {
        if let Some(entry) = &self.entry {
            match entry.value.get_type() {
                Types::Array(e) => e.len() as isize,
                _ => -1,
            }
        } else {
            -1
        }
    }

    /// Check if entry is the equvalent of the given value
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let first_column = &db.table("users").unwrap().get_where(|x| x.row("name").is("John".to_string()))[0];
    /// assert_eq!(first_column.row("name").is("John".to_string()), true);
    /// ```
    pub fn is<T>(&self, key: T) -> bool
    where
        T: Into<SafeType> + PartialEq,
    {
        if let Some(entry) = &self.entry {
            let f = Into::into(key);
            f == entry.value
            /*
            match &f {
                Types::String(_) => entry.value.get_type_name() == TypeDefs::String && entry.value == f,
                Types::Char(_) => entry.value.get_type_name() == TypeDefs::Char && entry.value == f,
                Types::I8(_) => entry.value.is_i8() && entry.value == f,
                Types::I64(_) => entry.value.is_i64() && entry.value == f,
                Types::U64(_) => entry.value.is_u64() && entry.value == f,
                Types::F32(_) => entry.value.is_f32() && entry.value == f,
                Types::F64(_) => entry.value.is_f64() && entry.value == f,
                Types::Bool(_) => entry.value.is_bool() && entry.value == f,
                Types::Array(e) => {
                    entry.value.is_array()
                        && matches!(entry.value.get_array_type(), Some(x) if x == e)
                        && entry.value == f
                }
            }
            */
            //key == entry.value.clone().into()
        } else {
            false
        }
    }

    /// Check if entry is the equvalent of the given type
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let first_column = &db.table("users").unwrap().get_where(|x| x.row("name").is("John".to_string()))[0];
    /// assert_eq!(first_column.row("name").is_it(TypeDefs::String), true);
    /// ```
    pub fn is_it(&self, key: TypeDefs) -> bool {
        if let Some(entry) = &self.entry {
            key == entry.value.get_type_def()
        } else {
            false
        }
    }

    /// Get the value of the entry
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let first_column = &db.table("users").unwrap().get_where(|x| x.row("name").is("John".to_string()))[0];
    /// assert_eq!(first_column.row("name").get_value(), Some("John".to_string()));
    /// ```
    pub fn get_value<T>(&self) -> Option<T>
    where
        T: From<Types>,
    {
        if let Some(entry) = &self.entry {
            Some(entry.value.get())
        } else {
            None
        }
    }
}

impl Entries {
    /// Get the value of a column by key
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let entries = db.table("users").unwrap().get_all();
    /// let first_entry = entries[0].clone();
    /// assert_eq!(first_entry.get("name").unwrap().key, "name");
    /// ```
    pub fn get(&self, key: &str) -> Option<&Entry> {
        self.entries.iter().find(|x| x.key == key).map(|x| x)
    }

    /// Get the value of a column by index
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///     TableRow::new("name", TypeDefs::String),
    ///     TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let entries = db.table("users").unwrap().get_all();
    /// let first_entry = entries[0].clone();
    /// assert_eq!(first_entry.get_at(0).unwrap().key, "name");
    /// ```
    pub fn get_at(&self, index: usize) -> Option<&Entry> {
        self.entries.get(index).map(|x| x)
    }

    /// Get the row, This function extends to `RowQuery`
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///     TableRow::new("name", TypeDefs::String),
    ///     TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let entries = db.table("users").unwrap().get_all();
    /// let first_entry = entries[0].clone();
    /// assert_eq!(first_entry.row("name").is("John".to_string()), true);
    /// ```
    pub fn row(&self, key: &str) -> RowQuery {
        RowQuery {
            entry: match self.entries.iter().find(|x| x.key == key).map(|x| x) {
                Some(x) => Some(x.clone()),
                None => None,
            },
        }
    }
}

/// A row in a table
#[derive(Debug, Clone)]
pub struct TableRow {
    /// Name of row
    pub key: String,
    /// Type of row
    pub rtype: TypeDefs,
}

impl TableRow {
    /// Create a new table row
    /// ## Arguments
    /// * `name` - Name of row
    /// * `rtype` - Type of row
    /// ## Returns
    /// * [`TableRow`]
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///  TableRow::new("name", TypeDefs::String),
    /// TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// ```
    pub fn new(key: &str, rtype: TypeDefs) -> Self {
        TableRow {
            key: key.to_string(),
            rtype,
        }
    }
}

impl Table {
    /// Get table name
    /// ## Returns
    /// [`&str`] Table name
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs, Types}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///     TableRow::new("name", TypeDefs::String),
    ///     TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// assert_eq!(db.table("users").unwrap().get_name(), "users");
    /// ```
    pub fn get_name(&self) -> &str {
        &self.name
    }

    ///Get Headers
    /// ## Returns
    /// * [`Vec<TableRow>`] Vector of table rows
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{TableRow, TypeDefs}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// let headers = db.table("users").unwrap().get_headers();
    /// assert_eq!(headers.len(), 2);
    /// assert_eq!(headers[0].key, "name");
    /// assert_eq!(headers[0].rtype, TypeDefs::String);
    /// ```
    pub fn get_headers(&self) -> Vec<TableRow> {
        self.headers.clone()
    }

    /// Get all columns as a vector of Entries
    /// ## Returns
    /// * [`Vec<Entries>`]
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{Entries, Entry, Types, TypeDefs, TableRow}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///   TableRow::new("name", TypeDefs::String),
    ///   TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///   "John".into(),
    ///   12_i64.into()
    /// ]);
    /// let entries = db.table("users").unwrap().get_all();
    /// ```
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
    /// ## Example
    /// ```
    /// use safe_en::{Database, table::{Entries, Entry, Types, TypeDefs, TableRow}};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///     TableRow::new("name", TypeDefs::String),
    ///     TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().insert(vec![
    ///     "John".into(),
    ///     12_i64.into()
    /// ]);
    /// let entries = db.table("users").unwrap().get_at(0).unwrap();
    /// ```
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

    /// Remove a row by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// ## Returns
    /// [`usize`] Effected row length
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// use safe_en::table::{TableRow, TypeDefs};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().remove_where(|entry| entry.row("name").is("Ahmet".to_string()));
    /// ```
    pub fn remove_where<E: Fn(Entries) -> bool + Clone + Sized>(&mut self, filter: E) -> usize {
        let mut found_entries = Vec::new();
        for (index, entries) in self.columns.iter().enumerate() {
            let fake_entries = Entries {
                entries: entries
                    .iter()
                    .enumerate()
                    .map(|(ix, value)| Entry {
                        key: self.headers[ix].key.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            };

            if filter(fake_entries.clone()) {
                found_entries.push(index);
            }
        }

        for i in &found_entries {
            self.columns.remove(*i);
        }
        found_entries.len()
    }

    /// Get the value of a column by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// * `value` - Value to set
    /// ## Returns
    /// [`Vec<Entries>`]
    /// ## Example
    /// ```
    /// use safe_en::Database;
    /// use safe_en::table::{TableRow, TypeDefs};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///   TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().get_where(|entry| entry.row("name").is("Ahmet".to_string()));
    /// ```
    pub fn get_where<E: Fn(Entries) -> bool + Clone + Sized>(&self, filter: E) -> Vec<Entries> {
        let mut found_entries = Vec::new();
        for entries in self.columns.iter() {
            let fake_entries = Entries {
                entries: entries
                    .iter()
                    .enumerate()
                    .map(|(ix, value)| Entry {
                        key: self.headers[ix].key.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            };

            if filter(fake_entries.clone()) {
                found_entries.push(fake_entries);
            }
        }
        found_entries
    }

    /// Increase the value of a number by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// ## Returns
    /// * [`Ok<()>`]
    /// * [`Err<Vec<String>>`] - Error messages
    /// ## Example
    /// ```rust
    /// use safe_en::Database;
    /// use safe_en::table::{TableRow, TypeDefs};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().inc_where(|x| {
    ///     x.row("age").exists()
    /// }, "age");
    /// //Increases all ages by 1
    /// ```
    pub fn inc_where<E: Fn(Entries) -> bool + Clone + Sized>(
        &mut self,
        filter: E,
        row: &str,
    ) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        for entries in &mut self.columns {
            let fake_entries = Entries {
                entries: entries
                    .iter()
                    .enumerate()
                    .map(|(ix, value)| Entry {
                        key: self.headers[ix].key.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            };

            if filter(fake_entries.clone()) {
                let header_pos = self.headers.iter().position(|x| x.key == row).unwrap();
                match match entries[header_pos].clone().rtype {
                    Types::I8(e) => {
                        if e == std::i8::MAX {
                            errors.push(format!("'I8' about to be overflow"));
                            None
                        } else {
                            Some(Types::I8(e + 1))
                        }
                    }
                    Types::I64(e) => {
                        if e == i64::max_value() {
                            errors.push(format!("'I64' about to be overflow"));
                            None
                        } else {
                            Some(Types::I64(e + 1))
                        }
                    }
                    Types::U64(e) => {
                        if e == u64::max_value() {
                            errors.push(format!("'U64' about to be overflow"));
                            None
                        } else {
                            Some(Types::U64(e + 1))
                        }
                    }
                    Types::F32(e) => {
                        if e == f32::MAX {
                            errors.push(format!("'F32' about to be overflow"));
                            None
                        } else {
                            Some(Types::F32(e + 1.))
                        }
                    }
                    Types::F64(e) => {
                        if e == f64::MAX {
                            errors.push(format!("'F64' about to be overflow"));
                            None
                        } else {
                            Some(Types::F64(e + 1.))
                        }
                    }
                    _ => {
                        errors.push(format!("{} is not a integer type column", row));
                        None
                    }
                } {
                    Some(e) => {
                        entries[header_pos].rtype = e;
                    }
                    None => {}
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Increase the value of a number by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// ## Returns
    /// * [`Ok<()>`]
    /// * [`Err<Vec<String>>`] - Error messages
    /// ## Example
    /// ```rust
    /// use safe_en::Database;
    /// use safe_en::table::{TableRow, TypeDefs};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().inc_where(|x| {
    ///     x.row("age").exists()
    /// }, "age");
    /// //Increases all ages by 1
    /// ```
    pub fn push_where<E: Fn(Entries) -> bool + Clone + Sized>(
        &mut self,
        filter: E,
        row: &str,
        value: SafeType,
    ) -> Result<(), Vec<String>> {
        let mut errors = vec![];
        for entries in &mut self.columns {
            let fake_entries = Entries {
                entries: entries
                    .iter()
                    .enumerate()
                    .map(|(ix, value)| Entry {
                        key: self.headers[ix].key.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            };

            if filter(fake_entries.clone()) {
                let header_pos = self.headers.iter().position(|x| x.key == row).unwrap();
                match match entries[header_pos].clone().rtype {
                    Types::Array(e) => {
                        let type_inner_type = entries[header_pos]
                            .clone()
                            .get_type_def()
                            .inner_type()
                            .unwrap();

                        if value.get_type().is_array() {
                            let value_inner_type = value.get_type_def().inner_type().unwrap();

                            if type_inner_type == value_inner_type {
                                let mut arr_copy = e.clone();
                                let contents = match value.get_type() {
                                    Types::Array(c) => c,
                                    _ => unreachable!(),
                                };
                                arr_copy.extend(contents);
                                Some(Types::Array(arr_copy))
                            } else {
                                errors.push(format!(
                                    "Inner type of array is '{}' but given value is '{}'",
                                    type_inner_type,
                                    value.get_type()
                                ));
                                None
                            }
                        } else {
                            let mut arr_copy = e.clone();

                            if type_inner_type == value.get_type_def() {
                                arr_copy.push(value.clone());
                                Some(Types::Array(arr_copy))
                            } else {
                                errors.push(format!(
                                    "Inner type of array is '{}' but given value is '{}'",
                                    type_inner_type,
                                    value.get_type()
                                ));
                                None
                            }
                        }
                    }
                    _ => {
                        errors.push("Field is not a array type".to_string());
                        None
                    }
                } {
                    Some(e) => {
                        entries[header_pos].rtype = e;
                    }
                    None => (),
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Set the value of a column by filter
    /// ## Arguments
    /// * `filter` - Filter function [`Fn(`Entry`) -> bool`]
    /// * `value` - Value to set
    /// ## Returns
    /// * [`Ok<usize>`] - Effected row length
    /// * [`Err<Vec<String>>`] - Error messages
    /// ## Example
    /// ```rust
    /// use safe_en::Database;
    /// use safe_en::table::{TableRow, TypeDefs};
    /// let mut db = Database::new();
    /// db.create_table("users", vec![
    ///    TableRow::new("name", TypeDefs::String),
    ///    TableRow::new("age", TypeDefs::I64),
    /// ]);
    /// db.table("users").unwrap().set_where(|x| {
    ///     x.row("name").is("Ahmet".to_string())
    /// }, vec![
    ///     safe_en::table::Entry {
    ///         key: "name".to_string(),
    ///         value: "Ahmetcan".into(),
    ///     },
    /// ]);
    /// ```
    pub fn set_where<E: Fn(Entries) -> bool + Clone + Sized, T>(
        &mut self,
        filter: E,
        value: Vec<Entry>,
    ) -> Result<usize, Vec<String>>
    where
        Types: From<T>,
        T: Clone,
    {
        let mut changed_rows = 0;
        let mut errors = vec![];
        if value.len() > self.headers.len() {
            errors.push("Value length is not equal to header length".to_string());
            return Err(errors);
        }
        'entryloop: for entries in &mut self.columns {
            let fake_entries = Entries {
                entries: entries
                    .iter()
                    .enumerate()
                    .map(|(ix, value)| Entry {
                        key: self.headers[ix].key.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            };

            if filter(fake_entries.clone()) {
                for value_entry in value.iter() {
                    let targt = fake_entries
                        .entries
                        .iter()
                        .find(|x| x.key == value_entry.key);
                    if let Some(target) = targt {
                        if target.value.get_type_def() == value_entry.value.get_type_def() {
                            let header_pos = self
                                .headers
                                .iter()
                                .position(|x| x.key == value_entry.key)
                                .unwrap();
                            changed_rows += 1;
                            entries[header_pos] = value_entry.value.clone();
                        } else {
                            errors.push(format!(
                                "Value type is not equal to header type. Header: {}, Value: {}",
                                target.value.get_type_def(),
                                value_entry.value.get_type_def()
                            ));
                            break 'entryloop;
                        }
                    } else {
                        errors.push(format!("Could not find key '{}' in table", value_entry.key));
                        break 'entryloop;
                    }
                }
            }
        }
        if errors.is_empty() {
            Ok(changed_rows)
        } else {
            Err(errors)
        }
    }

    /// Insert data to table
    /// ## Arguments
    /// * `rows` - [`TableRow`]
    /// ## Returns
    /// * [`Result<()>`]
    /// * [`Err<Vec<String>>`] for insert errors
    /// ## Example
    /// ```
    /// use safe_en::{table::{TableRow, TypeDefs, Types},Database};
    /// let mut db = Database::new();
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
    pub fn insert(&mut self, rows: Vec<SafeType>) -> Result<(), Vec<String>> {
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
            let rtype: SafeType = rows[i].clone().into();
            if header.rtype == rtype.get_type_def() {
                _rows.push(rtype);
            } else {
                errors.push(format!(
                    "Type mismatch, expected {}, got {} on column {}",
                    header.rtype,
                    rtype.get_type_def(),
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
