//use bevy::core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
//use bevy::ecs::query::{QueryData, WorldQuery};
//use bevy::render::camera::CameraRenderGraph;
use std::{any::TypeId, marker::PhantomData};
// use bevy_obj::ObjPlugin;
// use bevy_rapier3d::dynamics::{RigidBody, AdditionalMassProperties};
// use bevy_rapier3d::geometry::SolverGroups;
// use bevy_rapier3d::prelude::{AsyncCollider, ImpulseJoint};
use crate::prelude::mesh::MeshPrimitive;
use bevy_core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy_ecs::query::{QueryData, WorldQuery};
use bevy_render::camera::CameraRenderGraph;
use moonshine_save::load::load_from_file_on_request;
use moonshine_save::load::LoadPlugin;
use moonshine_save::load::LoadSystem;
use moonshine_save::save::SavePlugin;
use moonshine_save::save::SaveSystem;
use crate::traits::{ChangeChecked, FromStructure, IntoHashMap, LazyDeserialize};
use super::resources::*;
use super::systems::*;
use crate::wrappers::mesh::{GeometryFile, GeometryFlag};
use crate::{
    traits::*,
    wrappers::material::MaterialFlag,
};
use core::fmt::Debug;
use moonshine_save::prelude::save_default_with;
use moonshine_save::prelude::SaveFilter;


use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_asset::prelude::*;
use bevy_reflect::GetTypeRegistration;
use bevy_pbr::prelude::*;
use bevy_render::prelude::*;
use bevy_math::prelude::*;

pub struct SerializeQueryFor<S, T, U> {
    query: PhantomData<fn() -> S>,
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
    //post_processing: Vec<fn() -> ()>,
}

/// adds the given type to the skipped types list when serializing
pub fn skip_serializing<SkippedType: 'static>(
    app: &mut App
) {
    type L = SerializeFilter;
    let mut skip_list = app.world_mut().get_resource_or_insert_with::<L>(|| L::default());

    let skip_list_copy = skip_list.clone();
    skip_list.filter.components = skip_list_copy
        .filter
        .components
        .deny_by_id(TypeId::of::<SkippedType>());

}

impl<S, T, U> Plugin for SerializeQueryFor<S, T, U>
where
    S: 'static + QueryData + ChangeChecked,
    T: 'static
        + Component
        + Debug
        + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
    U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
{
    fn build(&self, app: &mut App) {
        skip_serializing::<T>(app);

        app
        .register_type::<U>()
        .add_systems(
            PreUpdate, 
            (
                serialize_for::<T, U>,
                deserialize_as_one::<S, T>,
            ).chain()
        );
        // app.add_systems(PreUpdate, (serialize_for::<T, U>,).before(SaveSystem::Save))
        //     .add_systems(
        //         Update,
        //         (deserialize_as_one::<S, T>).after(LoadSystem::PostLoad),
        //     );
    }
}

impl<S, T, U> Default for SerializeQueryFor<S, T, U> {
    fn default() -> Self {
        Self {
            query: PhantomData,
            thing: PhantomData,
            wrapper_thing: PhantomData,
            //post_processing: Vec::new()
        }
    }
}

pub trait Bob: Default {}

/// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
#[derive(Default)]
pub struct SerializeComponentFor<T, U>
where
    T: 'static + Component + for<'a> From<&'a U>,
    U: 'static + Component + for<'a> From<&'a T>,// + ManagedTypeRegistration ,
{
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeComponentFor<T, U>
// until trait alaises are stabalized, trait bounds have to be manually duplicated...
where
    T: 'static + Component + for<'a> From<&'a U>,
    U: 'static + Component + for<'a> From<&'a T> + GetTypeRegistration,
{
    fn build(&self, app: &mut App) {
        skip_serializing::<T>(app);

        app
        .register_type::<U>()
        .add_systems(PreUpdate, 
            (
                serialize_for::<T, U>,
                deserialize_for::<U, T>,
            ).chain()
        );
        // app.add_systems(
        //     PreUpdate,
        //     (serialize_for::<T, U>,), //.run_if(resource_added::<SaveRequest>())
        //                               //.before(SaveSet::Save)
        // )
        // .add_systems(
        //     Update,
        //     //(
        //     deserialize_for::<U, T>, //)//.after(LoadSet::PostLoad)
        // );
        
    }
}

/// plugin for serialization for WrapperComponent -> Asset, Asset -> WrapperComponent
#[derive(Default)]
pub struct SerializeAssetFor<T, U> {
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeAssetFor<T, U>
where
    T: 'static + Asset + for<'a> From<&'a U>,
    U: 'static + Component + GetTypeRegistration + for<'a> From<&'a T> + PartialEq,
{
    fn build(&self, app: &mut App) {
        skip_serializing::<Handle<T>>(app);

        app
        .register_type::<U>()
        // .add_systems(
        //     PreUpdate,
        //     (try_serialize_asset_for::<T, U>,), //.after(resource_added::<SaveRequest>())
        // )
        // .add_systems(
        //     Update,
        //     //(
        //     deserialize_asset_for::<U, T>, //).after(LoadSet::PostLoad)
        // );
        .add_systems(PreUpdate, 
            (
                try_serialize_asset_for::<T, U>,
                deserialize_asset_for::<U, T>,
            ).chain()

        );
    }
}

/// Plugin for deserialization for WrapperComponent -> Asset.
/// This is for assets that don't have 1:1 conversions from asset to warpper.
///
/// !!!Changes made to these assets at runtime will not be saved!!!
pub struct DeserializeAssetFrom<U, T> {
    wrapper_thing: PhantomData<fn() -> U>,
    thing: PhantomData<fn() -> T>,
}

impl<U, T> Plugin for DeserializeAssetFrom<U, T>
where
    U: 'static + Component + GetTypeRegistration,
    T: 'static + Asset + for<'a> Unwrap<&'a U>,
{
    fn build(&self, app: &mut App) {
        skip_serializing::<Handle<T>>(app);
        
        app
        .register_type::<U>()
        .add_systems(
            Update,
            (deserialize_wrapper_for::<U, T>).after(LoadSystem::PostLoad),
        );
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

pub struct SerializeManyAsOneFor<T, U> {
    things_query: PhantomData<fn() -> T>,
    composed_things_resource: PhantomData<fn() -> U>,
}

impl<U, T> Default for SerializeManyAsOneFor<U, T> {
    fn default() -> Self {
        Self {
            things_query: PhantomData,
            composed_things_resource: PhantomData,
        }
    }
}

impl<'v, T, U> Plugin for SerializeManyAsOneFor<T, U>
where
    T: 'static + QueryData,
    U: 'static
        + Asset
        + Default
        + Clone
        + for<'w, 's> IntoHashMap<Query<'w, 's, T>>
        + FromStructure
        + LazyDeserialize
        //+ LazySerialize,
{
    fn build(&self, app: &mut App) {
        app.world_mut()
            .get_resource_or_insert_with::<AssetSpawnRequestQueue<U>>(|| {
                AssetSpawnRequestQueue::<U>::default()
            });
        app.add_systems(
            PreUpdate,
            (serialize_structures_as_assets::<T, U>,).before(SaveSystem::Save),
        )
        .add_systems(
            Update,
            (deserialize_assets_as_structures::<U>).after(LoadSystem::PostLoad),
        );
    }
}
/// base addons for [`SerializationPlugins`]. Adds wrappers for some bevy structs that don't serialize/fully reflect otherwise.
pub struct SerializationBasePlugin;

impl Plugin for SerializationBasePlugin {
    fn build(&self, app: &mut App) {
        app

        // default conversions
        .add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFile, Mesh>::default());
    }
}

/// plugin that adds systems/plugins for serialization.
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<[f32; 3]>()
        .register_type::<AlphaMode>()
        .register_type::<ParallaxMappingMethod>()
        .register_type::<Camera3dDepthTextureUsage>()
        .register_type::<InheritedVisibility>()
        .register_type::<ScreenSpaceTransmissionQuality>()
        .register_type::<GeometryFile>()
        .register_type::<MeshPrimitive>()
        .register_type::<GeometryFlag>()
        .register_type::<[[f32; 3]; 3]>()
        .register_type::<[Vec3; 3]>()
        .register_type::<CameraRenderGraph>()
        .register_type::<TypeRegistryOnSave>()
        .register_type::<LoadRequest>()
        .register_type::<SaveRequest>()
        .register_type::<ComponentsOnSave>()
        .register_type::<ShowSerializable>()
        .register_type::<ShowUnserializable>()
        .insert_resource(ShowSerializable::default())
        .insert_resource(ShowUnserializable::default())
        .insert_resource(ComponentsOnSave::default())
        .insert_resource(TypeRegistryOnSave::default())
        .insert_resource(RefreshCounter::default())
        ;
        app.add_plugins((SavePlugin, LoadPlugin))
            // .register_type::<Option<Entity>>()

            //.add_systems(Update, from_structure::<Linkage, ImpulseJoint>)
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_added::<SaveRequest>),
            )
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_added::<LoadRequest>),
            )
            .add_systems(
                PreUpdate,
                update_last_saved_typedata.run_if(resource_changed::<RefreshCounter>),
            )
            .add_systems(
                Update,
                save_default_with(save_filter).into_file_on_request::<SaveRequest>(),
            )
            .add_systems(Update, add_inherieted_visibility.after(LoadSystem::PostLoad))
            .add_systems(Update, add_view_visibility.after(LoadSystem::PostLoad))
            .add_systems(Update, load_from_file_on_request::<LoadRequest>());
    }
}
// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveFilter {
    let f_modified = f.filter.clone();
    //f_modified.components.deny::<ComputedVisibility>();
    f_modified
}
