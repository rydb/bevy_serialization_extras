//! A crate that extends bevy_serialization_extras to include bevy <-> Rapier serialization support.

pub mod plugins;
mod systems;
pub mod synonyms;
//pub mod loaders;
// pub mod bundles;

pub mod prelude {
    pub use crate::{
        //loaders::*,
        // bundles::*,
        plugins::*,
        synonyms::*,
    };
}
