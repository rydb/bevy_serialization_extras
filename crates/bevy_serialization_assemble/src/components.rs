use std::{any::{type_name, Any, TypeId}, collections::HashMap, fmt::Debug, marker::PhantomData, ops::Deref};

use crate::{prelude::{AssetCheckers, InitializedStagers, RollDownCheckers}, systems::{check_roll_down, initialize_asset_structure}, traits::{Disassemble, Structure}};
use bevy_asset::prelude::*;
use bevy_ecs::{component::{ComponentHooks, ComponentId, StorageType}, prelude::*, world::DeferredWorld};
use bevy_log::warn;
use bevy_hierarchy::prelude::*;
use bevy_reflect::{DynamicTypePath, Reflect};
use bevy_serialization_core::prelude::mesh::{MeshFlag3d, MeshWrapper};
use bevy_state::commands;
use bevy_transform::components::Transform;

// /// The structure this entity belongs to 
// #[derive(Component, Reflect)]
// #[reflect(Component)]
// pub struct StructureFlag(pub String);

/// Take inner new_type and add components to this components entity from [`Disassemble`]
#[derive(Clone)]
pub struct RequestStructure<T: Disassemble + Sync + Send + Clone + 'static>(pub T);

impl<T: Disassemble + Sync + Send + Clone + 'static> Component for RequestStructure<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                let comp = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get Disassemble on: {:#}", e);
                        return
                    },
                };
                comp.0.clone()
            };

            match Disassemble::components(comp) {
                Structure::Root(bundle) => {
                    world.commands().entity(e).insert(bundle);
                },
                Structure::Children(bundles, split) => {
                    let mut children = Vec::new();
                    for bundle in bundles {
                        
                        let child = world.commands().spawn(bundle).id();
                        //let components = world.entity(child).archetype().components().collect::<Vec<_>>();
                        if !split.0 {
                            world.commands().entity(e).add_child(child);

                        } else {
                            //TODO: expand this to other components than [`Transform`]
                            let parent_transform = {
                                let parent = world.entity(e).get::<Transform>();
                                parent.map(|n| n.clone())
                            };
                            if let Some(parent_trans) = parent_transform {
                                world.commands().entity(child).insert(parent_trans);
                            };

                        }
                        children.push(child);
                    }
                    {
                        let mut initialized_stagers = world.get_resource_mut::<InitializedStagers>().unwrap();

                        let previous_result = initialized_stagers.0.get_mut(&e);

                        match previous_result {
                            Some(res) => {
                                for child in children {
                                    res.push(child);
                                }
                            },
                            None => {
                                initialized_stagers.0.insert(e, children.clone());
                            },
                        }
                        //initialized_stagers.0.insert(e, v)
                        // let mut initialized_children = world.get_resource_mut::<InitializedChildren>().unwrap();
                        // if let Some(old_val) = initialized_children.0.insert(e, children) {
                        //     warn!("old value replaced for InitializedChildren, Refactor this to work with multiple  RequestStructur::Children to prevent bugs");
                        // }
                    }
                    world.commands().entity(e).remove::<Self>();
                },
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// Staging component for deserializing [`Disassemble`] implemented asset wrappers. 
/// depending on the owned information of the asset, this component is gradually elevated from Path -> Handle -> Asset
/// until [`Disassemble`] can be ran
#[derive(Clone, Debug)]
pub enum RequestAssetStructure<T> 
    where
        T: From<T::Target> + Deref,
        T::Target: Asset + Sized
{
    Path(String),
    Handle(Handle<T::Target>),
    Asset(T)
}

impl<T> Component for RequestAssetStructure<T>
    where
        T: Clone + From<T::Target> + Deref + Disassemble+ Send + Sync + 'static,
        T::Target: Asset + Clone
{
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let asset = {
                let path = match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get RequestAssetStructure on: {:#}", e);
                        return
                    },
                };
                let asset = match path {
                    RequestAssetStructure::Path(path) => {
                        let handle = world.load_asset(path);
                        //upgrade path to asset handle.
                        world.commands().entity(e).remove::<Self>();
                        world.commands().entity(e).insert(Self::Handle(handle));
                        return;
                    },
                    RequestAssetStructure::Handle(_) => {
                        
                        let add_system = 
                        {
                            let asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();
                            //let type_path = T::Inner::type_path();

                            //let comp_id = T::
                            if asset_checkers.0.contains_key(&id) {
                                //asset_checkers.0.insert(id, type_path.to_string());
                                false
                            } else {
                                true
                            }
                        };

                        if add_system {
                            let system_id = {
                                world.commands().register_system(initialize_asset_structure::<T>)
                            };
                            let mut asset_checkers = world.get_resource_mut::<AssetCheckers>().unwrap();

                            asset_checkers.0.insert(id, system_id);
                        }
                        return;
                    },
                    RequestAssetStructure::Asset(asset) => {
                        //world.commands().entity(e)
                        //println!("got asset");
                        asset
                    }
                };
                asset
            };
            //println!("populating structure for {:#}", e);
            
            
            match Disassemble::components(asset) {
                Structure::Root(bundle) => {
                    world.commands().entity(e).insert(bundle);
                },
                Structure::Children(bundles, split) => {
                    let mut children = Vec::new();
                    for bundle in bundles {
                        let child = world.commands().spawn(bundle).id();
                        if !split.0 {
                            world.commands().entity(e).add_child(child);
                        }
                        //let components = world.entity(child).archetype().components().collect::<Vec<_>>();
                        children.push((child));
            
                    }
                    {
                        let mut initialized_stagers = world.get_resource_mut::<InitializedStagers>().unwrap();

                        let previous_result = initialized_stagers.0.get_mut(&e);

                        match previous_result {
                            Some(res) => {
                                for child in children {
                                    res.push(child);
                                }
                            },
                            None => {
                                
                                for child in &children {
                                    initialized_stagers.0.insert(e, children.clone());
                                }    
                            
                            },
                        }
                        // let mut initialized_children = world.get_resource_mut::<InitializedChildren>().unwrap();
                        // if let Some(old_val) = initialized_children.0.insert(e, children) {
                        //     warn!("old value replaced for InitializedChildren, Refactor this to work with multiple  RequestStructur::Children to prevent bugs");
                        // }
                    }

                    world.commands().entity(e).remove::<Self>();
                },
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// Staging component for optional components. Is split open into inner component if it exists.
pub struct Maybe<T: Component>(pub Option<T>);


/// A hook that runs whenever [`Maybe`] is added to an entity.
///
/// Generates a [`MaybeCommand`].
fn maybe_hook<B: Component>(mut world: DeferredWorld<'_>, entity: Entity, _component_id: ComponentId) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().queue(MaybeCommand {
        entity,
        _phantom: PhantomData::<B>,
    });
}

