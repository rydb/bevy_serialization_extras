use bevy::prelude::*;
use bevy_rapier3d::prelude::RigidBody;

#[derive(Component, Reflect, Clone, Default)]
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
