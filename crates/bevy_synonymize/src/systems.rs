use crate::{
    prelude::{ComponentsOnSave, TypeRegistryOnSave},
    traits::*,
};
use std::{
    any::{TypeId, type_name},
    collections::HashMap,
};

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::warn;
// use moonshine_save::save::Save;

/// synonymizes a component synonym with its target.
pub fn synonymize<Wrapper>(
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

/// takes an asset handle, and spawns a serializable copy of it on its entity
/// try to synonymize a component asset wrapper synonym 
pub fn try_synonymize_asset<Wrapper>(
    assets: ResMut<Assets<AssetType<Wrapper>>>,
    things_query: Query<
        (Entity, &Wrapper::WrapperTarget),
        Or<(
            Changed<Wrapper::WrapperTarget>,
            Added<Wrapper::WrapperTarget>,
        )>,
    >,
    changed_wrapper: Query<Entity, Changed<Wrapper>>,
    mut commands: Commands,
) where
    Wrapper: AssetWrapper,
{
    for (e, thing_handle) in things_query.iter() {
        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper.contains(e) == false {
            let new_wrapper = if let Some(path) = thing_handle.path() {
                Wrapper::from(path.to_string())
            } else {
                let Some(asset) = assets.get(&**thing_handle) else {
                    warn!(
                        "Attempted serialize non-file asset {:#} to {:#} while the asset was unloaded. Skipping attempt",
                        type_name::<AssetType<Wrapper>>(),
                        type_name::<Wrapper>()
                    );
                    return;
                };
                let pure = Wrapper::PureVariant::from(asset);

                Wrapper::from(pure)
            };

            commands.entity(e).try_insert(new_wrapper);
        }
    }
}

// /// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn desynonymize_assset<Wrapper>(
    mut assets: ResMut<Assets<AssetType<Wrapper>>>,
    wrapper_thing_query: Query<(Entity, &Wrapper), Changed<Wrapper>>,
    changed_wrapper_targets: Query<Entity, Changed<Wrapper::WrapperTarget>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    Wrapper: AssetWrapper,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);

        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper_targets.contains(e) == false {
            let insert = match wrapper_thing.asset_state() {
                AssetState::Path(wrapper_path) => {
                    let handle = asset_server.load(wrapper_path);
                    Wrapper::WrapperTarget::from(handle)
                }
                AssetState::Pure(wrapper) => {
                    let new_asset = AssetType::<Wrapper>::from(wrapper);
                    let handle = assets.add(new_asset);
                    Wrapper::WrapperTarget::from(handle)
                }
            };
            commands.entity(e).try_insert(insert);
        }
    }
}
/// desynonymize a synonym component back into its target.
pub fn desynonymize<Wrapper>(
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


