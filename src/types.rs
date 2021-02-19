use crate::common::*;

/// Correspond to <pose> in annotation XML.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "pose")]
pub enum Pose {
    Frontal,
    Left,
    Rear,
    Right,
    Unspecified,
}

/// Correspond to <bndbox> in annotation XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "bndbox")]
pub struct BndBox {
    pub xmin: R64,
    pub ymin: R64,
    pub xmax: R64,
    pub ymax: R64,
}

/// Correspond to <size> in annotation XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "size")]
pub struct Size {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

/// Correspond to <size> in annotation XML.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "point")]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

/// Correspond to <weather> in annotation XML.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "weather")]
pub enum Weather {
    Cloudy,
    Overexposure,
    Rain,
    Shadow,
    Sun,
}

/// Correspond to <time> in annotation XML.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "time")]
pub enum Time {
    Day,
    Dusk,
    Night,
}

/// Correspond to <road> in annotation XML.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "road")]
pub enum Road {
    Alley,
    ExpressWay,
    General,
    Highway,
}

/// Correspond to <object> in annotation XML.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "object")]
pub struct Object {
    pub name: String,
    pub pose: Pose,
    pub truncated: Option<bool>,
    pub difficult: Option<bool>,
    pub bndbox: BndBox,
    pub weather: Weather,
    pub time: Time,
    pub road: Road,
}

/// Correspond to <part> in annotation XML.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "part")]
pub struct Part {
    pub name: String,
    pub bndbox: BndBox,
}

/// Correspond to <source> in annotation XML.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "source")]
pub struct Source {
    pub database: String,
    pub annotation: String,
    pub image: String,
}

/// Parsed annotation XML.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename = "annotation")]
pub struct Annotation {
    pub filename: String,
    pub folder: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub object: Vec<Object>,
    pub segmented: bool,
    pub size: Size,
    pub source: Source,
}
