use crate::components::{RequestAssetStructure, RollDown, RollDownIded};
use crate::prelude::*;
use crate::resources::{AssetSpawnRequestQueue, RequestFrom};
use crate::traits::{Disassemble, Assemble};
use bevy_asset::prelude::*;
use bevy_core::Name;
use bevy_ecs::component::{ComponentId, Components};
use bevy_ecs::system::SystemState;
use bevy_ecs::{prelude::*, query::QueryData};
use bevy_hierarchy::{BuildChildren, Children};
use bevy_log::prelude::*;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::ops::Deref;
use crate::prelude::UrdfWrapper;
/// proxy system for checking load status of assets for component hooks.
pub fn run_asset_status_checkers(
    asset_systems: Res<AssetCheckers>, mut commands: Commands
) {
    for (_, system) in asset_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

pub fn run_rolldown_checkers(
    rolldown_systems: Res<RollDownCheckers>,
    mut commands: Commands
) {
    for (_, system) in rolldown_systems.0.iter() {
        // run systems for each asset type
        commands.run_system(*system);
    }
}

pub fn check_roll_down<T: Clone + Component>(
    initialized_children: Res<InitializedStagers>,
    rolldowns: Query<(Entity, &Name, &RollDownIded<T>)>,
    test: Query<EntityRef>,
    decendents: Query<&Children>,
    mut commands: Commands,
    components: &Components,
){
    for (e, name, rolldown) in &rolldowns {
        let Ok(children) = decendents.get(e) else {
            return
        };

        for child in children {
            let Ok(e_ref) = test.get(*child) else {
                return
            };
            let check_list = e_ref.archetype().components().collect::<Vec<_>>();
            //if components.any(|n| rolldown.1.contains(n));
            if rolldown.1.iter().any(|n| check_list.contains(n)) {
                commands.entity(*child).insert(rolldown.0.clone());
                commands.entity(e).remove::<RollDownIded<T>>();
                println!("finished rolling down");
            }
        }
    }
    
}

pub fn initialize_asset_structure<T>(
    //events: EventReader<AssetEvent<T::Inner>>,
    asset_server: Res<AssetServer>,
    requests: Query<(Entity, &RequestAssetStructure<T>)>,
    assets: Res<Assets<T::Target>>,
    mut commands: Commands,

) 
    where
        T: Clone + From<T::Target> + Deref + Disassemble + Send + Sync + 'static,
        T::Target: Asset + Clone
{
    //println!("checking initialize_asset structures...");
    for (e, request) in &requests {
        //println!("checking load status for... {:#}", e);
        let handle = match request {
            RequestAssetStructure::Handle(handle) => {handle},
            _ => {
                warn!("no handle??");
                return;
            }
        };
        if asset_server.is_loaded(handle) {
            let Some(asset) = assets.get(handle) else {
                warn!("handle for Asset<T::inner> reports being loaded by asset not available?");
                return;
            };
            //println!("Asset loaded for {:#}", e);
            // upgrading handle to asset
            commands.entity(e).remove::<RequestAssetStructure<T>>();
            commands.entity(e).insert(RequestAssetStructure::Asset(T::from(asset.clone())));
        } 
        // else {
        //     let status = asset_server.load_state(handle);
        //     println!("Asset unloaded: REASON: {:#?}", status);
        // }
    }
}

// //FIXME: implement this properly. Once an asset builder that could use this exists.
// pub fn serialize_structures_as_assets<ThingSet, AssetType>(//thing_query: Query<ThingSet>,
//     //asset_server: Res<AssetServer>,
//     //mut assets: ResMut<Assets<AssetType>>,
// )
// where
//     ThingSet: QueryData,
//     AssetType: Asset + for<'w, 's> Assemble<Query<'w, 's, ThingSet>> + Clone,
// {
//     // let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
//     // //println!("assets list is {:#?}", assets_list.keys());
//     // for (name, uncached_asset) in assets_list.iter() {
//     //     asset_server.add(uncached_asset.clone());
//     //     //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
//     // }
// }

#[derive(Resource)]
pub struct AssembleRequest(Vec<Entity>);

pub fn save_asset<T>(
    world: &mut World
)
    where
        T: Assemble + 'static
{
    let selected = world.get_resource::<AssembleRequest>().unwrap().0.clone();

    
    let mut system_state = SystemState::<T::Params>::new(world);

    let fetched = system_state.get_mut(world);
    let asset_target = T::assemble(selected, fetched);
    // let assets_list: HashMap<String, AssetType> = IntoHashMap::into_hashmap(thing_query);
    // //println!("assets list is {:#?}", assets_list.keys());
    // for (name, uncached_asset) in assets_list.iter() {
    //     asset_server.add(uncached_asset.clone());
    //     //LazyDeserialize::deserialize(uncached_asset.clone(), asset_handle.path());
    // }
}