use std::{
    any::{TypeId, type_name},
    fmt::Debug,
    marker::PhantomData,
};

use crate::{
    Assemblies, AssemblyId,
    prelude::{AssetCheckers, InitializedStagers, RollDownCheckers},
    systems::{check_roll_down, initialize_asset_structure},
    traits::{Disassemble, DisassembleSettings, Structure},
};
use bevy_asset::prelude::*;
use bevy_derive::Deref;
use bevy_ecs::{
    component::{ComponentHooks, ComponentId, StorageType},
    prelude::*,
    world::DeferredWorld,
};
use bevy_hierarchy::prelude::*;
use bevy_log::warn;
use bevy_reflect::Reflect;
use bevy_transform::components::Transform;
use bevy_utils::HashSet;

// /// The structure this entity belongs to
// #[derive(Component, Reflect)]
// #[reflect(Component)]
// pub struct StructureFlag(pub String);

pub fn disassemble_components<'a, T>(
    world: &mut DeferredWorld<'a>,
    e: Entity,
    _id: ComponentId,
    comp: T,
    disassemble_settings: DisassembleSettings,
) where
    T: Disassemble,
{
    let assembly_id = {
        if let Some(assembly_id) = world.entity(e).get::<AssemblyId>() {
            assembly_id.0
        } else {
            // println!("creating new id for {:#}", e);
            let assemblies = world.get_resource_mut::<Assemblies>().unwrap();
            let mut latest_assembly = assemblies.0.iter().last().map(|n| *n.0).unwrap_or(0);

            latest_assembly += 1;

            world
                .commands()
                .entity(e)
                .insert(AssemblyId(latest_assembly));
            latest_assembly
        }
    };

    match Disassemble::components(comp, disassemble_settings) {
        Structure::Root(bundle) => {
            world.commands().entity(e).insert(bundle);
        }
        Structure::Children(bundles, split) => {
            let mut children = Vec::new();
            for bundle in bundles {
                let child = world.commands().spawn(bundle).id();
                world
                    .commands()
                    .entity(child)
                    .insert(AssemblyId(assembly_id));
                if !split.split {
                    world.commands().entity(e).add_child(child);
                } else {
                    // //TODO: expand this to other components than [`Transform`]

                    if split.inheriet_transform {
                        let parent_transform = {
                            let parent = world.entity(e).get::<Transform>();
                            parent.map(|n| n.clone())
                        };
                        if let Some(parent_trans) = parent_transform {
                            world.commands().entity(child).insert(parent_trans);
                        };
                    }

                    // //TODO: expand this to other components than [`Transform`]
                    // let parent_transform = {
                    //     let parent = world.entity(e).get::<Transform>();
                    //     parent.map(|n| n.clone())
                    // };
                    // if let Some(parent_trans) = parent_transform {
                    //     world.commands().entity(child).insert(parent_trans);
                    // };
                }
                children.push(child);
            }
            let mut assemblies = world.get_resource_mut::<Assemblies>().unwrap();
            let current_assembly_entities = assemblies
                .0
                .entry(assembly_id)
                .or_insert(HashSet::default());
            for child in &children {
                current_assembly_entities.insert(child.clone());
            }

            {
                let mut initialized_stagers =
                    world.get_resource_mut::<InitializedStagers>().unwrap();

                let previous_result = initialized_stagers.0.get_mut(&e);

                match previous_result {
                    Some(res) => {
                        for child in children {
                            res.push(child);
                        }
                    }
                    None => {
                        initialized_stagers.0.insert(e, children.clone());
                    }
                }
                //initialized_stagers.0.insert(e, v)
                // let mut initialized_children = world.get_resource_mut::<InitializedChildren>().unwrap();
                // if let Some(old_val) = initialized_children.0.insert(e, children) {
                //     warn!("old value replaced for InitializedChildren, Refactor this to work with multiple  RequestStructur::Children to prevent bugs");
                // }
            }
            //world.commands().entity(e).remove::<Self>();
        }
    }
}

/// Take inner new_type and add components to this components entity from [`Disassemble`]
#[derive(Deref, Clone)]
pub struct DisassembleRequest<T: Disassemble>(#[deref] pub T, pub DisassembleSettings);

impl<T: Disassemble> Component for DisassembleRequest<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let comp = {
                let comp = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get Disassemble on: {:#}", e);
                        return;
                    }
                };
                comp.clone()
            };

            disassemble_components(&mut world, e, id, comp.0, comp.1);
            world.commands().entity(e).remove::<Self>();
        });
    }
}

// /// Staging component for deserializing [`Disassemble`] implemented asset wrappers.
// /// depending on the owned information of the asset, this component is gradually elevated from Path -> Handle -> Asset
// /// until [`Disassemble`] can be ran
// #[derive(Clone, Debug)]
// pub enum DisassembleAssetRequest<T>
// where
//     T: From<T::Target> + Disassemble,
//     T::Target: Asset + Sized,
// {
//     Path(String),
//     Handle(Handle<T::Target>),
//     Asset(T),
// }

pub struct AssemblyIdList {
    pub parent: Option<Entity>,
    pub child: Option<Entity>,
}

/// Staging component for deserializing [`Disassemble`] implemented asset wrappers.
/// depending on the owned information of the asset, this component is gradually elevated from Path -> Handle -> Asset
/// until [`Disassemble`] can be ran
pub struct DisassembleAssetRequest<T>(pub DisassembleStage<T>, pub DisassembleSettings)
where
    T: Disassemble,
    T::Target: Asset + Sized;

