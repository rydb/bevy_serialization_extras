
use bevy_serialization_core::prelude::SerializeComponentFor;

use crate::{
    prelude::{
        collisiongroupfilter::CollisionGroupsFlag, continous_collision::CcdFlag,
        friction::FrictionFlag, link::JointRecieverFlag, AsyncColliderFlag, ColliderFlag,
    },
    wrappers::{
        link::{JointFlag, LinkFlag, StructureFlag},
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
        app.register_type::<StructureFlag>()
            .register_type::<LinkFlag>()
            .register_type::<JointRecieverFlag>()
            .register_type::<CollisionGroupsFlag>()
            .register_type::<AsyncColliderFlag>()
            .register_type::<FrictionFlag>()
            .add_plugins(SerializeComponentFor::<AsyncColliderFlag>::default())
            .add_plugins(SerializeComponentFor::<RigidBodyFlag>::default())
            .add_plugins(SerializeComponentFor::<MassFlag>::default())
            .add_plugins(SerializeComponentFor::<SolverGroupsFlag>::default())
            .add_plugins(SerializeComponentFor::<CollisionGroupsFlag>::default())
            .add_plugins(SerializeComponentFor::<CcdFlag>::default())
            .add_plugins(SerializeComponentFor::<ColliderFlag>::default())
            //.add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
            .add_plugins(SerializeComponentFor::<JointFlag>::default())
            ;
            // post processing
            //.add_systems(Update, local_frame2_shift)
    }
}
