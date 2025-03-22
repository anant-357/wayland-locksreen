use crate::wayland::types::{
    common::argument::Object,
    request::{Message, RequestMessage},
};

pub enum SubPixel {
    Unkown,
    None,
    HorizontalRGB,
    HorizontalBGR,
    VerticalRGB,
    VerticalBGR,
}

pub enum Transform {
    Normal,
    D90,
    D180,
    D270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

pub enum ModeType {
    Current,
    Preferred,
}

pub struct Mode {
    flags: ModeType,
    width: i32,
    height: i32,
    refresh: i32
}

pub struct OutputGeometry {
    x: i32,
    y: i32,
    physical_width: i32,
    physical_height: i32,
    subpixel: SubPixel,
    make: String,
    model: String,
    transform: Transform,
}

pub struct WlOutput {
    id: Object,
    geometry: Option<OutputGeometry>,
    factor: Option<i32>,
    name: Option<String>,
    description: Option<String>,
    modes: Vec<Mode>
}
