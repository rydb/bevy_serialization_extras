use bevy::prelude::*;

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct MaterialFlag {
    color: Color
}

impl From<&MaterialFlag> for StandardMaterial {
    fn from(value: &MaterialFlag) -> Self {
        Self {
            base_color: value.color,
            ..default()
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