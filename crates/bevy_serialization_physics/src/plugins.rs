use bevy_serialization_core::prelude::SerializeComponentFor;

use crate::{
    prelude::{
        ColliderFlag, collisiongroupfilter::CollisionGroupsFlag, continous_collision::CcdFlag,
        friction::FrictionFlag, link::JointRecieverFlag,
    },
    systems::{generate_collider_from_children, generate_primitive_for_request},
    wrappers::{
        link::{JointFlag, LinkFlag},
        mass::MassFlag,
        rigidbodies::RigidBodyFlag,
        solvergroupfilter::SolverGroupsFlag,
    },
};

use bevy_app::prelude::*;

/// This plugin is an addon for [`SerializationPlugin`] for physics.
pub struct SerializationPhysicsPlugin;

impl Plugin for SerializationPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        // .register_type::<StructureFlag>()
            .register_type::<LinkFlag>()
            .register_type::<JointRecieverFlag>()
            .register_type::<CollisionGroupsFlag>()
            //.register_type::<AsyncColliderFlag>()
            .register_type::<FrictionFlag>()
            //.add_plugins(SerializeComponentFor::<AsyncColliderFlag>::default())
            .add_plugins(SerializeComponentFor::<RigidBodyFlag>::default())
            .add_plugins(SerializeComponentFor::<MassFlag>::default())
            .add_plugins(SerializeComponentFor::<SolverGroupsFlag>::default())
            .add_plugins(SerializeComponentFor::<CollisionGroupsFlag>::default())
            .add_plugins(SerializeComponentFor::<CcdFlag>::default())
            .add_plugins(SerializeComponentFor::<ColliderFlag>::default())
            //.add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
            .add_plugins(SerializeComponentFor::<JointFlag>::default())
            .add_systems(Update, generate_primitive_for_request)
            .add_systems(Update, generate_collider_from_children);
    }
}
