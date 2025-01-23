use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_asset::Asset;
use bevy_ecs::{prelude::*, query::QueryData};
use moonshine_save::{load::LoadSystem, save::SaveSystem};

use crate::{
    resources::AssetSpawnRequestQueue,
    systems::{deserialize_assets_as_structures, run_asset_status_checkers, serialize_structures_as_assets},
    traits::{FromStructure, IntoHashMap},
};

/// Plugin for serializing collections of entities/components into a singular asset and vice versa.
pub struct SerializeManyAsOneFor<T, U> 
where
    T: 'static + QueryData,
    U: 'static
        + Asset
        + Default
        + Clone
        + for<'w, 's> IntoHashMap<Query<'w, 's, T>>
        + FromStructure
        //+ LazyDeserialize, //+ LazySerialize,
{
    things_query: PhantomData<fn() -> T>,
    composed_things_resource: PhantomData<fn() -> U>,
}

impl<T, U> Default for SerializeManyAsOneFor<T, U> 
where
    T: 'static + QueryData,
    U: 'static
        + Asset
        + Default
        + Clone
        + for<'w, 's> IntoHashMap<Query<'w, 's, T>>
        + FromStructure
        //+ LazyDeserialize, //+ LazySerialize,
{
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
        + FromStructure,
    
        //+ LazyDeserialize, //+ LazySerialize,
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

pub struct SerializationAssembleBasePlugin;

impl Plugin for SerializationAssembleBasePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, run_asset_status_checkers)
        ;
    }
}