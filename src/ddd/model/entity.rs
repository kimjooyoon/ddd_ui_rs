use crate::ddd::model::r#type::r#type::ModelType;
use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleData;

pub fn entity(data: RectangleData) -> Vec<Element> {
    let rect = ModelType::Entity.rectangle_element(data);
    let text =  ModelType::Entity.text_element();

    let (rect, text) = rect.bind(text);
    vec![Element::_RectangleElement(rect), Element::_TextElement(text)]
}