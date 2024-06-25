use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::util::excalidraw::element::bound_element::BoundElement;
use crate::util::excalidraw::element::color::base::Color;
use crate::util::excalidraw::element::color::base::Color::{Black, Blue, Green, Red, Yellow};
use crate::util::excalidraw::element::id::generate_container_id;
use crate::util::excalidraw::element::style::fill::Fill;
use crate::util::excalidraw::element::style::fill::Fill::Solid;
use crate::util::excalidraw::element::text::TextElement;
use crate::util::excalidraw::r#type::Type;
use crate::util::excalidraw::r#type::Type::{Rectangle, Text};

#[derive(Debug, Serialize)]
pub struct RectangleElement {
    pub id: String,
    r#type: Type,
    version: i32,
    versionNonce: i64,
    index: String,
    isDeleted: bool,
    fillStyle: Fill,
    strokeStyle: Fill,
    strokeWidth: i32,
    roughness: i32,
    opacity: i32,
    angle: i32,
    pub(crate) x: f64,
    pub(crate) y: f64,
    strokeColor: Color,
    backgroundColor: Color,
    pub(crate) width: f32,
    pub(crate) height: f32,
    seed: i32,
    groupIds: Vec<String>,
    frameId: Option<String>,
    roundness: Option<String>,
    pub(crate) boundElements: Vec<BoundElement>,
    updated: i64,
    link: Option<String>,
    locked: bool,
}

pub struct RectangleData {
    pub x_index: i16,
    pub y_index: i16,
}

impl RectangleElement {
    pub fn black(rectangle_data: RectangleData) -> Self { Self::new(Black, rectangle_data) }
    pub fn yellow(rectangle_data: RectangleData) -> Self { Self::new(Yellow, rectangle_data) }
    pub fn green(rectangle_data: RectangleData) -> Self { Self::new(Green, rectangle_data) }
    pub fn red(rectangle_data: RectangleData) -> Self { Self::new(Red, rectangle_data) }
    pub fn blue(rectangle_data: RectangleData) -> Self { Self::new(Blue, rectangle_data) }

    fn new(color: Color, data: RectangleData) -> Self {
        let id = generate_container_id();
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let version_nonce: i64 = since_the_epoch.as_secs() as i64 ^ 248376931;

        let x = 699.2920998251401 + (300 * data.x_index) as f64;

        RectangleElement {
            id,
            r#type: Rectangle,
            version: 192,
            versionNonce: version_nonce,
            index: "a1".to_string(),
            isDeleted: false,
            fillStyle: Solid,
            strokeStyle: Solid,
            strokeWidth: 2,
            roughness: 1,
            opacity: 100,
            angle: 0,
            x,
            y: 232.48399107064995,
            strokeColor: color,
            backgroundColor: Color::Transparent,
            width: 136.08398437499997,
            height: 83.26953125,
            seed: 1,
            groupIds: vec![],
            frameId: None,
            roundness: None,
            boundElements: vec![],
            updated: since_the_epoch.as_millis() as i64,
            link: None,
            locked: false,
        }
    }

    pub fn bind(mut self, text_element: TextElement) -> (RectangleElement, TextElement) {
        let text_element = text_element.bind(&self);

        self.boundElements = vec![
            BoundElement {
                id: text_element.id.clone(),
                r#type: Text,
            },
        ];
        (self, text_element)
    }
}
