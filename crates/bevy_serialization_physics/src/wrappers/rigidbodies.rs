use bevy_rapier3d::prelude::RigidBody;
use strum_macros::EnumIter;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use super::{colliders::ColliderFlag, friction::FrictionFlag, mass::MassFlag};

#[derive(Component, Reflect, Clone, Default, EnumIter)]
#[reflect(Component)]
#[require(MassFlag)]
pub enum RigidBodyFlag {
    #[default]
    Fixed,
    Dynamic,
}

impl From<&RigidBodyFlag> for RigidBody {
    fn from(value: &RigidBodyFlag) -> Self {
        match value {
            RigidBodyFlag::Fixed => Self::Fixed,
            RigidBodyFlag::Dynamic => Self::Dynamic,
        }
    }
}
impl From<&RigidBody> for RigidBodyFlag {
    fn from(value: &RigidBody) -> Self {
        match *value {
            RigidBody::Fixed => Self::Fixed,
            RigidBody::Dynamic => Self::Dynamic,
            _ => panic!("Rigidbody serialization only implemented for fixed and dynamic. populate wrapper for more types")
        }
    }
}
