use crate::wrappers::material::MaterialFlag;
use crate::wrappers::mesh::GeometryFlag;
use bevy::prelude::*;
/// Wrapper bundle made to tie together everything that composes a "model", in a serializable format
/// !!! THIS WILL LIKELY BE REFACTORED AWAY WITH ASSETSV2 IN 0.12!!!
#[derive(Bundle, Default)]
pub struct ModelBundle {
    pub mesh: GeometryFlag,
    pub material: MaterialFlag,
    pub visibility: Visibility,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}
