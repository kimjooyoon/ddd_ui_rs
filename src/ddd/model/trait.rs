use crate::util::excalidraw::element::element::Element;

pub trait VecElement {
    fn aggregate_element(self) -> Vec<Element>;
}
