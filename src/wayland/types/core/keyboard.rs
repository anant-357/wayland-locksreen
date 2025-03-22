use crate::wayland::types::{
    common::argument::Object,
    request::{Message, RequestMessage},
};


#[derive(Debug, Clone, Copy)]
enum KeymapFormat {
    NoKeymap,
    XKBV1
}

#[derive(Debug, Clone, Copy)]
enum KeyState {
    Released,
    Pressed,
    // since version 10
    Repeated
}

#[derive(Debug, Clone, Copy)]
struct Keymap {
    // implement fd first
    format: KeymapFormat,
    fd: u32,
    size: u32
}

#[derive(Debug, Clone, Copy)]
pub struct WlKeyboard{
    id: Object,
    keymap: Option<Keymap>
}


