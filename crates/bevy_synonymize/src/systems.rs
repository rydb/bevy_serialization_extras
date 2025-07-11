use crate::{
    traits::*,
};
use std::{
    any::{type_name, TypeId},
    collections::HashMap, ops::Deref,
};

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::warn;
// use moonshine_save::save::Save;

/// synonymizes a component synonym with its target.
pub fn synonymize<Synonym>(
    thing_query: Query<
        (Entity, &Synonym::SynonymTarget),
        Or<(
            Added<Synonym::SynonymTarget>,
            Changed<Synonym::SynonymTarget>,
        )>,
    >,
    synonyms: Query<Entity, Changed<Synonym>>,
    mut commands: Commands,
) where
    Synonym: ComponentSynonym,
{
    for (e, f) in thing_query.iter() {
        // prevent infinite change back and forth
        if synonyms.contains(e) == false {
            // entity may not exist when inserting component if entity is deleted in the same frame as this.
            // this checks to make sure it exists to prevent a crash.
            commands.entity(e).try_insert(Synonym::from(&f));
        }
    }
}

/// takes an asset handle, and spawns a serializable copy of it on its entity
/// try to synonymize a component asset wrapper synonym 
pub fn try_synonymize_asset<Impl>(
    assets: ResMut<Assets<Impl::AssetType>>,
    things_query: Query<
        (Entity, &SynonymTarget<Impl>),
        Or<(
            Changed<SynonymTarget<Impl>>,
            Added<SynonymTarget<Impl>>,
        )>,
    >,
    changed_wrapper: Query<Entity, Changed<Impl::Synonym>>,
    mut commands: Commands,
) where
    Impl: AssetSynonymTarget,
{
    for (e, thing_handle) in things_query.iter() {
        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper.contains(e) == false {
            let new_wrapper = if let Some(path) = thing_handle.path() {
                Impl::Synonym::from(path.to_string())
            } else {
                let Some(asset) = assets.get(&**thing_handle) else {
                    warn!(
                        "Attempted serialize non-file asset {:#} to {:#} while the asset was unloaded. Skipping attempt",
                        type_name::<Impl::AssetType>(),
                        type_name::<Impl::Synonym>()
                    );
                    return;
                };
                let pure = Impl::from_asset(asset);

                Impl::Synonym::from(pure)
            };

            commands.entity(e).try_insert(new_wrapper);
        }
    }
}

// /// takes a wrapper component, and deserializes it back into its unserializable asset handle varaint
pub fn desynonymize_assset<Impl>(
    mut assets: ResMut<Assets<Impl::AssetType>>,
    wrapper_thing_query: Query<(Entity, &Impl::Synonym), Changed<Impl::Synonym>>,
    changed_wrapper_targets: Query<Entity, Changed<SynonymTarget<Impl>>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) where
    Impl: AssetSynonymTarget,
{
    for (e, wrapper_thing) in wrapper_thing_query.iter() {
        log::trace!("converting wrapper thing {:#?}", e);

        // do not update on the same frame that the target has updated to prevent infinite update chain
        if changed_wrapper_targets.contains(e) == false {
            let insert = match wrapper_thing.asset_state() {
                AssetState::Path(wrapper_path) => {
                    let handle = asset_server.load(wrapper_path);
                    SynonymTarget::<Impl>::from(handle)
                }
                AssetState::Pure(wrapper) => {
                    // let new_asset = Impl::AssetType::from(wrapper);
                    let new_asset = Impl::from_synonym(wrapper);

                    let handle = assets.add(new_asset);
                    SynonymTarget::<Impl>::from(handle)
                }
            };
            
            commands.entity(e).try_insert(insert);
        }
    }
}
/// desynonymize a synonym component back into its target.
pub fn desynonymize<Synonym>(
    wrapper_thing_query: Query<(Entity, &Synonym), Or<(Added<Synonym>, Changed<Synonym>)>>,
    mut commands: Commands,
) where
    Synonym: ComponentSynonym,
{
    for (e, f) in wrapper_thing_query.iter() {
        commands
            .entity(e)
            .try_insert(Synonym::SynonymTarget::from(&f));
    }
}


