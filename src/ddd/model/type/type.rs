use crate::util::excalidraw::element::rectangle::{RectangleData, RectangleElement};
use crate::util::excalidraw::element::text::{TextElement, TextElementData};

pub enum ModelType {
    Aggregate,
    Event,
    Context,
    Entity,
    Vo,
}

impl ModelType {
    pub fn text(self) -> String {
        match self {
            ModelType::Aggregate => { String::from("Aggregate") }
            ModelType::Event => { String::from("Event") }
            ModelType::Context => { String::from("Context") }
            ModelType::Entity => { String::from("Entity") }
            ModelType::Vo => { String::from("VO") }
        }
    }

    pub fn rectangle_element(self, data: RectangleData) -> RectangleElement {
        let x_index = data.x_index;

        match self {
            ModelType::Aggregate => {
                RectangleElement::green(data)
            }
            ModelType::Event => {
                RectangleElement::yellow(data)
            }
            ModelType::Context => {
                RectangleElement::red(data)
            }
            ModelType::Entity => {
                RectangleElement::blue(data)
            }
            ModelType::Vo => {
                RectangleElement::black(data)
            }
        }
    }

    pub fn text_element(self) -> TextElement {
        match self {
            ModelType::Aggregate => {
                TextElement::green(TextElementData {
                    text: self.text(),
                })
            }
            ModelType::Event => {
                TextElement::yellow(TextElementData {
                    text:  self.text(),
                })
            }
            ModelType::Context => {
                TextElement::red(TextElementData {
                    text: self.text(),
                })
            }
            ModelType::Entity => {
                TextElement::blue(TextElementData {
                    text: self.text(),
                })
            }
            ModelType::Vo => {
                TextElement::black(TextElementData {
                    text: self.text(),
                })
            }
        }
    }
}
