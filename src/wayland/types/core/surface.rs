 use crate::wayland::types:: {
    common:: argument::Object,
    request::{Message, RequestMessage}
 };

pub struct WlSurface{
    id: Object
}

impl WlSurface {
    pub fn new(id: Object) -> Self {
        Self {
            id
        }
    }

    pub fn destroy(&self) -> RequestMessage {
        RequestMessage::build(self.id, 0, 8, Message::Empty)
    }

}
