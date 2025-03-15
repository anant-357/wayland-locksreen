use crate::wayland::types::header::Header;

use std::io::{Error, Result};

fn parse_wayland_string(data: &[u8]) -> Option<String> {
    let nul_pos = data.iter().position(|&b| b == 0)?;
    Some(String::from_utf8(data[..nul_pos].to_vec()).unwrap())
}

#[derive(Debug)]
pub enum Event {
    Interface { id: u32, name: String, version: u32 },
    Other,
}

impl Event {
    fn parse(header: Header, payload: &[u8]) -> Result<Self> {
        if header.object_id == 2 && header.opcode == 0 {
            if payload.len() < 8 {
                Err(Error::new(
                    std::io::ErrorKind::Other,
                    "Global event payload too short!",
                ))
            } else {
                let id = u32::from_le_bytes(payload[0..4].try_into().unwrap());
                let version = u32::from_le_bytes(payload[4..8].try_into().unwrap());
                let interface = match parse_wayland_string(&payload[8..]) {
                    Some(name) => Ok(Self::Interface { id, name, version }),
                    None => Err(Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to parse wayland string",
                    )),
                }?;
                tracing::info!("{:?}", interface);
                Ok(interface)
            }
        } else {
            tracing::warn!(
                "Not a global event: id {}, opcode {}",
                header.object_id,
                header.opcode
            );
            Ok(Self::Other)
        }
    }

    pub fn get_global(&self) -> Option<(u32, String, u32)> {
        match self {
            Self::Interface { id, name, version } => Some((*id, name.to_string(), *version)),
            _ => None,
        }
    }
}

pub struct EventMessage {
    header: Header,
    pub payload: Event,
}

impl EventMessage {
    pub fn parse_messages(buffer: &[u8]) -> Vec<Self> {
        let mut messages = Vec::new();
        let mut offset = 0;

        while offset + 8 <= buffer.len() {
            let message = Self::parse(&buffer[offset..]).unwrap();
            offset += message.header.size as usize;
            messages.push(message);
        }

        messages
    }

    fn parse(buffer: &[u8]) -> Result<Self> {
        let object_id = u32::from_le_bytes(buffer[0..4].try_into().unwrap());
        let opcode = u16::from_le_bytes(buffer[4..6].try_into().unwrap());
        let message_size = u16::from_le_bytes(buffer[6..8].try_into().unwrap());
        let header = Header::new(object_id, opcode, message_size);
        if message_size > buffer.len() as u16 {
            tracing::warn!("Incomplete message!");
            Err(Error::new(std::io::ErrorKind::Other, "Incomplete Message"))
        } else {
            let payload = Event::parse(header.clone(), &buffer[8..message_size.into()])?;
            Ok(Self { header, payload })
        }
    }

    pub fn is_global(&self) -> bool {
        if self.header.object_id == 2 && self.header.opcode == 0 {
            true
        } else {
            false
        }
    }
}
