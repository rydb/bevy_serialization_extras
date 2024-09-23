use bevy_obj::ObjPlugin;
use bevy_rapier3d::{
    dynamics::{AdditionalMassProperties, ImpulseJoint, RigidBody},
    geometry::{AsyncCollider, SolverGroups},
};
use bevy_serialization_core::plugins::{SerializeComponentFor, SerializeQueryFor};

use crate::{
    systems::{bind_joints_to_entities, local_frame2_shift},
    wrappers::{
        colliders::ColliderFlag,
        link::{JointFlag, LinkFlag, Linkage, StructureFlag},
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
        app.add_plugins(ObjPlugin)
            .register_type::<StructureFlag>()
            .register_type::<LinkFlag>()
            .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())
            .add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
            .add_plugins(SerializeComponentFor::<RigidBody, RigidBodyFlag>::default())
            .add_plugins(SerializeComponentFor::<AdditionalMassProperties, MassFlag>::default())
            .add_plugins(SerializeComponentFor::<SolverGroups, SolverGroupsFlag>::default())
            // post processing
            .add_systems(Update, local_frame2_shift)
            .add_systems(PostUpdate, bind_joints_to_entities);
    }
}
