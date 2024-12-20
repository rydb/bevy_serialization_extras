//! The backbone of bevy_serialziation_extras

pub mod plugins;
pub mod queries;
pub mod resources;
mod systems;
pub mod traits;
pub mod wrappers;

pub mod prelude {
    pub use crate::{plugins::*, queries::*, resources::*, traits::*, wrappers::*};
}
