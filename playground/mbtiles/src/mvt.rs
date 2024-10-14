use std::fmt::{Display, Formatter};
use geozero::mvt::tile::Value;

#[derive(Debug)]
pub struct TmpValue {
    string_value: Option<String>,
    float_value: Option<f32>,
    double_value: Option<f64>,
    int_value: Option<i64>,
    uint_value: Option<u64>,
    sint_value: Option<i64>,        // TODO: Ask, what sint is?
    bool_value: Option<bool>,
}
impl From<&Value> for TmpValue {
    fn from(value: &Value) -> Self {
        Self {
            string_value: value.string_value.clone(),
            float_value: value.float_value.clone(),
            double_value: value.double_value.clone(),
            int_value: value.int_value.clone(),
            uint_value: value.uint_value.clone(),
            sint_value: value.sint_value.clone(),
            bool_value: value.bool_value.clone(),
        }
    }
}

#[derive(Debug)]
pub struct TmpFeature {
    pub id: Option<u64>,
    pub tags: Vec<u32>,
    pub r#type: Option<TmpGeomType>,
    pub geometry: Vec<u32>,
}

#[derive(Debug)]
#[repr(i32)]
pub enum TmpGeomType {
    Unknown = 0,
    Point = 1,
    Linestring = 2,
    Polygon = 3,
}
impl TmpGeomType {
    pub fn from_int(value: i32) -> Self {
        match value {
            1 => Self::Point,
            2 => Self::Linestring,
            3 => Self::Polygon,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
pub struct TmpLayer {
    pub version: u32,
    pub name: String,
    pub features: Vec<TmpFeature>,
    pub keys: Vec<String>,
    pub values: Vec<TmpValue>,
    pub extent: Option<u32>,
}

#[derive(Debug)]
pub struct TmpTile {
    pub layers: Vec<TmpLayer>,
}
