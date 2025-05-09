//! Rust wrapper around the [`KHR_IMPLICIT_SHAPES`] section of the gltf physics spec proposal.
//! https://github.com/eoineoineoin/glTF_Physics/tree/master/extensions/2.0/Khronos/KHR_implicit_shapes

use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

pub const KHR_IMPLICIT_SHAPES: &'static str = "khr_implicit_shapes";

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct KHRImplicitShapesMap {
    pub shapes: Vec<Shape>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Shape {
    Box(BoxShape),
    Cylinder(CylinderShape),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoxShape {
    #[serde(rename = "box")]
    pub size: BoxData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CylinderShape {
    #[serde(rename = "cylinder")]
    pub dimensions: CylinderData,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BoxData {
    pub size: [f64; 3],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CylinderData {
    pub height: f64,
    pub radius_bottom: f64,
    pub radius_top: f64,
}

impl<'de> Deserialize<'de> for Shape {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let shape_type = value
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("Missing shape type"))?;

        match shape_type {
            "box" => Ok(Shape::Box(BoxShape {
                size: BoxData {
                    size: serde_json::from_value(value["box"]["size"].clone())
                        .map_err(serde::de::Error::custom)?,
                },
            })),
            "cylinder" => Ok(Shape::Cylinder(CylinderShape {
                dimensions: CylinderData {
                    height: value["cylinder"]["height"]
                        .as_f64()
                        .ok_or_else(|| serde::de::Error::custom("Missing cylinder height"))?,
                    radius_bottom: value["cylinder"]["radiusBottom"]
                        .as_f64()
                        .ok_or_else(|| serde::de::Error::custom("Missing radiusBottom"))?,
                    radius_top: value["cylinder"]["radiusTop"]
                        .as_f64()
                        .ok_or_else(|| serde::de::Error::custom("Missing radiusTop"))?,
                },
            })),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown shape type: {}",
                shape_type
            ))),
        }
    }
}
