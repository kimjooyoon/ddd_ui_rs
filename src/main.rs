use std::ops::{Deref, Index};
use crate::ddd::model::object::aggregate::Aggregate;
use crate::ddd::model::object::context::Context;
use crate::ddd::model::object::entity::Entity;

use crate::ddd::model::r#trait::VecElement;
use crate::util::excalidraw::drawing::Drawing;
use crate::util::excalidraw::element::arrow::ArrowElement;
use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleData;
use crate::util::fileio::json::write_json;

mod util;
mod ddd;

macro_rules! chaining {
    ($el1:expr, $($el:expr), *) => {
        {
            $el1.into_iter()$(.chain($el.into_iter()))*.collect()
        }
    };
}

struct Counter {
    value: i16,
}

impl Counter {
    fn new() -> Self {
        Counter { value: -1 }
    }

    fn index(&mut self) -> i16 {
        self.value = self.value + 1;
        self.value
    }
}

fn main() {
    let mut counter = Counter::new();

    // aggregate -> entity
    let el1 = Aggregate::new(RectangleData { x_index: counter.index(), y_index: 0 }).elements();
    let el2 = Entity::new(RectangleData { x_index: counter.index(), y_index: 0 }).elements();
    let arrow = ArrowElement::new_blue();
    let arrow_id = arrow.id.clone();

    let el1_rect = el1.index(0).rectangle().unwrap();
    let el2_rect = el2.index(0).rectangle().unwrap();

    let arrow = vec!(Element::_ArrowElement(arrow.bind(el1_rect, el2_rect)));

    let el1: Vec<Element> = el1.into_iter().map(|x| x.bind_arrow(arrow_id.clone())).collect();
    let el2: Vec<Element> = el2.into_iter().map(|x| x.bind_arrow(arrow_id.clone())).collect();

    // entity -> context
    let context = Context::new(RectangleData { x_index: counter.index(), y_index: 0 }).elements();
    let arrow2 = ArrowElement::new_red();
    let arrow2_id = arrow2.id.clone();

    let el2_rect = el2.index(0).rectangle().unwrap();
    let context_rect = context.index(0).rectangle().unwrap();

    let arrow2 = vec!(Element::_ArrowElement(arrow2.bind(el2_rect, context_rect)));

    let el2: Vec<Element> = el2.into_iter().map(|x| x.bind_arrow(arrow2_id.clone())).collect();
    let context: Vec<Element> = context.into_iter().map(|x| x.bind_arrow(arrow2_id.clone())).collect();

    let i = chaining!(el1, el2, arrow, context, arrow2);

    let draw = Drawing::new(i);
    let json_string = serde_json::to_string_pretty(&draw).unwrap();

    write_json(json_string).expect("panic: failure to out json");
}
