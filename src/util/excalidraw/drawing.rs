use serde::Serialize;
use serde_json;

use crate::util::excalidraw::element::element::Element;
use crate::util::excalidraw::r#type::Type;

#[derive(Debug, Serialize)]
pub struct AppState {
    theme: String,
    viewBackgroundColor: String,
    currentItemStrokeColor: String,
    currentItemBackgroundColor: String,
    currentItemFillStyle: String,
    currentItemStrokeWidth: i32,
    currentItemStrokeStyle: String,
    currentItemRoughness: i32,
    currentItemOpacity: i32,
    currentItemFontFamily: i32,
    currentItemFontSize: i32,
    currentItemTextAlign: String,
    currentItemStartArrowhead: Option<String>,
    currentItemEndArrowhead: Option<String>,
    scrollX: i32,
    scrollY: i32,
    zoom: Zoom,
    currentItemRoundness: String,
    gridSize: Option<String>,
    gridColor: GridColor,
    currentStrokeOptions: Option<String>,
    previousGridSize: Option<String>,
    frameRendering: FrameRendering,
    objectsSnapModeEnabled: bool,
}

#[derive(Debug, Serialize)]
pub struct Zoom {
    value: f32,
}

#[derive(Debug, Serialize)]
pub struct GridColor {
    Bold: String,
    Regular: String,
}

#[derive(Debug, Serialize)]
pub struct FrameRendering {
    enabled: bool,
    clip: bool,
    name: bool,
    outline: bool,
}

#[derive(Debug, Serialize)]
pub struct Drawing {
    r#type: Type,
    version: i8,
    source: String,
    elements: Vec<Element>,
    files: Vec<String>,
    appState: AppState,
}

impl Drawing {
    pub fn new(
        el: Vec<(Element)>
    ) -> Self {
        let app_state = AppState {
            theme: String::from("light"),
            viewBackgroundColor: String::from("#ffffff"),
            currentItemStrokeColor: String::from("#1e1e1e"),
            currentItemBackgroundColor: String::from("transparent"),
            currentItemFillStyle: String::from("solid"),
            currentItemStrokeWidth: 2,
            currentItemStrokeStyle: String::from("solid"),
            currentItemRoughness: 1,
            currentItemOpacity: 100,
            currentItemFontFamily: 1,
            currentItemFontSize: 20,
            currentItemTextAlign: String::from("left"),
            currentItemStartArrowhead: None,
            currentItemEndArrowhead: Some(String::from("arrow")),
            scrollX: 0,
            scrollY: 0,
            zoom: Zoom { value: 1.0 },
            currentItemRoundness: String::from("round"),
            gridSize: None,
            gridColor: GridColor {
                Bold: String::from("#cccccc"),
                Regular: String::from("#e5e5e5"),
            },
            currentStrokeOptions: None,
            previousGridSize: None,
            frameRendering: FrameRendering {
                enabled: true,
                clip: true,
                name: true,
                outline: true,
            },
            objectsSnapModeEnabled: false,
        };

        Drawing {
            r#type: Type::Excalidraw,
            version: 2,
            source: String::from("https://github.com/zsviczian/obsidian-excalidraw-plugin/releases/tag/2.2.5"),
            elements: el,
            files: vec![],
            appState: app_state,
        }
    }
}