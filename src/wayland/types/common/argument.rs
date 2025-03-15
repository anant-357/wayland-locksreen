use crate::wayland::types::common::parse_utils::ParseResult;
use std::io::{Cursor, Read};

pub enum Argument {
    Int(i32),
    Uint(u32),
    Fixed(f32),
    String(Option<String>),
    Object(u32),
    NewId {
        id: u32,
        interface: Option<String>,
        version: Option<u32>,
    },
    Array(Vec<u8>),
    Fd(i32),
}

impl Argument {
    pub fn parse_wayland_u32(data: &mut Cursor<&[u8]>) -> ParseResult<u32> {
        let mut bytes = [0u8; 4];
        data.read_exact(&mut bytes)?;
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn parse_wayland_u16(data: &mut Cursor<&[u8]>) -> ParseResult<u16> {
        let mut bytes = [0u8; 2];
        data.read_exact(&mut bytes)?;
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn parse_wayland_string(cursor: &mut Cursor<&[u8]>) -> ParseResult<Option<String>> {
        Self::parse_wayland_u32(cursor)?;
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

        Ok(String::from_utf8(result).ok())
    }
}