impl<T: Disassemble> DisassembleAssetRequest<T>
where
    T: Disassemble,
    T::Target: Asset + Sized,
{
    pub fn path(path: String, custom_settings: Option<DisassembleSettings>) -> Self {
        Self(
            DisassembleStage::Path(path),
            custom_settings.unwrap_or_default(),
        )
    }
    pub fn handle(handle: Handle<T::Target>, custom_settings: Option<DisassembleSettings>) -> Self {
        Self(
            DisassembleStage::Handle(handle),
            custom_settings.unwrap_or_default(),
        )
    }
}

/// Stage of disassembly for a given [`Disassemble`] target. This is gradually broken down from Path -> Handle -> Asset until it can be disassembled.
pub enum DisassembleStage<T>
where
    T: Disassemble,
    T::Target: Asset + Sized,
{
    Path(String),
    Handle(Handle<T::Target>),
    Asset(T),
}

impl<T> Component for DisassembleAssetRequest<T>
where
    T: From<T::Target> + Disassemble,
    T::Target: Asset + Clone,
{
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let (asset, settings) = {
                let path = match world.entity(e).get::<Self>() {
                    Some(val) => val,
                    None => {
                        warn!("could not get DisassembleAssetRequest on: {:#}", e);
                        return;
                    }
                };

                let settings = path.1.clone();
                let asset = match &path.0 {
                    DisassembleStage::Path(path) => {
                        let handle = world.load_asset(path);
                        //upgrade path to asset handle.
                        world.commands().entity(e).remove::<Self>();
                        world
                            .commands()
                            .entity(e)
                            .insert(Self(DisassembleStage::Handle(handle), settings));
                        return;
                    }
                    DisassembleStage::Handle(_) => {
                        if world
                            .get_resource_mut::<AssetCheckers>()
                            .unwrap()
                            .0
                            .contains_key(&id)
                            == false
                        {
                            let system_id = {
                                world
                                    .commands()
                                    .register_system(initialize_asset_structure::<T>)
                            };
                            let mut asset_checkers =
                                world.get_resource_mut::<AssetCheckers>().unwrap();

                            asset_checkers.0.insert(id, system_id);
                        }
                        return;
                    }
                    DisassembleStage::Asset(asset) => asset,
                };
                (asset.clone(), settings)
            };
            disassemble_components(&mut world, e, id, asset, settings);
            world.commands().entity(e).remove::<Self>();
        });
    }
}

/// Staging component for optional components. Is split open into inner component if it exists.
pub struct Maybe<T: Component>(pub Option<T>);

/// A hook that runs whenever [`Maybe`] is added to an entity.
///
/// Generates a [`MaybeCommand`].
fn maybe_hook<B: Component>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _component_id: ComponentId,
) {
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
pub enum Resolve<T: Component, U: Component> {
    One(T),
    Other(U),
}

struct ResolveCommand<T, U> {
    entity: Entity,
    _phantom: PhantomData<(T, U)>,
}

fn resolve_command<T: Component, U: Component>(
    mut world: DeferredWorld<'_>,
    entity: Entity,
    _component_id: ComponentId,
) {
    // Component hooks can't perform structural changes, so we need to rely on commands.
    world.commands().queue(ResolveCommand {
        entity,
        _phantom: PhantomData::<(T, U)>,
    });
}

impl<T: Component, U: Component> Command for ResolveCommand<T, U> {
    fn apply(self, world: &mut World) {
        let Ok(mut e_mut) = world.get_entity_mut(self.entity) else {
            return;
        };

        let e = e_mut.id();
        let Some(comp) = e_mut.take::<Resolve<T, U>>() else {
            warn!("could not get {:#?} on: {:#?}", type_name::<Self>(), e);
            return;
        };

        match comp {
            Resolve::One(one) => e_mut.insert(one),
            Resolve::Other(other) => e_mut.insert(other),
        };
    }
}

impl<T: Component, U: Component> Component for Resolve<T, U> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut bevy_ecs::component::ComponentHooks) {
        _hooks.on_add(resolve_command::<T, U>);
    }
}

#[derive(Clone)]
pub enum Ids {
    TypeId(Vec<TypeId>),
    ComponentId(Vec<ComponentId>),
}

/// staging component to roll down component to all children.
#[derive(Reflect, Clone)]
pub struct RollDown<T: Clone + Component>(
    pub T,
    /// components to check for roll down.
    /// no world access so this is a [`TypeId`] instead of [`ComponentId`]
    /// insert an empty vec to just insert on first apperance of children
    pub Vec<TypeId>,
);

/// Rolldown post [`ComponentId`] assignment.
#[derive(Component)]
pub struct RollDownIded<T: Component>(pub T, pub Vec<ComponentId>);

impl<T: Component + Clone> Component for RollDown<T> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;

    fn register_component_hooks(_hooks: &mut ComponentHooks) {
        _hooks.on_add(|mut world, e, id| {
            let rolldown_checkers = world.get_resource_mut::<RollDownCheckers>().unwrap();
            if !rolldown_checkers.0.contains_key(&id) {
                //warn!("Adding rolldown system for {:#?}", id);
                let system_id = { world.commands().register_system(check_roll_down::<T>) };
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
                }
                None => {
                    warn!("could not get RollDown<T> on: {:#}", e);
                    return;
                }
            };
            //world.commands().entity(e).remove::<Self>();
            world
                .commands()
                .entity(e)
                .insert(RollDownIded(inner, valid_ids));

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
