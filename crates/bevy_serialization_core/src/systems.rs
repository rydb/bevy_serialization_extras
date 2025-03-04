use crate::{
    prelude::{ComponentsOnSave, TypeRegistryOnSave},
    traits::*,
};
use std::any::{type_name, TypeId};

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::warn;
use bevy_reflect::TypeInfo;
use bevy_utils::HashMap;
use moonshine_save::save::Save;

/// takes a component, and spawns a serializable copy of it on its entity
pub fn serialize_for<Wrapper>(
    thing_query: Query<
        (Entity, &Wrapper::WrapperTarget),
        Or<(
            Added<Wrapper::WrapperTarget>,
            Changed<Wrapper::WrapperTarget>,
        )>,
    >,
    wrappers: Query<Entity, Changed<Wrapper>>,
    mut commands: Commands,
) where
    Wrapper: ComponentWrapper,
    // Echo<Wrapper>: From<Wrapper::Target>
{
    for (e, f) in thing_query.iter() {
        // prevent infinite change back and forth
        if wrappers.contains(e) == false {
            // entity may not exist when inserting component if entity is deleted in the same frame as this.
            // this checks to make sure it exists to prevent a crash.
            commands.entity(e).try_insert(Wrapper::from(&f));
        }
    }
}

/// Takes a query based interpretation of thing(`thing` that is composted of several components), and decomposes it into a single component
// pub fn deserialize_as_one<T, U>(
//     mut commands: Commands,
//     structure_query: Query<(Entity, T), Changed<T::ChangeCheckedComp>>,
// ) where
//     T: QueryData + ChangeChecked,
//     U: Component
//         + Debug
//         + for<'a, 'b> From<&'b <<T as QueryData>::ReadOnly as WorldQuery>::Item<'a>>,
// {
//     for (e, thing_query) in structure_query.into_iter() {
//         let unwrapped_thing = U::from(&thing_query);
//         //FIXME: This gets run very frequently, will need to figure out why that is
//         log::trace!("On, {:?}, inserting {:?}", e, unwrapped_thing);
//         commands.entity(e).try_insert(unwrapped_thing);
//     }
// }

// /// takes an asset handle, and spawns a serializable copy of it on its entity
// pub fn try_serialize_asset_for<Target, Wrapper>(
//     assets: ResMut<Assets<Target::AssetHandleComponent>>,
//     things_query: Query<(Entity, &Target), Or<(Changed<Target>, Without<Wrapper>)>>,
//     wrapper_things_query: Query<&Wrapper>,
//     mut commands: Commands,
// )
// // -> bool
// where
//     Target: AssetHandleComponent + Component + Deref<Target = Handle<Target::AssetHandleComponent>>,
//     Target::AssetHandleComponent: Asset,
//     Wrapper: Component + FromAsset<Target> + PartialEq,
// {
//     for (e, thing_handle) in things_query.iter() {
//         let new_wrapper_thing = Wrapper::from_asset(thing_handle, &assets);

//         let mut insert = true;

//         if let Ok(old_wrapper_thing) = wrapper_things_query.get(e) {
//             // don't re-insert the same component
//             if &new_wrapper_thing != old_wrapper_thing {
//                 log::trace!("changing Wrapper to match changed asset for {:#?}", e);
//                 insert = false;
//             }
//         }

//         if insert {
//             commands.entity(e).try_insert(new_wrapper_thing);
//         }
//     }
// }

