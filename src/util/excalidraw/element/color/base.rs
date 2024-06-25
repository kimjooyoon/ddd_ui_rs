use std::fmt::{Debug, Formatter};
use serde::{Serialize, Serializer};

pub enum Color {
    Black,
    Yellow,
    Green,
    Red,
    Blue,

    Transparent
}

impl Color {
    pub fn hex(&self) -> &str {
        match self {
            Color::Black => { "#1e1e1e" }
            Color::Yellow => { "#f08c00" }
            Color::Green => { "#2f9e44" }
            Color::Red => { "#e03131" }
            Color::Blue => { "#1971c2" }
            Color::Transparent => { "transparent" }
        }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.hex())
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.hex())
    }
}

