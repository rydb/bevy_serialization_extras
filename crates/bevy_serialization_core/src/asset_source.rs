use bevy::reflect::Reflect;

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
