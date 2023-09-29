use bevy::{prelude::*, ecs::world};

pub trait Wrapper {
    fn serialize(world: &mut World);

    fn deserialize(world: &mut World);
}