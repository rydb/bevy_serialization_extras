use bevy::prelude::*;
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{geometry::{AsyncCollider, SolverGroups}, dynamics::{ImpulseJoint, RigidBody, AdditionalMassProperties}};
use bevy_serialization_core::plugins::{SerializeComponentFor, SerializeQueryFor, SerializeManyAsOneFor};

use crate::{wrappers::{link::{StructureFlag, LinkFlag, Linkage, JointFlag}, colliders::ColliderFlag, rigidbodies::RigidBodyFlag, mass::MassFlag, solvergroupfilter::SolverGroupsFlag}, systems::{bind_joints_to_entities, local_frame2_shift}};



/// this is the plugin that containss all of the wrappers that physics should need to serialize(minus the backend)
pub struct PhysicsSerializationPlugin;


impl Plugin for PhysicsSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ObjPlugin)


        .register_type::<StructureFlag>()
        .register_type::<LinkFlag>()

        .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())
        .add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
        .add_plugins(SerializeComponentFor::<RigidBody, RigidBodyFlag>::default())
        .add_plugins(SerializeComponentFor::<AdditionalMassProperties, MassFlag>::default())
        .add_plugins(SerializeComponentFor::<SolverGroups, SolverGroupsFlag>::default())

  
        // post processing
        .add_systems(Update, local_frame2_shift)
        .add_systems(PostUpdate, bind_joints_to_entities)
        
        ;
    }
}