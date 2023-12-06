use std::{collections::{HashMap, VecDeque}, str::FromStr};

use bevy::{prelude::*, ecs::{query::{WorldQuery, self, ReadOnlyWorldQuery}, system::ReadOnlySystemParam}, asset::io::AssetSource};
use multimap::MultiMap;
use crate::{traits::*, wrappers::urdf::FromStructure, resources::AssetSpawnRequestQueue};

use bevy::asset::Asset;

// pub fn serialize_as_one<F, ThingSet, WrapperThing>(
//     mut commands: Commands,
// ) 
//     where
//         F: ReadOnlyWorldQuery,
//         ThingSet: Structure<ThingSet>
// {
    

// }

// pub fn deserialize_as_many_for<Thing, WrapperThing>(
//     thing_query: Query<&Thing>,
//     mut commands: Commands,
// ) 
//     where
//     Thing: Component,
//     WrapperThing: Component + for<'a> From<&'a Thing>  
// {
//     for f in thing_query.iter() {

//     }

// }

pub fn serialize_structures_as_assets<ThingSet, AssetType> (
    thing_query: Query<ThingSet>,
) 
    where
        ThingSet: WorldQuery,
        AssetType: Asset + for<'w, 's> From<Query<'w, 's, ThingSet>>,
{
    // populate later...
}

// pub fn serialize_structures_as_resource<ThingSet, ThingResource> (
//     thing_query: Query<ThingSet>,
//     things_resource: ResMut<ThingResource>,
//     mut commands: Commands,
// )

//     where
//         ThingSet: WorldQuery,
//         ThingResource: Resource + for<'w, 's> From<Query<'w, 's, ThingSet>>,
// {
//     commands.insert_resource(
//         ThingResource::from(thing_query)       
//     )
// }

pub fn deserialize_assets_as_structures<ThingAsset>(
    thing_assets: Res<Assets<ThingAsset>>,
    mut asset_spawn_requests: ResMut<AssetSpawnRequestQueue<ThingAsset>>,
    mut commands: Commands,
) 
    where
        ThingAsset: Asset + Clone + FromStructure
{
    let mut failed_requests = VecDeque::new();
    while asset_spawn_requests.requests.len() != 0 {
        if let Some(request) = asset_spawn_requests.requests.pop_front() {
            if let Some(asset) = thing_assets.get(request.item_id) {
                FromStructure::into_structures(&mut commands, asset.clone(), request)
            } else {
                let mut failed_request = request;
                failed_request.failed_load_attempts += 1;
                failed_requests.push_back(failed_request)
            }
        }
    }
    // re-add failed requests to asset_spawn_requests, as there could be a chance the asset just hasn't loaded yet.
    asset_spawn_requests.requests.append(&mut failed_requests);
    // for (asset_id, asset) in thing_assets.iter() {
    //     //FromStructure::into_structures(&mut commands, asset, asset_load_request.requests.clone());
    // }
}

