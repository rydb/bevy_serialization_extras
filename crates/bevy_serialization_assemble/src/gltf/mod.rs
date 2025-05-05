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
use physics::{khr_implicit_shapes::khr_implicit_shapes::{KHRImplicitShapesMap, Shape, KHR_IMPLICIT_SHAPES}, khr_physics_rigid_bodies::{khr_physics_rigid_bodies::KHR_PHYSICS_RIGID_BODIES, khr_physics_rigid_bodies_nodes::KHRPhysicsRigidBodiesNodeProp}, PhysicsProperties};
use ref_cast::RefCast;
use strum::IntoEnumIterator;


pub mod wrappers;
pub mod physics;

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

// impl IntoHashMap<Query<'_, '_, GltfNodeQuery>> for GltfNodeWrapper {
//     fn into_hashmap(value: Query<'_, '_, GltfNodeQuery>, world: &World) -> std::collections::HashMap<String, Self> {
//         let mut gltf_map = HashMap::new();

//         //let meshs = world.query::<&Mesh3d>();
//         //let materials = world.query::<&MeshMaterial3d<StandardMaterial>>();
//         let Some(gltf_meshes) = world.get_resource::<Assets<GltfMesh>>() else {
//             warn!("no Asssets<GltfMesh> resource. Aborting");
//             return gltf_map
//         };

//         for node in value.iter() {
//             //let node = &node.node.0;

//             // let Some(node) = node else {
//             //     warn!("No associated node found for gltf_node. Skipping");
//             //     continue;
//             // };

//             //gltf_map.insert(node.clone(), NodeFlag(node.clone()));
//             let mut primitives = Vec::new();
//             for child in node.children.iter() {

//                 //let mesh = world.get::<Mesh3d>(*child).map(|n| n.0.clone());
//                 let Some(mesh) = world.get::<Mesh3d>(*child) else {
//                     warn!("primitive has no mesh. skipping");
//                     continue;
//                 };
//                 let name = world.get::<Name>(*child).map(|n| n.to_string()).unwrap_or_default();
//                 let material = world.get::<MeshMaterial3d<StandardMaterial>>(*child).map(|n| n.0.clone());

//                 let primitive = GltfPrimitive {
//                     //TODO: implement properly
//                     index: 0,
//                     //TODO: implement properly
//                     parent_mesh_index: 0,
//                     name: name,
//                     mesh: mesh.0.clone(),
//                     material: material,
//                     //TODO: implement properly
//                     extras: None,
//                     material_extras: None,
//                 };
//                 primitives.push(primitive);

//             }

//             let gltf_mesh = GltfMesh {
//                 //implement properly
//                 index: 0,
//                 //implement properly
//                 name: node.node.0.clone(),
//                 primitives: primitives,
//                 extras: todo!(),
//             };
//             let gltf_mesh_handle = gltf_meshes.add(gltf_mesh);
//             gltf_map.insert(node.node.0,
//                 GltfNodeWrapper(GltfNode {
//                     // implement properly
//                     index: 0,
//                     // implement properly
//                     name: node.node.0,
//                     // not supported.
//                     children: Vec::default(),
//                     mesh: Some(gltf_mesh_handle),
//                     skin: None,
//                     transform: todo!(),
//                     // implement properly
//                     is_animation_root: false,
//                     // implement properly
//                     extras: None,
//             }));
//         }
//         gltf_map
//     }
// }
