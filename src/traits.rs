use bevy::{prelude::*, ecs::world};
use bevy::ecs::system::{SystemState, SystemParam};

/// Denotes that a wrapper component can be serialized from the Bevy ECS world.
/// serializability implies deserializability
pub trait ECSSerialize: ECSDeserialize + Component + Sized {
    fn serialize_for<T: Component + From<Self>>(world: &mut World);
}

/// Denotes that a wrapper component can be deserialized into a Bevy ECS world.
pub trait ECSDeserialize: Component {
    //fn deserialize<T: SystemParam>(world: &mut World, system_param: SystemState<T>);
    fn deserialize(world: &mut World);
}