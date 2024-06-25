use std::fmt::{Debug, Formatter};

use serde::{Deserializer, Serialize, Serializer};

pub enum Type {
    Rectangle,
    Text,
    Excalidraw,
    Arrow,
}

impl Type {
    fn string(&self) -> &str {
        match self {
            Type::Rectangle => { "rectangle" }
            Type::Text => { "text" }
            Type::Excalidraw => { "excalidraw" }
            Type::Arrow => { "arrow" }
        }
    }
}

impl Debug for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string())
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.string())
    }
}
