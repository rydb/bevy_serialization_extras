
use std::{marker::PhantomData, any::TypeId};
use bevy::core_pipeline::core_3d::{Camera3dDepthTextureUsage, ScreenSpaceTransmissionQuality};
use bevy::ecs::query::WorldQuery;
use bevy_rapier3d::prelude::{AsyncCollider, ImpulseJoint};
use moonshine_save::prelude::{SavePlugin, LoadPlugin, LoadSet, load_from_file_on_request};
use moonshine_save::save::SaveSet;
use bevy::asset::Asset;
use bevy::{prelude::*, reflect::GetTypeRegistration};
use crate::loaders::urdf_loader::UrdfLoaderPlugin;
use crate::wrappers::link::{Linkage, JointFlag, LinkQuery};
use crate::wrappers::urdf::{FromStructure, Urdfs};
use crate::{wrappers::{colliders::ColliderFlag, material::MaterialFlag}, traits::{Unwrap, ManagedTypeRegistration}};
use crate::wrappers::mesh::GeometryFlag;
use moonshine_save::prelude::save_default_with;
use moonshine_save::prelude::SaveFilter;
use crate::ui::update_last_saved_typedata;
use super::systems::*;
use super::resources::*;


pub struct SerializeQueryFor<S, T, U> {
    query: PhantomData<fn() -> S>,
    thing: PhantomData<fn() -> T>,
    wrapper_thing: PhantomData<fn() -> U>,
}

impl<S,T,U> Plugin for SerializeQueryFor<S, T, U>
    where
        S: 'static + WorldQuery,
        T: 'static + Component + for<'a, 'b> From<&'b <<S as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>>,
        U: 'static + Component + for<'a> From<&'a T> + ManagedTypeRegistration
{
    fn build(&self, app: &mut App) {
        type L = SerializeFilter;

        let mut skip_list = app.world
            .get_resource_or_insert_with::<L>(| |L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy.filter.components.deny_by_id(TypeId::of::<T>());
        
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
                deserialize_as_one::<S, T>
            ).after(LoadSet::PostLoad)
        )
        ;
    }
}

impl<S, T, U> Default for SerializeQueryFor<S, T, U> {
    fn default() -> Self {
        Self {
            query: PhantomData,
            thing: PhantomData,
            wrapper_thing: PhantomData,
        }
    }
}

/// plugin for serialization for WrapperComponent -> Component, Component -> WrapperComponent
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
        type L = SerializeFilter;
        let mut skip_list = app.world
            .get_resource_or_insert_with::<L>(| |L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy.filter.components.deny_by_id(TypeId::of::<T>());

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

/// plugin for serialization for WrapperComponent -> Asset, Asset -> WrapperComponent
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
        type L = SerializeFilter;
        let mut skip_list = app.world
            .get_resource_or_insert_with::<L>(| |L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy.filter.components.deny_by_id(TypeId::of::<Handle<T>>());


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
        let mut skip_list = app.world
            .get_resource_or_insert_with::<L>(| |L::default());

        let skip_list_copy = skip_list.clone();
        skip_list.filter.components = skip_list_copy.filter.components.deny_by_id(TypeId::of::<Handle<T>>());
        //skip_list.filter.components.deny_by_id(TypeId::of::<Handle<T>>());
        
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
        T: 'static + WorldQuery,
        U: 'static + Resource + Default + Clone + for<'w, 's> From<Query<'w, 's, T>> + FromStructure

 {
    fn build(&self, app: &mut App) {
        // type L = SerializeFilter;
        // let mut skip_list = app.world
        //     .get_resource_or_insert_with::<L>(| |L::default());

        // let skip_list_copy = skip_list.clone();
        // skip_list.filter.components = skip_list_copy.filter.components.deny_by_id(TypeId::of::<Handle<T>>());
        //skip_list.filter.components.deny_by_id(TypeId::of::<Handle<T>>());
        app.world.insert_resource(U::default());
        app.world.get_resource_or_insert_with::<ResLoadRequests<U>>(
            | |ResLoadRequests::<U>::default()
        );

        app
        .add_systems(PreUpdate,
            (
                serialize_structures_as_resource::<T, U>,
            ).before(SaveSet::Save)
        )
        .add_systems(Update, 
            (
                deserialize_resource_as_structures::<U>
            ).after(LoadSet::PostLoad)
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
        .insert_resource(ShowSerializable::default())
        .insert_resource(ShowUnserializable::default())
        .insert_resource(ComponentsOnSave::default())
        .insert_resource(TypeRegistryOnSave::default())
        .insert_resource(RefreshCounter::default())

        .add_plugins(UrdfLoaderPlugin)

        .add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())
        .add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())
        .add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())
        .add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
        //.add_plugins(SerializeComponentFor::<Mass, ColliderFlag>::default())

        .add_plugins(SerializeManyAsOneFor::<LinkQuery, Urdfs>::default())
        ;

        app
        .add_plugins(
            (
                SavePlugin,
                LoadPlugin,
            )
        )
        // .register_type::<Option<Entity>>()
        .register_type::<[f32; 3]>()
        .register_type::<AlphaMode>()
        .register_type::<ParallaxMappingMethod>()
        .register_type::<Camera3dDepthTextureUsage>()
        .register_type::<InheritedVisibility>()
        .register_type::<ScreenSpaceTransmissionQuality>()
        //.add_systems(Update, from_structure::<Linkage, ImpulseJoint>)
        .add_systems(PreUpdate, update_last_saved_typedata.run_if(resource_added::<SaveRequest>()))
        .add_systems(PreUpdate, update_last_saved_typedata.run_if(resource_added::<LoadRequest>()))
        .add_systems(PreUpdate, update_last_saved_typedata.run_if(resource_changed::<RefreshCounter>()))

        .add_systems(Update, 
            save_default_with(save_filter)
            .into_file_on_request::<SaveRequest>()             
        )
        .add_systems(Update, add_inherieted_visibility.after(LoadSet::PostLoad))
        .add_systems(Update, add_view_visibility.after(LoadSet::PostLoad))
        .add_systems(Update, 
            load_from_file_on_request::<LoadRequest>())
        ;  
    }
}
// save filter for this library.
fn save_filter(f: Res<SerializeFilter>) -> SaveFilter {
    let f_modified =     f.filter.clone();
    //f_modified.components.deny::<ComputedVisibility>();
    f_modified
}