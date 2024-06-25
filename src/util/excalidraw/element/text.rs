use serde::Serialize;
use crate::util::excalidraw::element::color::base::Color;
use crate::util::excalidraw::element::color::base::Color::{Black, Blue, Green, Red, Yellow};
use crate::util::excalidraw::element::id::generate_element_id;
use crate::util::excalidraw::element::rectangle::RectangleElement;
use crate::util::excalidraw::element::style::fill::Fill;
use crate::util::excalidraw::element::style::fill::Fill::Solid;
use crate::util::excalidraw::r#type::Type;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::util::excalidraw::element::bound_element::BoundElement;

#[derive(Debug, Serialize)]
pub struct TextElement {
    pub id: String,
    r#type: Type,
    version: i32,
    versionNonce: i64,
    index: String,
    isDeleted: bool,
    fillStyle: Fill,
    strokeWidth: i32,
    strokeStyle: Fill,
    roughness: i32,
    opacity: i32,
    angle: i32,
    x: f64,
    y: f64,
    strokeColor: Color,
    backgroundColor: Color,
    width: f32,
    height: f32,
    seed: i32,
    groupIds: Vec<String>,
    frameId: Option<String>,
    roundness: Option<String>,
    boundElements: Vec<BoundElement>,
    updated: i64,
    link: Option<String>,
    locked: bool,
    fontSize: i32,
    fontFamily: i32,
    text: String,
    rawText: String,
    textAlign: String,
    verticalAlign: String,
    containerId: String,
    originalText: String,
    autoResize: bool,
    lineHeight: f64,
}

pub struct TextElementData {
    pub text: String,
}

impl TextElement {
    pub fn black(data :TextElementData) -> Self { Self::new(data, Black) }
    pub fn yellow(data :TextElementData) -> Self { Self::new(data, Yellow) }
    pub fn green(data :TextElementData) -> Self { Self::new(data, Green) }
    pub fn red(data :TextElementData) -> Self { Self::new(data, Red) }
    pub fn blue(data :TextElementData) -> Self { Self::new(data, Blue) }

    fn new(data :TextElementData, color: Color) -> Self {
        let id = generate_element_id();
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let version_nonce: i64 = since_the_epoch.as_secs() as i64 ^ 1629517;

        TextElement {
            id,
            r#type: Type::Text,
            version: 137,
            versionNonce: version_nonce,
            index: "a2".to_string(),
            isDeleted: false,
            fillStyle: Solid,
            strokeWidth: 2,
            strokeStyle: Solid,
            roughness: 1,
            opacity: 100,
            angle: 0,
            x: 754.9040993368588,
            y: 261.61875669564995,
            strokeColor: color,
            backgroundColor: Color::Transparent,
            width: 24.8599853515625,
            height: 25.0,
            seed: 950972365,
            groupIds: vec![],
            frameId: None,
            roundness: None,
            boundElements: vec![],
            updated: since_the_epoch.as_millis() as i64,
            link: None,
            locked: false,
            fontSize: 20,
            fontFamily: 1,
            text: data.text.clone(),
            rawText: data.text.clone(),
            textAlign: "center".to_string(),
            verticalAlign: "middle".to_string(),
            containerId: String::new(),
            originalText: data.text.clone(),
            autoResize: true,
            lineHeight: 1.25,
        }
    }

    pub fn bind(mut self, rectangle_element: &RectangleElement) -> Self {
        self.containerId = rectangle_element.id.clone();
        self.x = rectangle_element.x.clone();
        self
    }
}

