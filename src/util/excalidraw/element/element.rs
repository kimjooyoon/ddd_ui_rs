use std::io::SeekFrom;
use serde::Serialize;
use crate::util::excalidraw::element::arrow::ArrowElement;
use crate::util::excalidraw::element::bound_element::BoundElement;

use crate::util::excalidraw::element::rectangle::RectangleElement;
use crate::util::excalidraw::element::text::TextElement;
use crate::util::excalidraw::r#type::Type::Arrow;

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Element {
    _RectangleElement(RectangleElement),
    _TextElement(TextElement),
    _ArrowElement(ArrowElement),
}

impl Element {
    pub fn rectangle(&self) -> Result<&RectangleElement, String> {
        match self {
            Element::_RectangleElement(rect) => {
                Ok(rect)
            }
            _ => {
                Err(String::from("not rectangle"))
            }
        }
    }

    pub fn bind_arrow(self, id: String) -> Self {
        match self {
            Element::_RectangleElement(mut el) => {
                el.boundElements = el.boundElements.into_iter().chain(
                    vec!(BoundElement {
                        r#type: Arrow,
                        id,
                    })
                ).collect();
                Element::_RectangleElement(el)
            }
            _ => {
                self
            }
        }
    }
}