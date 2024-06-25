use crate::util::excalidraw::element::rectangle::RectangleData;
use crate::ddd::model::r#trait::VecElement;
use crate::util::excalidraw::element::element::Element;
use crate::ddd::model::r#type::r#type::ModelType;

#[macro_export]
macro_rules! generate_model {
    (
        struct $name:ident {
            $($field_name:ident: $field_type:ty),*
        }
    ) => {
        pub struct $name {
            $($field_name: $field_type),*
        }

        impl $name {
            pub fn new($($field_name: $field_type),*) -> Self {
                $name {
                    $($field_name),*
                }
            }
        }

        impl VecElement for $name {
            fn elements(self) -> Vec<Element> {
                let rect = ModelType::$name.rectangle_element(self.data);
                let text = ModelType::$name.text_element();

                let (rect, text) = rect.bind(text);
                vec![Element::_RectangleElement(rect), Element::_TextElement(text)]
            }
        }
    };
}

generate_model!(struct Aggregate {
    data: RectangleData
});