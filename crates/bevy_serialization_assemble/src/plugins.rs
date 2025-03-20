use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_serialization_core::run_proxy_system;

use crate::{
    Assemblies, AssemblyId, SaveSuccess,
    prelude::{AssembleRequests, AssetCheckers, InitializedStagers, RollDownCheckers},
    systems::{
        SaveAssembledRequests, StagedAssembleRequestTasks, bind_joint_request_to_parent,
        handle_save_tasks, save_asset, stage_save_asset_request,
    },
    traits::{Assemble, Disassemble},
    urdf::UrdfWrapper,
};

/// Plugin for serializing collections of entities/components into a singular asset and vice versa.
pub struct SerializeManyAsOneFor<U>
where
    U: 'static + Default + Clone + Assemble + Disassemble, //+ LazyDeserialize, //+ LazySerialize,
{
    composed_things_resource: PhantomData<fn() -> U>,
}

impl<U> Default for SerializeManyAsOneFor<U>
where
    U: 'static + Default + Clone + Assemble + Disassemble,
{
    fn default() -> Self {
        Self {
            composed_things_resource: PhantomData,
        }
    }
}

impl<'v, T> Plugin for SerializeManyAsOneFor<T>
where
    T: 'static + Default + Clone + Assemble + Disassemble,
{
    fn build(&self, app: &mut App) {
        app
        .insert_resource(AssembleRequests::<T>::default())
        .insert_resource(SaveAssembledRequests::<T::Target>::default())
        .add_systems(PreUpdate, stage_save_asset_request::<T>)
        // .add_systems(PreUpdate, handle_save_tasks)
        .add_systems(PreUpdate, save_asset::<T>)
        // .add_systems()
        ;
    }
}


pub struct SerializationAssembleBasePlugin;

impl Plugin for SerializationAssembleBasePlugin {
    fn build(&self, app: &mut App) {
        app
        
        .add_plugins(SerializeManyAsOneFor::<UrdfWrapper>::default())
        .insert_resource(AssetCheckers::default())
        .insert_resource(InitializedStagers::default())
        .insert_resource(RollDownCheckers::default())
        .insert_resource(Assemblies::default())
        .init_resource::<StagedAssembleRequestTasks>()
        .register_type::<AssemblyId>()
        .add_event::<SaveSuccess>()

        // .register_type::<RollDown>()
        .add_systems(Update, handle_save_tasks)
        .add_systems(Update, run_proxy_system::<AssetCheckers>)
        .add_systems(Update, run_proxy_system::<RollDownCheckers>)
        .add_systems(PreUpdate, bind_joint_request_to_parent)
        //.add_systems(Update, name_from_id)
        ;
    }
}
