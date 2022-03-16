

pub struct RawType {
    pub type_size: usize,
    pub type_data: Vec<u8>,
}

pub fn type_to_bytes(type_: T) -> RawType where T: Types {
    let mut bytes = Vec::new();
    for c in type_.chars() {
        bytes.push(c as u8);
    }
    bytes
}