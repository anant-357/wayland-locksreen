use crate::wayland::types::{
    common::argument::Object,
    request::{Message, RequestMessage},
};

#[derive(Debug, Clone, Copy)]
pub struct WlDisplay(Object);

impl WlDisplay {
    pub fn new(id: Object) -> Self {
        Self(id)
    }
    pub fn sync(&self, callback: u32) -> RequestMessage {
        RequestMessage::build(self.0, 0, 12, Message::Sync { callback })
    }

    pub fn get_registry(&self, id: u32) -> RequestMessage {
        RequestMessage::build(
            self.0,
            1,
            12,
            Message::GetRegistry {
                registry: Object::new(id),
            },
        )
    }
}
