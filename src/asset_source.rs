use bevy::{reflect::Reflect, ecs::component::Component};

// pub struct AssetSource<T: Component> {

// }

#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum AssetSource {
    Package(String),
    Placeholder(String),
}


impl Default for AssetSource {
    fn default() -> Self {
        Self::Placeholder("PLACE_HOLDER_PATH".to_owned())
    }
}
