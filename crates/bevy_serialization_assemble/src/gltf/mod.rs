use std::{any::type_name, collections::HashMap};

use bevy_asset::{AssetContainer, Handle, RenderAssetUsages};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::prelude::*;
use bevy_gltf::{Gltf, GltfExtras, GltfLoaderSettings, GltfMesh, GltfNode, GltfPrimitive};
use bevy_log::warn;
use bevy_math::primitives::{Cuboid, Cylinder};
use bevy_pbr::MeshMaterial3d;
use bevy_render::prelude::*;
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use bevy_serialization_physics::prelude::{ColliderFlag, RequestCollider};
use bevy_transform::components::Transform;
use bytemuck::TransparentWrapper;
use derive_more::derive::From;
use glam::{Quat, Vec3};
use gltf::json::Value;
use physics::{
    PhysicsProperties,
    khr_implicit_shapes::khr_implicit_shapes::{KHR_IMPLICIT_SHAPES, KHRImplicitShapesMap, Shape},
    khr_physics_rigid_bodies::{
        extension::KHR_PHYSICS_RIGID_BODIES, node::KHRPhysicsRigidBodiesNodeProp,
    },
};
use ref_cast::RefCast;
use strum::IntoEnumIterator;

pub mod physics;
pub mod wrappers;

use crate::{
    components::{DisassembleAssetRequest, DisassembleRequest, DisassembleStage, Maybe},
    traits::{AssetLoadSettings, Disassemble, DisassembleSettings, Split, Structure},
};

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
