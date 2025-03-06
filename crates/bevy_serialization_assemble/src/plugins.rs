use std::marker::PhantomData;

use bevy_app::prelude::*;
use bevy_serialization_core::run_proxy_system;

use crate::{
    prelude::{AssembleRequests, AssetCheckers, InitializedStagers, RollDownCheckers}, systems::{bind_joint_request_to_parent, generate_primitive_for_request}, traits::{Assemble, Disassemble, LazySerialize}, urdf::UrdfWrapper, Assemblies, AssemblyId
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
    U: 'static + Default + Clone + Assemble + Disassemble, //+ LazyDeserialize, //+ LazySerialize,
{
    fn default() -> Self {
        Self {
            composed_things_resource: PhantomData,
        }
    }
}

impl<'v, T> Plugin for SerializeManyAsOneFor<T>
where
    T: 'static + Default + Clone + Assemble + Disassemble + LazySerialize, //+ LazyDeserialize, //+ LazySerialize,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(AssembleRequests::<T>::default())
        //.add_systems(PreUpdate, (save_asset::<T>,).before(SaveSystem::Save));
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
        .register_type::<AssemblyId>()
        .add_systems(Update, run_proxy_system::<AssetCheckers>)
        .add_systems(Update, run_proxy_system::<RollDownCheckers>)
        .add_systems(PreUpdate, bind_joint_request_to_parent)
        .add_systems(Update, generate_primitive_for_request)
        //.add_systems(Update, name_from_id)
        ;
    }
}
