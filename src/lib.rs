use std::{
    fs::File,
    io::{Read, Write},
    panic,
    str::pattern::StrSearcher,
};

use table::{Table, TableRow, TypeDefs};

use crate::table::Types;
pub mod table;

/*

TableNameSize = 128 bytes,TableHeadersSize = 128 bytes,TableDataSize = 128 bytes



*/

pub struct Database {
    name: String,
    size: usize,
    tables: Vec<table::Table>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tables: Vec::new(),
            name: "".to_string(),
            size: 0,
        }
    }

    ///Sets name of the database
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    ///Returns name of the database
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    ///Returns size of the database
    pub fn get_size(&self) -> usize {
        self.size
    }

    ///Returns number of tables in the database
    pub fn get_number_of_tables(&self) -> usize {
        self.tables.len()
    }

    /// Get query
    pub fn table(&mut self, table_name: &str) -> Option<&mut Table> {
        self.tables.iter_mut().find(|x| x.name == table_name)
    }

    /// Creates table
    pub fn create_table(&mut self, table_name: &str, rows: Vec<TableRow>) -> Result<(), ()> {
        let table = table::Table {
            name: table_name.to_owned(),
            headers: rows,
            columns: vec![],
            size_of_columns: 0,
            size_of_rows: 0,
        };
        if self.tables.iter().find(|x| x.name == table_name).is_some() {
            return Err(());
        } else {
            self.tables.push(table);
            Ok(())
        }
    }

    /// Load database
    pub fn load(&self, path: &str) {
        //let mut file = File::open(path).unwrap();
        //let db_name_len = [0u8; 128];
        //file.read_exact(&mut db_name_len).unwrap();
        //let db_name = vec![0u8; u64::from(db_name_len) as usize];
        //file.read_exact(buf)
    }

    /// Saves database
    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        for table in self.tables.iter() {
            let mut bytes = vec![];

            fn get_id_of_data(data: TypeDefs, second_dimentsion: bool) -> [u8; 2] {
                //Three bytes seperated for each data type
                //First byte is base byte
                //String is 1
                //Char is 2
                //I64 is 3
                //U64 is 4
                //Bool is 5
                //F32 is 6
                //F64 is 7
                //Array is 8
                //Second byte is the child of array if base is array

                let first_byte = match data {
                    TypeDefs::String => 1,
                    TypeDefs::Char => 2,
                    TypeDefs::I64 => 3,
                    TypeDefs::U64 => 4,
                    TypeDefs::Bool => 5,
                    TypeDefs::F32 => 6,
                    TypeDefs::F64 => 7,
                    TypeDefs::Array(_) => 8,
                };

                let second_byte = match data {
                    TypeDefs::String => 0,
                    TypeDefs::Char => 0,
                    TypeDefs::I64 => 0,
                    TypeDefs::U64 => 0,
                    TypeDefs::Bool => 0,
                    TypeDefs::F32 => 0,
                    TypeDefs::F64 => 0,
                    TypeDefs::Array(x) => get_id_of_data(*x, true)[0],
                };
                [first_byte, second_byte]
            }

            bytes.push(table.name.len() as u8);
            bytes.extend_from_slice(table.name.as_bytes());
            bytes.push(table.headers.len() as u8);
            for header in table.headers.iter() {
                //Key length
                bytes.push(header.key.len() as u8);
                //Key bytes
                bytes.extend_from_slice(header.key.as_bytes());
                //Data type
                bytes.extend(get_id_of_data(header.rtype.clone(), false));
                //Nullable
                bytes.push(header.nullable as u8);
                //Auto increment
                bytes.push(header.auto_increment as u8);
                println!("Header {:?} |{:?}", header.key, header.rtype);
            }

            for row in table.columns.iter() {
                for data in row.iter() {
                    match data {
                        Types::String(x) => {
                            bytes.push(1);
                            bytes.push(0);
                            bytes.extend_from_slice(x.as_bytes());
                        }
                        Types::Char(x) => {
                            bytes.push(2);
                            bytes.push(0);
                            bytes.push(x.to_digit(10).unwrap() as u8);
                        }
                        Types::I64(x) => {
                            bytes.extend_from_slice(&x.to_be_bytes());
                        }
                        Types::U64(x) => {
                            bytes.extend_from_slice(&x.to_le_bytes());
                        }
                        Types::Bool(x) => {
                            bytes.push(*x as u8);
                        }
                        Types::F32(x) => {
                            bytes.extend_from_slice(&x.to_le_bytes());
                        }
                        Types::F64(x) => {
                            bytes.extend_from_slice(&x.to_le_bytes());
                        }
                        Types::Array(x) => {
                            panic!("unimplemented");
                            //bytes.extend_from_slice(&x.len().to_le_bytes());
                            //for data in x.iter() {
                            //
                            //    bytes.extend_from_slice(&get_id_of_data(*data, true));
                            //}
                        }
                    }
                }
                println!("Data {:?}", row);
            }

            println!("Bytes: {:?}", bytes);
            file.write_all(&bytes).unwrap();
        }
    }
}
