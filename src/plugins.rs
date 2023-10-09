
use bevy_rapier3d::prelude::{AsyncCollider, RigidBody};
use moonshine_save::{
    prelude::{SavePlugin, LoadPlugin, load_from_file, LoadSet}, save::{SaveSet, save_default},
    //save::*,
};
use bevy::{prelude::*, core_pipeline::core_3d::Camera3dDepthTextureUsage};
//use crate::urdf::urdf_loader::BevyRobot;
use bevy_component_extras::components::*;
use rapier3d::prelude::ColliderFlags;
use crate::physics::{colliders::ColliderFlag, material::MaterialFlag, mesh::MeshPrimitive};
use crate::physics::mesh::GeometryFlag;
use crate::physics::rigidbodies::RigidBodyFlag;
// use bevy_rapier3d::dynamics::rigid_body::RigidBody;
// use bevy_rapier3d::geometry::collider::Friction;
// use bevy_rapier3d::geometry::collider::SolverGroups;
// use bevy_rapier3d::geometry::collider::CollisionGroups;
//use moonshine_save::save::Save;


use super::systems::*;
use super::components::*;
/// marks component as a valid candidate for serialization. 
// #[derive(Component)]
// pub struct Serializable;
use moonshine_save::prelude::Save;
const SAVE_PATH: &str = "cube.ron";

/// plugin that manages serialization of bevy things.
/// if it exists, and you want it to serialize. Ensure its: 
/// 
/// 1. its in the type registry
/// 2. If it can be Serialized + Deserialized, it has a wrapper type which implements [`ECSSerialize`]
/// for two way serializaiton
/// 3. if it can only be Deserialized(things that rely on Handle<T> for example), it implements
/// [`ECSDeserialize`]

pub struct SerializationSystems;

impl Plugin for SerializationSystems {
    fn build(&self, app: &mut App) {
        app
        // All wrapper structs that detect normally unserializable things, and make them serializable
        .add_systems(Update,
            (
                serialize_for::<AsyncCollider, ColliderFlag>,
                serialize_for::<RigidBody, RigidBodyFlag>,
                try_serialize_asset_for::<StandardMaterial, MaterialFlag>,
                //try_serialize_asset_for::<Mesh, MeshFlag>,
            ).before(SaveSet::Save)
        )
        .add_systems(Update,
            (
                deserialize_for::<ColliderFlag, AsyncCollider>,
                deserialize_for::<RigidBodyFlag, RigidBody>,
                deserialize_asset_for::<MaterialFlag, StandardMaterial>,
                deserialize_wrapper_for::<GeometryFlag, Mesh>,
            )//.after(LoadSet::PostLoad)
        )
        ;
    }
}

/// plugin that adds systems/plugins for serialization. 
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (SavePlugin, LoadPlugin, SerializationSystems)
        )
        //.register_type::<ModelFlag>()
        //.register_type::<Geometry>()
        //.register_type::<MeshPrimitive>()
        //.register_type::<SerializeType>()
        .register_type::<Option<Entity>>()
        .register_type::<[f32; 3]>()
        .register_type::<Option<Handle<Image>>>()
        .register_type::<AlphaMode>()
        .register_type::<ParallaxMappingMethod>()
        .register_type::<Camera3dDepthTextureUsage>()
        .register_type::<bevy_mod_raycast::RaycastMethod>()
        .register_type::<bevy_mod_raycast::system_param::RaycastVisibility>()
        .register_type::<Physics>()
        .register_type::<Debug>()
        .register_type::<Viewer>()
        .register_type::<Selectable>()
        .register_type::<ColliderFlag>()
        .register_type::<MaterialFlag>()
        .register_type::<RigidBodyFlag>()
        .register_type::<MeshPrimitive>()
        // .register_type::<Friction>()
        // .register_type::<SolverGroups>()
        // .register_type::<CollisionGroups>()
        // .register_type::<Save>()
        .register_type::<GeometryFlag>()
        // .register_type::<AdditionalMassProperties>()
        // .register_type::<Ccd>()
        // .register_type::<AsyncCollider>()
        //.register_type::<RigidBodyPhysicsFlag>()
        //.register_type::<Save>()
        .add_systems(PreUpdate, 
            save_default()
            .exclude_component::<ComputedVisibility>()
            .into_file("cube.ron")
            .run_if(check_for_save_keypress)
        )
        //.add_systems(Update, save_into_file(SAVE_PATH).run_if(check_for_save_keypress))
        .add_systems(Update, add_computed_visiblity.after(LoadSet::PostLoad))
        .add_systems(Update, load_from_file(SAVE_PATH).run_if(check_for_load_keypress))
        .add_systems(PostStartup, list_unserializable)
        //.add_systems(Update, spawn_unspawned_models)
        ;    
    }
}