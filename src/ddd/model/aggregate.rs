use crate::ddd::model::r#trait::VecElement;
use crate::ddd::model::r#type::r#type::ModelType;
use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleData;

pub struct Aggregate {
    data: RectangleData,
}

impl Aggregate {
    pub fn new(data: RectangleData) -> Self {
        Aggregate {
            data,
        }
    }
}

impl VecElement for Aggregate {
    fn aggregate_element(self) -> Vec<Element> {
        let rect = ModelType::Aggregate.rectangle_element(self.data);
        let text = ModelType::Aggregate.text_element();

        let (rect, text) = rect.bind(text);
        vec![Element::_RectangleElement(rect), Element::_TextElement(text)]
    }
}
