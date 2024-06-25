use crate::ddd::model::r#trait::VecElement;
use crate::ddd::model::r#type::r#type::ModelType;
use crate::generate_model;
use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleData;

generate_model!(
    struct Vo {
        data: RectangleData
    }
);
