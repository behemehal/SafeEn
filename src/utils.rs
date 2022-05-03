#![allow(unused_variables)]
use crate::table::{TypeDefs, Types};
use std::{fs::File, io::Read};

#[derive(Debug)]
pub(crate) struct RawType {
    pub type_size: usize,
    pub type_data: Vec<u8>,
}

pub(crate) fn read_one(data: &mut File) -> i8 {
    let mut buffer = [0; 1];
    data.read_exact(&mut buffer).unwrap();
    buffer[0] as i8
}

pub(crate) fn read_data(data: &mut File, rtype: TypeDefs) -> Types {
    match rtype {
        TypeDefs::String => {
            let header_size = read_one(data);
            let header_size: i8 = header_size.into();
            let mut header = vec![0; header_size as usize];
            data.read_exact(&mut header).unwrap();
            let mut str_buffer = vec![0; header[0] as usize];
            data.read_exact(&mut str_buffer).unwrap();
            let st = String::from_utf8(str_buffer).unwrap();
            Types::String(st)
        }
        TypeDefs::Char => {
            read_one(data);
            let mut header = [0; 4];
            data.read_exact(&mut header).unwrap();
            Types::Char(char::from_u32(u32::from_le_bytes(header)).unwrap())
        }
        TypeDefs::I8 => {
            let mut buffer = [0; 2];
            data.read_exact(&mut buffer).unwrap();
            Types::I8(buffer[1] as i8)
        }
        TypeDefs::I64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            Types::I64(i64::from_le_bytes(header))
        }
        TypeDefs::U64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            Types::U64(u64::from_le_bytes(header))
        }
        TypeDefs::Bool => {
            let mut buffer = [0; 2];
            data.read_exact(&mut buffer).unwrap();
            Types::Bool(buffer[1] == 1)
        }
        TypeDefs::F32 => {
            read_one(data);
            let mut header = [0; 4];
            data.read_exact(&mut header).unwrap();
            Types::F32(f32::from_le_bytes(header))
        }
        TypeDefs::F64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            Types::F64(f64::from_le_bytes(header))
        }
        TypeDefs::Array(e) => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            let array_size = usize::from_le_bytes(header);
            let mut array = Vec::with_capacity(array_size);
            for _ in 0..array_size {
                let data = read_data(data, *e.clone());
                array.push(data);
            }
            Types::Array(array)
        }
    }
}

pub(crate) fn extend_bytes_from_raw_type(bytes: &mut Vec<u8>, raw_type: &RawType) {
    bytes.push(raw_type.type_size as u8);
    bytes.extend_from_slice(&raw_type.type_data);
}

pub(crate) fn type_to_bytes<T>(type_: T) -> RawType
where
    T: Into<Types>,
{
    let rtype: Types = type_.into();
    #[allow(unused_attributes)]
    let mut _type_size = 0;
    let mut type_data = Vec::new();
    match rtype {
        Types::String(data) => {
            _type_size = core::mem::size_of::<usize>();
            type_data.extend(data.len().to_le_bytes().to_vec());
            type_data.extend_from_slice(data.as_bytes());
        }
        Types::Char(data) => {
            _type_size = core::mem::size_of::<u32>();
            type_data = (data as u32).to_le_bytes().to_vec();
        }
        Types::I8(data) => {
            _type_size = core::mem::size_of::<i8>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::I64(data) => {
            _type_size = core::mem::size_of::<i64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::U64(data) => {
            _type_size = core::mem::size_of::<u64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::Bool(data) => {
            _type_size = 1;
            type_data = vec![if data { 1 } else { 0 }];
        }
        Types::F32(data) => {
            _type_size = core::mem::size_of::<f32>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::F64(data) => {
            _type_size = core::mem::size_of::<f64>();
            type_data = data.to_le_bytes().to_vec();
        }
        Types::Array(data) => {
            _type_size = core::mem::size_of::<usize>();
            type_data = data.len().to_le_bytes().to_vec();
            for e in data {
                extend_bytes_from_raw_type(&mut type_data, &type_to_bytes(e));
            }
        }
    }
    RawType {
        type_size: _type_size,
        type_data,
    }
}
