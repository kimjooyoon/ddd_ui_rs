use serde::Serialize;
use crate::util::excalidraw::r#type::Type;

#[derive(Debug, Serialize)]
pub struct BoundElement {
    pub r#type: Type,
    pub id: String
}

