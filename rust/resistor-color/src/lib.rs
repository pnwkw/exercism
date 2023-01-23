use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    if let Ok(value) = ResistorColor::from_int(value as u8) {
        format!("{:?}", value)
    } else {
        String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut ret = all::<ResistorColor>().collect::<Vec<_>>();
    ret.sort_by(|a, b| a.int_value().partial_cmp(&b.int_value()).unwrap());
    ret
}
