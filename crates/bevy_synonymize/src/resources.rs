use std::{any::TypeId, collections::HashMap};

use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;

#[derive(Resource, Default, Deref, DerefMut)]
pub struct InitializedSynonyms{
    synonyms: HashMap<TypeId, String>
}