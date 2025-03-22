use crate::wayland::types::{
    common::argument::Object,
    request::{Message, RequestMessage},
};

enum Capability {
    Pointer,
    Keyboard,
    Touch
}

#[derive(Debug, Clone, Copy)]
pub struct WlSeat{
    id: Object,
};


