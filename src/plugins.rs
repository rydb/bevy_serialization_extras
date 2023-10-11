
use std::marker::PhantomData;

use bevy_rapier3d::prelude::{AsyncCollider, RigidBody};
use moonshine_save::{
    prelude::{SavePlugin, LoadPlugin, load_from_file, LoadSet}, save::{SaveSet, save_default},
    //save::*,
};
use bevy::{asset::Asset, reflect::TypeUuid};
use bevy::{prelude::*, core_pipeline::core_3d::Camera3dDepthTextureUsage, reflect::GetTypeRegistration};
//use crate::urdf::urdf_loader::BevyRobot;
use bevy_component_extras::components::*;
use crate::{physics::{colliders::ColliderFlag, material::MaterialFlag, mesh::MeshPrimitive}, traits::Unwrap};
use crate::physics::mesh::GeometryFlag;
use crate::physics::rigidbodies::RigidBodyFlag;

use super::systems::*;
use super::components::*;
const SAVE_PATH: &str = "cube.ron";

#[derive(Default)]
pub struct SerializeComponentFor<T, U> {
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeComponentFor<T, U>
    where
        T: 'static + Component + for<'a> From<&'a U>,
        U: 'static + Component + GetTypeRegistration  + for<'a> From<&'a T> {
    fn build(&self, app: &mut App) {
        app
        .register_type::<U>()
        .add_systems(PreUpdate,
            (
             serialize_for::<T, U>,
            ).before(SaveSet::Save)
        )
        .add_systems(Update, 
            (
                deserialize_for::<U, T>
            ).after(LoadSet::PostLoad)
        )
        ;
    }
} 
#[derive(Default)]
pub struct SerializeAssetFor<T, U> {
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeAssetFor<T, U>
    where
        T: 'static + Asset + for<'a> From<&'a U>,
        U: 'static + Component + GetTypeRegistration  + for<'a> From<&'a T> {
    fn build(&self, app: &mut App) {
        app
        .register_type::<U>()
        .add_systems(PreUpdate,
            (
             try_serialize_asset_for::<T, U>,
            ).before(SaveSet::Save)
        )
        .add_systems(Update, 
            (
                deserialize_asset_for::<U, T>
            ).after(LoadSet::PostLoad)
        )
        ;
    }
} 

pub struct DeserializeAssetFrom<U, T> {
    wrapper_thing: PhantomData<fn() -> U>,
    thing: PhantomData<fn() -> T>,
}

impl<U, T> Plugin for DeserializeAssetFrom<U, T>
    where
        U: 'static + Component + GetTypeRegistration,
        T: 'static + Asset + TypeUuid + for<'a> Unwrap<&'a U>,
 {
    fn build(&self, app: &mut App) {
        app
        .register_type::<U>()
        .add_systems(Update, 
            (
                deserialize_wrapper_for::<U, T>
            ).after(LoadSet::PostLoad)
        )
        ;
    }
} 

impl<U, T> Default for DeserializeAssetFrom<U, T> {
    fn default() -> Self {
        Self {
            wrapper_thing: PhantomData,
            thing: PhantomData,
        }
    }
}

/// plugin that adds systems/plugins for serialization. 
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(
            (
                SavePlugin,
                LoadPlugin,
            )
        )
        .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())
        .add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())

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
            .exclude_component::<Handle<StandardMaterial>>()
            .exclude_component::<Handle<Mesh>>()
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