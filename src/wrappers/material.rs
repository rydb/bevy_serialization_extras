use bevy::{prelude::*, ecs::query::WorldQuery};
use urdf_rs::Visual;



#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct MaterialFlag {
    pub color: Color
}

pub enum MaterialSource {
    Wrapper(MaterialFlag),
    File(MaterialFile),
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct MaterialFile {
    pub path: String, 
}



impl From<&MaterialFlag> for StandardMaterial {
    fn from(value: &MaterialFlag) -> Self {
        Self {
            base_color: value.color,
            ..default()
        }
    }
}

impl From<Vec<Visual>> for MaterialFlag {
    fn from(value: Vec<Visual>) -> Self {
        if let Some(visual) = value.first() { 
            if let Some(material) = &visual.material {
                if let Some(color) = &material.color {
                    let rgba = color.rgba.0;
                    Self {
                        color: Color::Rgba { red: rgba[0] as f32, green: rgba[1] as f32, blue: rgba[2] as f32, alpha: rgba[3] as f32 }
                    }
                } else {
                    Self::default()
                }
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
}

impl From<&StandardMaterial> for MaterialFlag {
    fn from(value: &StandardMaterial) -> Self {
        Self {
            color: value.base_color,
            ..default()
        }
    }
}

impl From<Color> for MaterialFlag {
    fn from(value: Color) -> Self {
        Self {
            color: value,
            ..default()
        }
    }
}