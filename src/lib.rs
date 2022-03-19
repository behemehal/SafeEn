use core::fmt;
use std::{fs::File, io::Write, panic};

use table::{Table, TableRow, TypeDefs};

use crate::table::Types;
pub mod table;
pub mod utils;

#[derive(Debug, Clone)]
pub struct LoadError;

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
        let db_name: String = utils::read_data(&mut file, TypeDefs::String).into();
        let table_len: u64 = utils::read_data(&mut file, TypeDefs::U64).into();
        self.set_name(db_name);
        for _ in 0..table_len {
            let table_name: String = utils::read_data(&mut file, TypeDefs::String).into();
            let table_headers_len: u64 = utils::read_data(&mut file, TypeDefs::U64).into();
            let mut table_rows: Vec<TableRow> = Vec::new();
            for _ in 0..table_headers_len {
                let table_header: String = utils::read_data(&mut file, TypeDefs::String).into();

                let base_header_type: i8 = utils::read_data(&mut file, TypeDefs::I8).into();
                let second_header_type: i8 = utils::read_data(&mut file, TypeDefs::I8).into();
                let nullable: bool = utils::read_data(&mut file, TypeDefs::Bool).into();
                let mut row = TableRow::new(
                    table_header,
                    TypeDefs::from_base_and_second_layer(
                        base_header_type as u8,
                        second_header_type as u8,
                    ),
                );
                if nullable {
                    row.set_nullable();
                }
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
                        Err(_) => {
                            return Err(LoadError)
                        },
                    },
                    None => return Err(LoadError),
                }
            }
            
        }

        /*
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
        */
        Ok(())

        //let db_name = vec![0u8; u64::from(db_name_len) as usize];
        //file.read_exact(buf)
    }

    /// Saves database
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
                bytes.push(header.nullable as u8);
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
                        Types::Array(_) => {
                            panic!("Array not supported yet");
                        }
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

        /*

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
        */
    }
}
