//use bevy::core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
//use bevy::ecs::query::{QueryData, WorldQuery};
//use bevy::render::camera::CameraRenderGraph;
use std::{any::TypeId, marker::PhantomData};
// use bevy_obj::ObjPlugin;
// use bevy_rapier3d::dynamics::{RigidBody, AdditionalMassProperties};
// use bevy_rapier3d::geometry::SolverGroups;
// use bevy_rapier3d::prelude::{AsyncCollider, ImpulseJoint};
use crate::prelude::mesh::MeshPrimitive;
use crate::prelude::UtilitySelection;
use bevy_core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy_ecs::query::{QueryData, WorldQuery};
use bevy_render::camera::CameraRenderGraph;
//use crate::prelude::UtilitySelection;
//use bevy::asset::Asset;
//use bevy::{prelude::*, reflect::GetTypeRegistration};
use moonshine_save::prelude::{load_from_file_on_request, LoadPlugin, LoadSet, SavePlugin};
use moonshine_save::save::SaveSet;
// use crate::loaders::urdf_loader::{UrdfLoaderPlugin, Urdf};
use crate::traits::{ChangeChecked, FromStructure, IntoHashMap, LazyDeserialize};
// use crate::wrappers::link::{Linkage, JointFlag, LinkQuery, StructureFlag, LinkFlag};
// use crate::wrappers::mass::MassFlag;
// use crate::wrappers::rigidbodies::RigidBodyFlag;
// use crate::wrappers::solvergroupfilter::SolverGroupsFlag;
use super::resources::*;
use super::systems::*;
use crate::ui::update_last_saved_typedata;
use crate::wrappers::mesh::{GeometryFile, GeometryFlag};
use crate::{
    traits::{ManagedTypeRegistration, Unwrap},
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

impl<S, T, U> Plugin for SerializeQueryFor<S, T, U>
where
    S: 'static + QueryData + ChangeChecked,
    T: 'static
        + Component
        + Debug
        + for<'a, 'b> From<&'b <<S as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
    U: 'static + Component + for<'a> From<&'a T> + ManagedTypeRegistration,
{
    fn build(&self, app: &mut App) {
        type L = SerializeFilter;
        let mut skip_list = app.world.get_resource_or_insert_with::<L>(|| L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy
            .filter
            .components
            .deny_by_id(TypeId::of::<T>());

        let type_registry = app.world.resource::<AppTypeRegistry>();
        for registration in U::get_all_type_registrations().into_iter() {
            type_registry.write().add_registration(registration)
        }
        app.add_systems(PreUpdate, (serialize_for::<T, U>,).before(SaveSet::Save))
            .add_systems(
                Update,
                (deserialize_as_one::<S, T>).after(LoadSet::PostLoad),
            );
        // for func in self.post_processing.clone() {
        //     app.add_systems(PostUpdate, func)
        //     ;
        // }
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

/// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
#[derive(Default)]
pub struct SerializeComponentFor<T, U>
where
    T: 'static + Component + for<'a> From<&'a U>,
    U: 'static + Component + ManagedTypeRegistration + for<'a> From<&'a T>,
{
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<T, U> Plugin for SerializeComponentFor<T, U>
// until trait alaises are stabalized, trait bounds have to be manually duplicated...
where
    T: 'static + Component + for<'a> From<&'a U>,
    U: 'static + Component + ManagedTypeRegistration + for<'a> From<&'a T>,
{
    fn build(&self, app: &mut App) {
        type L = SerializeFilter;
        let mut skip_list = app.world.get_resource_or_insert_with::<L>(|| L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy
            .filter
            .components
            .deny_by_id(TypeId::of::<T>());

        let type_registry = app.world.resource::<AppTypeRegistry>();
        for registration in U::get_all_type_registrations().into_iter() {
            type_registry.write().add_registration(registration)
        }
        app.add_systems(
            PreUpdate,
            (serialize_for::<T, U>,), //.run_if(resource_added::<SaveRequest>())
                                      //.before(SaveSet::Save)
        )
        .add_systems(
            Update,
            //(
            deserialize_for::<U, T>, //)//.after(LoadSet::PostLoad)
        );
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
    U: 'static + Component + GetTypeRegistration + for<'a> From<&'a T>,
{
    fn build(&self, app: &mut App) {
        type L = SerializeFilter;
        let mut skip_list = app.world.get_resource_or_insert_with::<L>(|| L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy
            .filter
            .components
            .deny_by_id(TypeId::of::<Handle<T>>());

        app.register_type::<U>()
            .add_systems(
                PreUpdate,
                (try_serialize_asset_for::<T, U>,), //.after(resource_added::<SaveRequest>())
            )
            .add_systems(
                Update,
                //(
                deserialize_asset_for::<U, T>, //).after(LoadSet::PostLoad)
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
    U: 'static + Component + ManagedTypeRegistration,
    T: 'static + Asset + for<'a> Unwrap<&'a U>,
{
    fn build(&self, app: &mut App) {
        type L = SerializeFilter;
        let mut skip_list = app.world.get_resource_or_insert_with::<L>(|| L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy
            .filter
            .components
            .deny_by_id(TypeId::of::<Handle<T>>());
        //skip_list.filter.components.deny_by_id(TypeId::of::<Handle<T>>());

        let type_registry = app.world.resource::<AppTypeRegistry>();
        for registration in U::get_all_type_registrations().into_iter() {
            type_registry.write().add_registration(registration)
        }
        app.add_systems(
            Update,
            (deserialize_wrapper_for::<U, T>).after(LoadSet::PostLoad),
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
        app.world
            .get_resource_or_insert_with::<AssetSpawnRequestQueue<U>>(|| {
                AssetSpawnRequestQueue::<U>::default()
            });
        app.add_systems(
            PreUpdate,
            (serialize_structures_as_assets::<T, U>,).before(SaveSet::Save),
        )
        .add_systems(
            Update,
            (deserialize_assets_as_structures::<U>).after(LoadSet::PostLoad),
        );
    }
}

/// plugin that adds systems/plugins for serialization.
/// `!!!THINGS THAT NEED TO BE SERIALIZED STILL MUST IMPLEMENT .register_type::<T>() IN ORDER TO BE USED!!!`
pub struct SerializationPlugin;

impl Plugin for SerializationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShowSerializable::default())
            .insert_resource(ShowUnserializable::default())
            .insert_resource(ComponentsOnSave::default())
            .insert_resource(TypeRegistryOnSave::default())
            .insert_resource(RefreshCounter::default())
            .insert_resource(UtilitySelection::default())
            // default conversions
            .add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())
            .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
            .add_plugins(DeserializeAssetFrom::<GeometryFile, Mesh>::default());

        app.add_plugins((SavePlugin, LoadPlugin))
            // .register_type::<Option<Entity>>()
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
            .add_systems(Update, add_inherieted_visibility.after(LoadSet::PostLoad))
            .add_systems(Update, add_view_visibility.after(LoadSet::PostLoad))
            .add_systems(Update, load_from_file_on_request::<LoadRequest>());
    }
}
// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveFilter {
    let f_modified = f.filter.clone();
    //f_modified.components.deny::<ComputedVisibility>();
    f_modified
}
