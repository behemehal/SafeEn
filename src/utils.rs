use crate::table::{TypeDefs, Types};
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct RawType {
    pub type_size: usize,
    pub type_data: Vec<u8>,
}

pub fn read_data(data: &mut File, rtype: TypeDefs) -> Types {
    match rtype {
        TypeDefs::String => {
            let header_size = read_data(data, TypeDefs::I8);
            let header_size: i8 = header_size.into();
            let mut header = vec![0; header_size as usize];
            data.read_exact(&mut header).unwrap();
            let mut str_buffer = vec![0; header[0] as usize];
            data.read_exact(&mut str_buffer).unwrap();
            let st = String::from_utf8(str_buffer).unwrap();
            Types::String(st)
        }
        TypeDefs::Char => todo!(),
        TypeDefs::I8 => {
            let mut buffer = [0; 1];
            data.read_exact(&mut buffer).unwrap();
            Types::I8(buffer[0] as i8)
        }
        TypeDefs::I64 => {
            read_data(data, TypeDefs::I8);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            Types::I64(i64::from_le_bytes(header))
        }
        TypeDefs::U64 => {
            read_data(data, TypeDefs::I8);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            Types::U64(u64::from_le_bytes(header))
        }
        TypeDefs::Bool => {
            let mut buffer = [0; 1];
            data.read_exact(&mut buffer).unwrap();
            Types::Bool(buffer[0] != 0)
        }
        TypeDefs::F32 => todo!(),
        TypeDefs::F64 => todo!(),
        TypeDefs::Array(_) => todo!(),
    }
}

pub fn extend_bytes_from_raw_type(bytes: &mut Vec<u8>, raw_type: &RawType) {
    bytes.push(raw_type.type_size as u8);
    bytes.extend_from_slice(&raw_type.type_data);
}

pub fn type_to_bytes<T>(type_: T) -> RawType
where
    T: Into<Types>,
{
    let rtype: Types = type_.into();
    let mut type_size = 0;
    let mut type_data = Vec::new();
    match rtype {
        Types::String(data) => {
            type_size = std::mem::size_of::<usize>();
            type_data.extend(data.len().to_le_bytes().to_vec());
            type_data.extend_from_slice(data.as_bytes());
        }
        Types::Char(data) => {
            type_size = 1;
            type_data = (data as u32).to_le_bytes().to_vec();
        }
        Types::I8(data) => {
            type_size = std::mem::size_of::<i8>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::I64(data) => {
            type_size = std::mem::size_of::<i64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::U64(data) => {
            type_size = std::mem::size_of::<u64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::Bool(data) => {
            type_size = std::mem::size_of::<bool>();
            type_data = vec![if data { 1 } else { 0 }];
        }
        Types::F32(data) => {
            type_size = std::mem::size_of::<f32>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::F64(data) => {
            type_size = std::mem::size_of::<f64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::Array(data) => {
            type_size = std::mem::size_of::<usize>();
            type_data = data.len().to_le_bytes().to_vec();
            for e in data {
                let mut e_bytes = type_to_bytes(e);
                type_data.append(&mut e_bytes.type_data);
            }
        }
    }
    RawType {
        type_size,
        type_data,
    }
}
