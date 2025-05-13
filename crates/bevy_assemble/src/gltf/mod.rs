
use bevy_asset::Handle;
use bevy_ecs::prelude::*;
use bevy_gltf::{Gltf, GltfExtras, GltfNode};
use bevy_log::warn;
use bevy_render::prelude::*;
use bevy_synonymize_physics::prelude::RequestCollider;
use bevy_transform::components::Transform;
use strum::IntoEnumIterator;

pub mod physics;
pub mod wrappers;


pub fn gltf_collider_request(extras: &GltfExtras) -> RequestCollider {
    if let Some(target) = extras.value.replace(['}', '{', '"'], "").split(':').last() {
        for variant in RequestCollider::iter() {
            if target == variant.to_string().to_lowercase() {
                return variant.into();
            }
        }
        warn!(
                "
                provided GltfExtra attribute did not match any valid primitive colliders. reverting to default
                valid variants: {:#?}
                parsed value: {:#}
                ", RequestCollider::iter().map(|n| n.to_string()).collect::<Vec<_>>(), target
            );
    };

    RequestCollider::default()
}

pub enum SchemaKind {
    GLTF,
}

/// a request to align the transform of this entity to match bevy's cordinate system.
#[derive(Component)]
#[require(Transform)]
pub struct TransformSchemaAlignRequest(pub Transform, pub SchemaKind);

#[derive(Component, Default)]
pub struct RootNode {
    handle: Handle<GltfNode>,
}

/// request to re-align geoemtry to match bevy.
/// TODO: replace with normal Mesh3d if gltf mesh loading is improved to not have this done at the [`Gltf`] level.
#[derive(Component)]
pub struct Mesh3dAlignmentRequest(pub Handle<Mesh>, pub SchemaKind);

/// Gltf associated with this entity.
#[derive(Component, Clone)]
pub struct GltfAssociation(pub Handle<Gltf>);
