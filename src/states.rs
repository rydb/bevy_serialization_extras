use bevy::prelude::States;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum SaveState {
    PostSave,
    #[default]
    PostLoad,
}