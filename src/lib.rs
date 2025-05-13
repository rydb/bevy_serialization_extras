pub mod prelude {
    #[cfg(feature = "assemble")]
    pub use bevy_assemble::prelude::*;
    pub use bevy_synonymize::prelude::*;
    #[cfg(feature = "physics")]
    pub use bevy_synonymize_physics::prelude::*;
}

// pub use bevy_synonymize;
// pub use bevy_synonymize_physics;
