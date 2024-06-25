use std::ops::{Deref, Index};

use crate::ddd::model::aggregate::Aggregate;
use crate::ddd::model::entity::entity_element;
use crate::ddd::model::r#trait::VecElement;
use crate::util::excalidraw::drawing::Drawing;
use crate::util::excalidraw::element::arrow::ArrowElement;
use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::element::rectangle::RectangleData;

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

    /*
    let elements1 = vo(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements2 = vo(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements3 = vo(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements4 = entity(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements5 = aggregate(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements6 = context(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements7 = entity(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements8 = event(RectangleData { x_index: counter.index(), y_index: 0 });
    let elements9 = vo(RectangleData { x_index: counter.index(), y_index: 0 });

    let arrow8and9 = ArrowElement::new_black();
    let arrow8and9 = vec!(Element::_ArrowElement(arrow8and9.bind(elements8.index(0).rectangle().unwrap(), elements9.index(0).rectangle().unwrap())));

    let i = chaining!(
        elements1, elements2, elements3, elements4,
    elements5, elements6, elements7, elements8, elements9,
        arrow8and9
    );
     */
    let aggregate= Aggregate::new(RectangleData { x_index: counter.index(), y_index: 0 });
    let el1 = aggregate.aggregate_element();
    let el2 = entity_element(RectangleData { x_index: counter.index(), y_index: 0 });
    let arrow = ArrowElement::new_blue();
    let arrow_id = arrow.id.clone();

    let el1_rect = el1.index(0).rectangle().unwrap();
    let el2_rect = el2.index(0).rectangle().unwrap();

    let arrow = vec!(Element::_ArrowElement(arrow.bind(el1_rect, el2_rect)));

    let el1: Vec<Element> = el1.into_iter().map(|x| x.bind_arrow(arrow_id.clone())).collect();
    let el2: Vec<Element> = el2.into_iter().map(|x| x.bind_arrow(arrow_id.clone())).collect();

    let i = chaining!(el1, el2, arrow);

    // let i = elements1.into_iter().chain(elements2.into_iter());
    let draw = Drawing::new(i);
    let json_string = serde_json::to_string_pretty(&draw).unwrap();
    println!("{}", json_string)
}
