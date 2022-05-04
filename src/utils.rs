#![allow(unused_variables)]
use crate::table::{SafeType, TypeDefs, Types};
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

pub(crate) fn read_data(data: &mut File, rtype: TypeDefs) -> SafeType {
    match rtype {
        TypeDefs::String => {
            let header_size = read_one(data);
            let header_size: i8 = header_size.into();
            let mut header = vec![0; header_size as usize];
            data.read_exact(&mut header).unwrap();
            let mut str_buffer = vec![0; header[0] as usize];
            data.read_exact(&mut str_buffer).unwrap();
            let st = String::from_utf8(str_buffer).unwrap();
            SafeType {
                type_id: rtype,
                rtype: st.into(),
            }
        }
        TypeDefs::Char => {
            read_one(data);
            let mut header = [0; 4];
            data.read_exact(&mut header).unwrap();
            SafeType {
                type_id: rtype,
                rtype: char::from_u32(u32::from_le_bytes(header)).unwrap().into(),
            }
        }
        TypeDefs::I8 => {
            let mut buffer = [0; 2];
            data.read_exact(&mut buffer).unwrap();
            SafeType {
                type_id: rtype,
                rtype: (buffer[1] as i8).into(),
            }
        }
        TypeDefs::I64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            SafeType {
                type_id: rtype,
                rtype: i64::from_le_bytes(header).into(),
            }
        }
        TypeDefs::U64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            SafeType {
                type_id: rtype,
                rtype: u64::from_le_bytes(header).into(),
            }
        }
        TypeDefs::Bool => {
            let mut buffer = [0; 2];
            data.read_exact(&mut buffer).unwrap();
            SafeType {
                type_id: rtype,
                rtype: (buffer[1] == 1).into(),
            }
        }
        TypeDefs::F32 => {
            read_one(data);
            let mut header = [0; 4];
            data.read_exact(&mut header).unwrap();
            SafeType {
                type_id: rtype,
                rtype: (f32::from_le_bytes(header)).into(),
            }
        }
        TypeDefs::F64 => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            SafeType {
                type_id: rtype,
                rtype: (f64::from_le_bytes(header)).into(),
            }
        }
        TypeDefs::Array(ref e) => {
            read_one(data);
            let mut header = [0; 8];
            data.read_exact(&mut header).unwrap();
            let array_size = usize::from_le_bytes(header);
            let mut array = Vec::with_capacity(array_size);
            for _ in 0..array_size {
                let data = read_data(data, *e.clone());
                array.push(data);
            }
            SafeType {
                type_id: rtype,
                rtype: Types::Array(array),
            }
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
                extend_bytes_from_raw_type(&mut type_data, &type_to_bytes(e.get_type()));
            }
        }
    }
    RawType {
        type_size: _type_size,
        type_data,
    }
}