// pub fn deserialize_resource_as_structures<ThingResource>(
//     things_resource: Res<ThingResource>,
//     mut resload_requests: ResMut<ResLoadRequests<ThingResource>>,
//     mut commands: Commands,
// ) 
//     where
//         ThingResource: Resource + Clone + FromStructure
// {
//     // let mut load_requests = Vec::new();
//     // if let Some(requests_as_resource) = resload_requests_check {
//     //     load_requests = requests_as_resource.requests
//     // }
//     //(TODO) get rid of this clone
//     //let leftover_reqs = 
//     //println!("handing of resource {:#?}", things_resource.into_inner().clone());
//     //let x = things_resource.into_inner().clone();
//     FromStructure::into_structures(&mut commands, things_resource.into_inner().clone(), resload_requests.requests.clone());
//     //resload_requests.requests = leftover_reqs;
// }

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Thing, WrapperThing>(
    thing_query: Query<(Entity, &Thing)>,
    mut commands: Commands,
)
    where
        Thing: Component,
        WrapperThing: Component + for<'a> From<&'a Thing>  
{
    for (e, f) in thing_query.iter() {
        commands.entity(e).insert(

            WrapperThing::from(f)
        );
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
pub fn deserialize_as_one<T, U>(
    mut commands: Commands,
    structure_query: Query<(Entity, T)>,
) 
    where
        T: WorldQuery,
        U: Component + for<'a, 'b> From<&'b <<T as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>>,
{
    for (e, thing_query) in structure_query.into_iter() {
        let unwrapped_thing = U::from(&thing_query);
        commands.entity(e).insert(
            unwrapped_thing
        );
    }
}

//takes a query, and serializes the components inside that query into a single resource
// pub fn serialize_as_one_resource<T, U, V>(
//     //mut commands: Commands,
//     thing_set_query: Query<(Entity, T)>
// ) 
//     where
//         T: WorldQuery + ReadOnlyWorldQuery + for<'a> Structure<<<T as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>>,
//         U: for<'w, 's> From<Query<'w, 's, (Entity, T)>>,
//         V: Resource,
// {
//     let mut thing_structures: HashMap<String, Vec<Entity>> = HashMap::new();

//     let urdf_resource = U::from(thing_set_query);
//     for (e, thing_set) in thing_set_query.iter() {

//         thing_structures.entry(T::structure(thing_set))
//         .or_default()
//         .push(e)
//         ;

//     } 
//     // for (structure_name, thing_set_) in thing_structures.iter() {
//     //     // let thing_set = thing_set_query.get(thing_set_entity);
//     //     for thing_
//     // }
// }

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Thing, WrapperThing> (
    things: ResMut<Assets<Thing>>,
    thing_query: Query<(Entity, &Handle<Thing>)>,
    mut commands: Commands,
)// -> bool
    where
        Thing: Asset,
        WrapperThing: Component + for<'a> From<&'a Thing> 
{
    for (e, thing_handle) in thing_query.iter() {
        match things.get(thing_handle) {
            Some(thing) => {
                commands.entity(e).insert(
                    WrapperThing::from(thing)
                );
            },
            None => {}
        }
    }
    //return true;
}

/// takes a wrapper component, and attempts to deserialize it into its asset handle through [`Unwrap`]
/// this is components that rely on file paths.
pub fn deserialize_wrapper_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Handle<Thing>>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + for<'a> Unwrap<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing_fetch_attempt = Thing::unwrap(wrapper_thing);
    
        match thing_fetch_attempt {
            Ok(thing) => {
               let thing_handle = things.add(thing);
                commands.entity(e).insert(
                    thing_handle
                );
            }
            Err(file_path) => {
                let thing_handle: Handle<Thing> = asset_server.load(file_path);
                commands.entity(e).insert(
                    thing_handle
                );
            }
        }
    }
}

/// takes a wrapper componnet, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<WrapperThing, Thing> (
    mut things: ResMut<Assets<Thing>>,
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Handle<Thing>>>,
    mut commands: Commands,
) 
    where
        WrapperThing: Component,
        Thing: Asset + for<'a> From<&'a WrapperThing>,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        let thing = Thing::from(wrapper_thing);
        let thing_handle = things.add(thing);

        commands.entity(e).insert(
            thing_handle
        );
    }
}

/// deserializes a wrapper component into its unserializable component variant.
pub fn deserialize_for<WrapperThing, Thing>(
    wrapper_thing_query: Query<(Entity, &WrapperThing), Without<Thing>>,
    mut commands: Commands,
) 
    where
        Thing: Component + for<'a> From<&'a WrapperThing>,
        WrapperThing: Component  
{
    for (e, f) in wrapper_thing_query.iter() {
        commands.entity(e).insert(
            Thing::from(f)
        );
    }
}

/// adds computed visability to componnets that don't have it. this should probably be removed
/// at some point...
pub fn add_inherieted_visibility(
    computed_visiblity_query: Query<Entity, Without<InheritedVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(InheritedVisibility::default());
    }
}

pub fn add_view_visibility(
    computed_visiblity_query: Query<Entity, Without<ViewVisibility>>,
    mut commands: Commands,
    
) {
    for e in computed_visiblity_query.iter() {
        commands.entity(e).insert(ViewVisibility::default());
    }
}


// pub fn query_test(
//     query: Query<(&Name, &Transform)>,
// ) {
//     for t in query.iter() {

//     }
// }