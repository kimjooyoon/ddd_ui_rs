use crate::util::excalidraw::element::element::Element;

pub trait VecElement {
    fn elements(self) -> Vec<Element>;
}
