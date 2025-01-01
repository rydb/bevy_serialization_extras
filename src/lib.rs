pub mod prelude {
    pub use bevy_serialization_core::prelude::*;
    #[cfg(feature = "physics")]
    pub use bevy_serialization_physics::prelude::*;
    #[cfg(feature = "assemble")]
    pub use bevy_serialization_assemble::prelude::*;
}

// pub use bevy_serialization_core;
// pub use bevy_serialization_physics;