/// takes an asset handle, and spawns a serializable copy of it on its entity
pub fn try_serialize_asset_for<Wrapper>(
    assets: ResMut<Assets<AssetType<Wrapper>>>,
    things_query: Query<(Entity, &Wrapper::WrapperTarget), Changed<Wrapper::WrapperTarget>>,
    wrapper_things_query: Query<&Wrapper>,
    mut commands: Commands,
) where
    Wrapper: AssetWrapper,
{
    for (e, thing_handle) in things_query.iter() {
        let mut insert = None;

        if let Ok(old_wrapper_thing) = wrapper_things_query.get(e) {
            let asset_state = old_wrapper_thing.asset_state();
            match asset_state {
                AssetState::Path(old_path) => {
                    if let Some(new_path) = thing_handle.path() {
                        let new_path = new_path.to_string();
                        if old_path != &new_path {
                            insert = Some(Wrapper::from(new_path))
                        }
                    }
                }
                AssetState::Pure(old_wrapper) => {
                    let Some(new_asset) = assets.get(&**thing_handle) else {
                        //warn!("Attempted serialize non-file asset {:#} to {:#} while the asset was unloaded. Skipping attempt", type_name::<AssetType<Wrapper>>(), type_name::<Wrapper>());
                        return;
                    };
                    let new_wrapper = Wrapper::PureVariant::from(new_asset);
                    // don't update wrapper unless asset changes would tangibly effect the wrapper
                    // (if part of the asset does not effect the wrapper, the wrapper wont update)
                    if old_wrapper != &new_wrapper {
                        insert = Some(Wrapper::from(new_wrapper));
                    }
                }
            };
        }

        if let Some(insert) = insert {
            commands.entity(e).try_insert(insert);
        }
        // if insert {
        //     commands.entity(e).try_insert(WrapAsset(new_wrapper));
        // }
    }
}

// /// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn deserialize_asset_for<Wrapper>(
    mut assets: ResMut<Assets<AssetType<Wrapper>>>,
    wrapper_thing_query: Query<(Entity, &Wrapper), Changed<Wrapper>>,
    things_query: Query<&Wrapper::WrapperTarget>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    Wrapper: AssetWrapper,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);

        let mut insert = None;

        if let Ok(current_asset) = things_query.get(e) {
            match wrapper_thing.asset_state() {
                AssetState::Path(wrapper_path) => {
                    if let Some(current_path) = current_asset.path() {
                        if &current_path.to_string() != wrapper_path {
                            let handle = asset_server.load(wrapper_path);
                            insert = Some(Wrapper::WrapperTarget::from(handle))
                        }
                    }
                }
                AssetState::Pure(wrapper) => {
                    let Some(current_asset) = assets.get(&**current_asset) else {
                        warn!("Attempted to deserialize non-file asset {:#} from {:#} while the asset was unloaded. Skipping attempt", type_name::<AssetType<Wrapper>>(), type_name::<Wrapper>());
                        return;
                    };
                    let current_asset_as_wrapper = Wrapper::PureVariant::from(current_asset);

                    // don't update wrapper unless asset changes would tangibly effect the wrapper
                    // (if part of the asset does not effect the wrapper, the wrapper wont update)
                    if wrapper != &current_asset_as_wrapper {
                        let new_asset = AssetType::<Wrapper>::from(wrapper);
                        let handle = assets.add(new_asset);
                        insert = Some(Wrapper::WrapperTarget::from(handle))
                    }
                }
            };
            if let Some(insert) = insert {
                commands.entity(e).try_insert(insert);
            }
        }
    }
}

pub fn deserialize_for<Wrapper>(
    wrapper_thing_query: Query<(Entity, &Wrapper), Or<(Added<Wrapper>, Changed<Wrapper>)>>,
    mut commands: Commands,
) where
    Wrapper: ComponentWrapper,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands
            .entity(e)
            .try_insert(Wrapper::WrapperTarget::from(&f));
    }
}

pub fn update_last_saved_typedata(world: &mut World) {
    let mut enetities_to_save = world.query_filtered::<Entity, With<Save>>();

    log::trace!("updating last saved type_data");

    let type_registry = world.resource::<AppTypeRegistry>();

    let mut saved_component_types = HashMap::new();
    for e in enetities_to_save.iter(&world) {
        for component in world.entity(e).archetype().components() {
            let comp_info = world.components().get_info(component).unwrap();
            saved_component_types.insert(comp_info.type_id().unwrap(), comp_info.name().to_owned());
        }
    }

    let registered_types = type_registry
        .read()
        .iter()
        .map(|id| {
            let type_id = id.type_id();

            return (type_id, TypeInfo::type_path(id.type_info()).to_owned());
        })
        .collect::<HashMap<TypeId, String>>();

    type L = TypeRegistryOnSave;
    world.insert_resource::<L>(L {
        registry: registered_types,
    });
    type O = ComponentsOnSave;
    world.insert_resource::<O>(O {
        components: saved_component_types,
    });
}
