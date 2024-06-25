use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleElement;
use crate::util::excalidraw::element::text::TextElement;

pub fn bind_element(
    rect: RectangleElement,
    text: TextElement,
) -> Vec<Element> {
    let (rect, text) = rect.bind(text);
    vec![Element::_RectangleElement(rect), Element::_TextElement(text)]
}