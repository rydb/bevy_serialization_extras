//! The backbone of bevy_serialziation_extras

pub mod plugins;
pub mod resources;
mod systems;
pub mod traits;
pub mod wrappers;
pub mod components;

pub mod prelude {
    pub use crate::{plugins::*, resources::*, traits::*, wrappers::*};
}
