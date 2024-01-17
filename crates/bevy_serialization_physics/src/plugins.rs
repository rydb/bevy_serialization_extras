use bevy::prelude::*;
use bevy_obj::ObjPlugin;
use bevy_rapier3d::{geometry::{AsyncCollider, SolverGroups}, dynamics::{ImpulseJoint, RigidBody, AdditionalMassProperties}};
use bevy_serialization_core::plugins::{SerializeComponentFor, SerializeQueryFor, SerializeManyAsOneFor};

use crate::{wrappers::{link::{StructureFlag, LinkFlag, Linkage, JointFlag, LinkQuery}, colliders::ColliderFlag, rigidbodies::RigidBodyFlag, mass::MassFlag, solvergroupfilter::SolverGroupsFlag}, systems::{urdf_origin_shift, bind_joints_to_entities}, ui::{UtilitySelection, CachedUrdf, physics_widgets_window}, loaders::urdf_loader::{Urdf, UrdfLoaderPlugin}};

/// this is the plugin that containss all of the wrappers that physics should need to serialize(minus the backend)
/// (TODO): seperate urdfs from this into a seperate crate
pub struct PhysicsSerializationPlugin;


impl Plugin for PhysicsSerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(ObjPlugin)
        .add_plugins(UrdfLoaderPlugin)
        .insert_resource(UtilitySelection::default())
        .insert_resource(CachedUrdf::default())

        .register_type::<StructureFlag>()
        .register_type::<LinkFlag>()

        .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())
        .add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
        .add_plugins(SerializeComponentFor::<RigidBody, RigidBodyFlag>::default())
        .add_plugins(SerializeComponentFor::<AdditionalMassProperties, MassFlag>::default())
        .add_plugins(SerializeManyAsOneFor::<LinkQuery, Urdf>::default())
        .add_plugins(SerializeComponentFor::<SolverGroups, SolverGroupsFlag>::default())

        .add_systems(Update, physics_widgets_window)
        .add_systems(Update, urdf_origin_shift)
        .add_systems(PostUpdate, bind_joints_to_entities)
        
        ;
    }
}