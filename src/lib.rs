pub mod prelude {
    #[cfg(feature = "assemble")]
    pub use bevy_assemble::prelude::*;
    pub use bevy_synonomize::prelude::*;
    #[cfg(feature = "physics")]
    pub use bevy_synonyms_physics::prelude::*;
}

// pub use bevy_synonomize;
// pub use bevy_synonyms_physics;
