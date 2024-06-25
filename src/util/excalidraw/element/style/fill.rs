use std::fmt::{Debug, Formatter};
use serde::{Serialize, Serializer};

pub enum Fill{
    Solid,
}

impl Fill {
    fn string(&self) -> &str {
        match self {
            Fill::Solid => { "solid" }
        }
    }
}

impl Debug for Fill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.string())
    }
}

impl Serialize for Fill {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_str(self.string())
    }
}

