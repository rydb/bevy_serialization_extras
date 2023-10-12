
use std::marker::PhantomData;
use bevy::core_pipeline::core_3d::Camera3dDepthTextureUsage;
use bevy_rapier3d::prelude::AsyncCollider;
use moonshine_save::{
    prelude::{SavePlugin, LoadPlugin, load_from_file, LoadSet}, save::{SaveSet, save_default},
};
use bevy::{asset::Asset, reflect::TypeUuid};
use bevy::{prelude::*, reflect::GetTypeRegistration};
use crate::{wrappers::{colliders::ColliderFlag, material::MaterialFlag}, traits::{Unwrap, ManagedTypeRegistration}};
use crate::wrappers::mesh::GeometryFlag;

use super::systems::*;
const SAVE_PATH: &str = "cube.ron";

#[derive(Default)]
pub struct SerializeComponentFor<T, U> {
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeComponentFor<T, U>
    where
        T: 'static + Component + for<'a> From<&'a U>,
        U: 'static + Component + ManagedTypeRegistration  + for<'a> From<&'a T> {
    fn build(&self, app: &mut App) {
        
        let type_registry = app.world.resource::<AppTypeRegistry>();
        for registration in U::get_all_type_registrations().into_iter() {
            type_registry.write().add_registration(registration)
        }
        app
        
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
        U: 'static + Component + ManagedTypeRegistration,
        T: 'static + Asset + TypeUuid + for<'a> Unwrap<&'a U>,
 {
    fn build(&self, app: &mut App) {

        
        let type_registry = app.world.resource::<AppTypeRegistry>();
        for registration in U::get_all_type_registrations().into_iter() {
            type_registry.write().add_registration(registration)
        }
        app
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

        // .register_type::<Option<Entity>>()
        .register_type::<[f32; 3]>()
        .register_type::<AlphaMode>()
        .register_type::<ParallaxMappingMethod>()
        .register_type::<Camera3dDepthTextureUsage>()
        //.register_type::<bevy_mod_raycast::RaycastMethod>()
        // .register_type::<bevy_mod_raycast::system_param::RaycastVisibility>()
        // .register_type::<Debug>()
        // .register_type::<Viewer>()
        // .register_type::<Selectable>()
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
        .add_systems(Update, list_unserializable.run_if(check_for_save_keypress))
        ;    
    }
}