struct MaybeCommand<B> {
    entity: Entity,
    _phantom: PhantomData<B>,
}

impl<B: Component> Command for MaybeCommand<B> {
    fn apply(self, world: &mut World) {
        let Ok(mut entity_mut) = world.get_entity_mut(self.entity) else {
            #[cfg(debug_assertions)]
            panic!("Entity with Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        let Some(maybe_component) = entity_mut.take::<Maybe<B>>() else {
            #[cfg(debug_assertions)]
            panic!("Maybe component not found");

            #[cfg(not(debug_assertions))]
            return;
        };

        if let Some(component) = maybe_component.0 {
            entity_mut.insert(component);
        }
    }
}

impl<T: Component> Component for Maybe<T> {
    const STORAGE_TYPE: bevy_ecs::component::StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(maybe_hook::<T>);
    }
}

/// staging component for resolving one component from another.
/// useful for bundles where the context for what something is has to be resolved later
#[derive(Clone)]
pub enum Resolve<T, U> {
    One(T),
    Other(U)
}

impl<T: Component + Clone, U: Component + Clone> Component for Resolve<T, U> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                match world.entity(e).get::<Self>() {
                    Some(val) => val.clone(),
                    None => {
                        warn!("could not get {:#?} on: {:#}", type_name::<Self>(), e);
                        return
                    },
                }
            };
            match comp {
                Resolve::One(one) => {world.commands().entity(e).insert(one);},
                Resolve::Other(other) => {world.commands().entity(e).insert(other);},
            }
            world.commands().entity(e).remove::<Self>();
        });
    }
}

#[derive(Clone)]
pub enum Ids {
    TypeId(Vec<TypeId>),
    ComponentId(Vec<ComponentId>)
}

/// staging component to roll down component to all children. 
#[derive(Clone)]
pub struct RollDown<T: Clone + Component>(
    pub T,
    /// components to check for roll down.
    /// no world access so this is a [`TypeId`] instead of [`ComponentId`]
    pub Vec<TypeId>,
);

/// Rolldown post [`ComponentId`] assignment.
#[derive(Component)]
pub struct RollDownIded<T: Component>(
    pub T,
    pub Vec<ComponentId>
);

impl<T: Component + Clone> Component for RollDown<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    
    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let rolldown_checkers = world.get_resource_mut::<RollDownCheckers>().unwrap();
            if !rolldown_checkers.0.contains_key(&id) {
                warn!("Adding rolldown system for {:#?}", id);
                let system_id = {
                    world.commands().register_system(check_roll_down::<T>)
                };
                let mut asset_checkers = world.get_resource_mut::<RollDownCheckers>().unwrap();

                asset_checkers.0.insert(id, system_id);
            }
            let mut valid_ids = Vec::new();

            let inner = match world.entity(e).get::<Self>() {
                Some(val) => {
                    let components = world.components();
                    for id in &val.1 {
                        if let Some(id) = components.get_id(*id) {
                            valid_ids.push(id);
                        }
                    }
                    //let x = world.get_by_id(entity, component_id)
                    val.0.clone()
                },
                None => {
                    warn!("could not get RollDown<T> on: {:#}", e);
                    return
                },
            };
            //world.commands().entity(e).remove::<Self>();
            world.commands().entity(e).insert(
                RollDownIded (
                    inner,
                    valid_ids
                )
            );
            

            // let children = {
            //     let Some(children) = world.entity(e).get::<Children>() else {
            //         warn!("No children for {:#}, skipping Rolldown<T>", e);
            //         return
            //     };
            //     children.to_vec()
            // };
            // for child in children.iter() {
            //     //let ids = world.register_component()
            //     //warn!("rolling down to {:#}", child);
            //     world.commands().entity(child.clone()).insert(comp.0.clone());
            // }

            //world.commands().entity(e).remove::<Self>();
        });
    }
}

