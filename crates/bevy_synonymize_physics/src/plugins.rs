use crate::{
    prelude::{
        ColliderFlag, collisiongroupfilter::CollisionGroupsFlag, continous_collision::CcdFlag,
        friction::FrictionFlag, link::JointRecieverFlag,
    },
    systems::{generate_collider_from_children, generate_primitive_for_request},
    synonyms::{
        link::{JointFlag, LinkFlag},
        mass::MassFlag,
        rigidbodies::RigidBodyFlag,
        solvergroupfilter::SolverGroupsFlag,
    },
};

use bevy_app::prelude::*;
use bevy_synonymize::plugins::SynonymizeComponent;

/// This plugin is an addon for [`SerializationPlugin`] for physics.
pub struct SynonymizePhysicsPlugin;

impl Plugin for SynonymizePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // .register_type::<StructureFlag>()
            .register_type::<LinkFlag>()
            .register_type::<JointRecieverFlag>()
            .register_type::<CollisionGroupsFlag>()
            //.register_type::<AsyncColliderFlag>()
            .register_type::<FrictionFlag>()
            //.add_plugins(SynonymizeComponent::<AsyncColliderFlag>::default())
            .add_plugins(SynonymizeComponent::<RigidBodyFlag>::default())
            .add_plugins(SynonymizeComponent::<MassFlag>::default())
            .add_plugins(SynonymizeComponent::<SolverGroupsFlag>::default())
            .add_plugins(SynonymizeComponent::<CollisionGroupsFlag>::default())
            .add_plugins(SynonymizeComponent::<CcdFlag>::default())
            .add_plugins(SynonymizeComponent::<ColliderFlag>::default())
            //.add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
            .add_plugins(SynonymizeComponent::<JointFlag>::default())
            .add_systems(Update, generate_primitive_for_request)
            .add_systems(Update, generate_collider_from_children);
    }
}
