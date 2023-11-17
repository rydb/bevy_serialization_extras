use std::collections::HashMap;

use bevy::{prelude::*, ecs::{query::{WorldQuery, self, ReadOnlyWorldQuery}, system::ReadOnlySystemParam}, asset::io::AssetSource};
use multimap::MultiMap;
use crate::traits::*;

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
// pub fn serialize_as_one<'a, T, U, V>(
//     //mut commands: Commands,
//     thing_set_query: Query<'a, 'a, T>
// ) 
//     where
//         T: WorldQuery + ReadOnlyWorldQuery + for<'b> Structure<&'b <<T as WorldQuery>::ReadOnly as WorldQuery>::Item<'a>>,
//         U: ,//AppendToResource,
//         V: Resource
// {
//     let mut thing_structures: HashMap<String, Vec<<T as WorldQuery>::Item<'a>>> = HashMap::new();

//     // collect all things that share the same structure, into a hashmap, where each "structure" string represents the structure they below to
//     // and each vec for that structure string represents how many thing_sets share that structure.
//     for thing_set in thing_set_query.iter() {

//         thing_structures.entry(T::structure(&thing_set))
//         .or_default()
//         .push(thing_set)
//         ;

//     } 
//     for (structure_name, things) in thing_structures.iter() {
//         // shove each structure into a "urdf" resource vec  
//     }
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