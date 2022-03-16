use core::fmt;
use std::{
    fs::File,
    io::{Read, Write},
    panic, error::Error,
};

use table::{Table, TableRow, TypeDefs};

use crate::table::Types;
pub mod table;

/*

TableNameSize = 128 bytes,TableHeadersSize = 128 bytes,TableDataSize = 128 bytes



*/


#[derive(Debug, Clone)]
pub struct LoadError;

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to load db from file")
    }
}

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
        };
        if self.tables.iter().find(|x| x.name == table_name).is_some() {
            return Err(());
        } else {
            self.tables.push(table);
            Ok(())
        }
    }

    /// Load database
    pub fn load(&mut self, path: &str) -> Result<(), LoadError> {
        let mut file = match File::open(path) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };

        let mut db_name_length = [0; 1];
        match file.read_exact(&mut db_name_length) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };
        let db_name_length = db_name_length[0];

        let mut db_name = vec![0; db_name_length as usize];
        match file.read_exact(&mut db_name) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };
        let db_name = match String::from_utf8(db_name) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };

        self.name = db_name.clone();

        let mut tables_length = [0; 1];
        match file.read_exact(&mut tables_length) {
            Ok(it) => it,
            Err(_) => return Err(LoadError),
        };
        let tables_length = tables_length[0];

        for _ in 0..tables_length {
            let mut table_name_length = [0; 1];
            match file.read_exact(&mut table_name_length) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            };
            let table_name_length = table_name_length[0];

            let mut table_name = vec![0; table_name_length as usize];
            match file.read_exact(&mut table_name) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            };
            let table_name = match String::from_utf8(table_name) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            };
            let mut headers_length = [0; 1];
            match file.read_exact(&mut headers_length) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            };
            let headers_length = headers_length[0];

            let mut table_rows = vec![];

            for _ in 0..headers_length {
                let mut header_name_length = [0; 1];
                match file.read_exact(&mut header_name_length) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let header_name_length = header_name_length[0];

                let mut header_name = vec![0; header_name_length as usize];
                match file.read_exact(&mut header_name) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let header_name = match String::from_utf8(header_name) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };

                let mut base_header_type = [0; 1];
                match file.read_exact(&mut base_header_type) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let base_header = base_header_type[0];

                let mut second_header_type = [0; 1];
                match file.read_exact(&mut second_header_type) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let second_header = second_header_type[0];

                let mut nullable = [0; 1];
                match file.read_exact(&mut nullable) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let nullable = nullable[0];

                let mut table_row = TableRow::new(
                    header_name,
                    TypeDefs::from_base_and_second_layer(base_header, second_header),
                );

                if nullable == 1 {
                    table_row.set_nullable();
                }

                table_rows.push(table_row);
            }

            match self.create_table(&table_name, table_rows) {
                Ok(it) => it,
                Err(_) => return Err(LoadError),
            }

            for _ in 0..headers_length {
                let mut base_type = [0; 1];
                match file.read_exact(&mut base_type) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let base_type = base_type[0];
                let mut second_type = [0; 1];
                match file.read_exact(&mut second_type) {
                    Ok(it) => it,
                    Err(_) => return Err(LoadError),
                };
                let second_type = second_type[0];
                let rtype = TypeDefs::from_base_and_second_layer(base_type, second_type);
            
                //let raw_type = 
                
            }

            

        }
        Ok(())

        //let db_name = vec![0u8; u64::from(db_name_len) as usize];
        //file.read_exact(buf)
    }

    /// Saves database
    pub fn save(&self, path: &str) {
        let mut file = File::create(path).unwrap();

        let mut bytes = vec![];
        bytes.push(size)
        bytes.push(self.name.len());
        bytes.extend_from_slice(self.name.as_bytes());
        bytes.push(self.tables.len() as u8);

        for table in self.tables.iter() {
            fn get_id_of_data(data: TypeDefs) -> [u8; 2] {
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
                    TypeDefs::Array(x) => get_id_of_data(*x)[0],
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
                bytes.extend(get_id_of_data(header.rtype.clone()));
                //Nullable
                bytes.push(header.nullable as u8);
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
                        Types::Array(_) => {
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
