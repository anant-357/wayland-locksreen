use crate::wayland::types::common::parse_utils::ParseResult;
use std::io::{Cursor, Read};

pub trait Argument: Sized {
    fn decode(data: &mut Cursor<&[u8]>) -> ParseResult<Self>;

    fn encode(&self) -> ParseResult<Vec<u8>>;

    fn encode_extend(&self, buffer: Vec<u8>) -> ParseResult<Vec<u8>>;
}

impl Argument for u32 {
    fn decode(data: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        let mut bytes = [0u8; 4];
        data.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    fn encode(&self) -> ParseResult<Vec<u8>> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn encode_extend(&self, mut buffer: Vec<u8>) -> ParseResult<Vec<u8>> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(buffer)
    }
}

impl Argument for u16 {
    fn decode(data: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        let mut bytes = [0u8; 2];
        data.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    fn encode(&self) -> ParseResult<Vec<u8>> {
        Ok(self.to_le_bytes().to_vec())
    }

    fn encode_extend(&self, mut buffer: Vec<u8>) -> ParseResult<Vec<u8>> {
        buffer.extend_from_slice(&self.to_le_bytes());
        Ok(buffer)
    }
}

impl Argument for String {
    fn decode(cursor: &mut Cursor<&[u8]>) -> ParseResult<String> {
        u32::decode(cursor)?;
        let mut result = Vec::new();
        let mut byte = [0];

        while cursor.read_exact(&mut byte).is_ok() {
            if byte[0] == 0 {
                break;
            }
            result.push(byte[0]);
        }

        let pos = cursor.position() as usize;
        let padding = (4 - (pos % 4)) % 4;

        cursor
            .set_position((cursor.position() + padding as u64).min(cursor.get_ref().len() as u64));

        Ok(String::from_utf8(result)?)
    }

    fn encode(&self) -> ParseResult<Vec<u8>> {
        let buffer: Vec<u8> = Vec::new();
        let zero: u32 = self.len() as u32 + 1;
        let mut buffer = zero.encode_extend(buffer)?;
        buffer.extend_from_slice(self.as_bytes());
        buffer.push(0);
        while buffer.len() % 4 != 0 {
            buffer.push(0);
        }
        Ok(buffer)
    }

    fn encode_extend(&self, mut buffer: Vec<u8>) -> ParseResult<Vec<u8>> {
        buffer.extend_from_slice(&self.encode()?);
        Ok(buffer)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Object(u32);

impl Object {
    pub fn inner(&self) -> u32 {
        self.0
    }

    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn len(&self) -> usize {
        4
    }
}

impl Argument for Object {
    fn decode(data: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        let mut bytes = [0u8; 4];
        data.read_exact(&mut bytes)?;
        Ok(Self(u32::from_le_bytes(bytes)))
    }

    fn encode(&self) -> ParseResult<Vec<u8>> {
        Ok(self.0.to_le_bytes().to_vec())
    }

    fn encode_extend(&self, mut buffer: Vec<u8>) -> ParseResult<Vec<u8>> {
        buffer.extend_from_slice(&self.0.to_le_bytes());
        Ok(buffer)
    }
}

#[derive(Debug, Clone)]
pub struct NewId(String, u32, Object);

impl NewId {
    pub fn new(interface: (String, u32), new_id: Object) -> Self {
        Self(interface.0, interface.1, new_id)
    }

    pub fn len(&self) -> usize {
        self.0.len() + 4 + self.2.len() + 1
    }
}

impl Argument for NewId {
    fn decode(data: &mut Cursor<&[u8]>) -> ParseResult<Self> {
        let interface = String::decode(data)?;
        let version = u32::decode(data)?;
        let new_id = Object::decode(data)?;

        Ok(Self(interface, version, new_id))
    }

    fn encode(&self) -> ParseResult<Vec<u8>> {
        let buffer = self.0.encode_extend(Vec::new())?;
        let buffer = self.1.encode_extend(buffer)?;
        let buffer = self.2.encode_extend(buffer)?;

        Ok(buffer)
    }

    fn encode_extend(&self, mut buffer: Vec<u8>) -> ParseResult<Vec<u8>> {
        buffer.extend_from_slice(&self.encode()?);
        Ok(buffer)
    }
}
