use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_asset::{processor::InitializeError, Asset};
use bevy_ecs::{prelude::*, query::QueryData};
use moonshine_save::{load::LoadSystem, save::SaveSystem};

use crate::{
    prelude::{AssembleRequest, AssetCheckers, InitializedStagers, RollDownCheckers}, resources::AssetSpawnRequestQueue, systems::{bind_joint_request_to_parent, generate_primitive_for_request, run_asset_status_checkers, run_rolldown_checkers, save_asset}, traits::{Assemble, Disassemble, LazySerialize}, Assemblies, AssemblyId
};

/// Plugin for serializing collections of entities/components into a singular asset and vice versa.
pub struct SerializeManyAsOneFor<U> 
where
    U: 'static
        + Asset
        + Default
        + Clone
        + Assemble
        + Disassemble
        //+ LazyDeserialize, //+ LazySerialize,
{
    composed_things_resource: PhantomData<fn() -> U>,
}

impl<U> Default for SerializeManyAsOneFor<U> 
where
    U: 'static
        + Asset
        + Default
        + Clone
        + Assemble
        + Disassemble
        //+ LazyDeserialize, //+ LazySerialize,
{
    fn default() -> Self {
        Self {
            composed_things_resource: PhantomData,
        }
    }
}

impl<'v, T> Plugin for SerializeManyAsOneFor<T>
where
    T: 'static
        + Asset
        + Default
        + Clone
        + Assemble
        + Disassemble
        + LazySerialize
        //+ LazyDeserialize, //+ LazySerialize,
{
    fn build(&self, app: &mut App) {
        // app.world_mut()
        //     .get_resource_or_insert_with::<AssetSpawnRequestQueue<U>>(|| {
        //         AssetSpawnRequestQueue::<U>::default()
        //     });
        app.add_systems(
            PreUpdate,
            (save_asset::<T>,).before(SaveSystem::Save),
        )
        ;
    }
}

pub struct SerializationAssembleBasePlugin;

impl Plugin for SerializationAssembleBasePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AssetCheckers::default())
        .insert_resource(InitializedStagers::default())
        .insert_resource(RollDownCheckers::default())
        .insert_resource(AssembleRequest::default())
        .insert_resource(Assemblies::default())
        .register_type::<AssemblyId>()
        .add_systems(Update, run_asset_status_checkers)
        .add_systems(PostUpdate, run_rolldown_checkers)
        .add_systems(PreUpdate, bind_joint_request_to_parent)
        //.add_systems(Update, generate_primitive_for_request)
        //.add_systems(Update, name_from_id)
        ;
    }
}