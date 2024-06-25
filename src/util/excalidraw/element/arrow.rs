use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

// Your custom modules
use crate::util::excalidraw::element::bound_element::BoundElement;
use crate::util::excalidraw::element::color::base::Color;
use crate::util::excalidraw::element::color::base::Color::{Black, Blue, Green, Red, Yellow};
use crate::util::excalidraw::element::id::generate_container_id;
use crate::util::excalidraw::element::rectangle::RectangleElement;
use crate::util::excalidraw::element::style::fill::Fill;
use crate::util::excalidraw::element::style::fill::Fill::Solid;
use crate::util::excalidraw::r#type::Type;
use crate::util::excalidraw::r#type::Type::Arrow;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrowElement {
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
    angle: f64,
    pub(crate) x: f64,
    y: f64,
    strokeColor: Color,
    backgroundColor: Color,
    width: f64,
    height: f64,
    seed: i64,
    groupIds: Vec<String>,
    frameId: Option<String>,
    roundness: Option<Roundness>,
    boundElements: Vec<BoundElement>,
    updated: i64,
    link: Option<String>,
    locked: bool,
    startBinding: Option<Binding>,
    endBinding: Option<Binding>,
    lastCommittedPoint: Option<String>,
    startArrowhead: Option<String>,
    endArrowhead: Option<String>,
    points: Vec<(f64, f64)>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Roundness {
    r#type: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Binding {
    elementId: String,
    gap: f64,
    focus: f64,
}

impl ArrowElement {
    pub fn new_black() -> Self { Self::new(Black) }
    pub fn new_yellow() -> Self { Self::new(Yellow) }
    pub fn new_green() -> Self { Self::new(Green) }
    pub fn new_red() -> Self { Self::new(Red) }
    pub fn new_blue() -> Self { Self::new(Blue) }

    fn new(color: Color) -> Self {
        let id = generate_container_id();
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let version_nonce: i64 = since_the_epoch.as_secs() as i64 ^ 248376931;

        ArrowElement {
            id,
            r#type: Arrow,
            version: 184,
            versionNonce: version_nonce,
            index: "aK".to_string(),
            isDeleted: false,
            fillStyle: Solid,
            strokeStyle: Solid,
            strokeWidth: 2,
            roughness: 1,
            opacity: 100,
            angle: 0.0,
            x: 682.6653920357953,
            y: 106.89439287043861,
            strokeColor: color,
            backgroundColor: Color::Transparent,
            width: 241.19841295821288,
            height: 18.697982705551013,
            seed: 1686108377,
            groupIds: vec![],
            frameId: None,
            roundness: Some(Roundness { r#type: 2 }),
            boundElements: vec![],
            updated: since_the_epoch.as_millis() as i64,
            link: None,
            locked: false,
            startBinding: Some(Binding {
                elementId: "qr1gH07y56HQ0XWwgdEr8g".to_string(),
                gap: 4.897977105263124,
                focus: 0.063913614377779,
            }),
            endBinding: Some(Binding {
                elementId: "X2JOWYORAW3tQHKLp8aifu".to_string(),
                gap: 1.0,
                focus: -0.0013036702246972418,
            }),
            lastCommittedPoint: None,
            startArrowhead: None,
            endArrowhead: Some("arrow".to_string()),
            points: vec![
                (0.0, 0.0),
                (163.916015625, 0.0),
            ],
        }
    }

    pub fn bind(mut self, left_rectangle: &RectangleElement, right_rectangle: &RectangleElement) -> ArrowElement {
        let start_x = left_rectangle.x.clone() + left_rectangle.width.clone() as f64;
        let start_y = left_rectangle.y.clone() + (left_rectangle.height.clone() as f64 / 2.0);

        let width = right_rectangle.x.clone();

        let left_id = left_rectangle.id.clone();
        let right_id = right_rectangle.id.clone();

        let start_binding = Some(Binding{
            elementId: left_id,
            gap: 1.0,
            focus: 0.01,
        });

        let end_binding = Some(Binding{
            elementId: right_id,
            gap: 1.0,
            focus: 0.01,
        });

        self.x = start_x;
        self.y = start_y;
        self.width = width;

        self.startBinding = start_binding;
        self.endBinding = end_binding;

        self.boundElements = vec![];
        (self)
    }
}

