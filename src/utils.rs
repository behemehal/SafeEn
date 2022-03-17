use std::{io::Read, fs::File, panic};

use byteorder::{LittleEndian, ReadBytesExt, ByteOrder};

use crate::table::{Types, TypeDefs};

#[derive(Debug)]
pub struct RawType {
    pub type_size: usize,
    pub type_data: Vec<u8>,
}

pub fn read_data(data: &mut File, rtype: TypeDefs) -> Types {
    let mut data_size_len = [0u8; 1];
    let mut data_size_len = [0u8; 1];
    data.read_exact(&mut data_size_len);
    let data_size_len = data_size_len[0] as usize;
    let mut data_size = vec![0u8; data_size_len];
    data.read_exact(&mut data_size);

    println!("TypeSize: {} raw type: {:?}", data_size_len, data_size);

    match rtype {
        TypeDefs::String => {
            let data_size = usize::from_le_bytes(data_size[0..data_size_len].try_into().unwrap());
            let mut data_data = vec![0u8; data_size];
            data.read_exact(&mut data_data);
            let data_data = String::from_utf8(data_data).unwrap();
            Types::String(data_data)
        },
        TypeDefs::Char => todo!(),
        TypeDefs::I8 => {
            let data_size = data_size[0] as i8;
            Types::I8(data_size)
        },
        TypeDefs::I64 => {
            let mut data_data = [0u8; 8];
            data.read_exact(&mut data_data);
            let data_data = i64::from_le_bytes(data_size[0..data_size_len].try_into().unwrap());
            Types::I64(data_data)
        },
        TypeDefs::U64 => {
            let mut data_data = [0u8; 8];
            data.read_exact(&mut data_data);
            let data_data = u64::from_le_bytes(data_size[0..data_size_len].try_into().unwrap());
            Types::U64(data_data)
        },
        TypeDefs::Bool => {
            let mut data_size = [0u8; 1];
            data.read_exact(&mut data_size);
            let data_size = data_size[0] as u64;
            todo!()

        },
        TypeDefs::F32 => {
            let mut data_size = [0u8; 4];
            data.read_exact(&mut data_size);
            let data_size = f32::from_le_bytes(data_size);
            todo!()

        },
        TypeDefs::F64 => {
            let mut data_size = [0u8; 8];
            data.read_exact(&mut data_size);
            let data_size = f64::from_le_bytes(data_size);
            todo!()

        },
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
