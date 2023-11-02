
use bevy::prelude::Component;
use urdf_rs::Link;

use super::{mesh::GeometryFlag, colliders::ColliderFlag, mass::MassFlag};
pub struct LinkStructure {
    pub name: String, 
}

#[derive(Component)]
pub struct LinkFlag {
    pub name: LinkStructure,
    pub inertial: MassFlag,
    pub visual: GeometryFlag,
    pub collision: ColliderFlag,
}

/// all of the things that compose a "Link"
type LinkAsTuple = (LinkStructure, MassFlag, GeometryFlag, ColliderFlag);

impl From<LinkAsTuple> for LinkFlag {
    fn from(value: LinkAsTuple) -> Self {
        Self {
            name: value.0,
            inertial: value.1,
            visual: value.2,
            collision: value.3,
        }
    }
}

impl From<LinkFlag> for LinkAsTuple {
    fn from(value: LinkFlag) -> Self {
        (
            value.name,
            value.inertial,
            value.visual,
            value.collision
        )
    }
}

// /// Urdf Link element
// /// See <http://wiki.ros.org/urdf/XML/link> for more detail.
// #[derive(Debug, YaDeserialize, YaSerialize, Clone)]
// pub struct Link {
//     #[yaserde(attribute)]
//     pub name: String,
//     pub inertial: Inertial,
//     pub visual: Vec<Visual>,
//     pub collision: Vec<Collision>,
// }