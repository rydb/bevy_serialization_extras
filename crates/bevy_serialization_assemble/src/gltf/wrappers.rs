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
use physics::{khr_implicit_shapes::khr_implicit_shapes::{KHRImplicitShapesMap, Shape, KHR_IMPLICIT_SHAPES}, khr_physics_rigid_bodies::{extension::KHR_PHYSICS_RIGID_BODIES, node::KHRPhysicsRigidBodiesNodeProp}};
use ref_cast::RefCast;
use strum::IntoEnumIterator;

use crate::{components::{DisassembleAssetRequest, DisassembleRequest, DisassembleStage, Maybe}, gltf::{physics::{parse_gltf_physics, GltfPhysicsRegistration, NodePhysicsRequest, PhysicsProperties}, GltfAssociation}, traits::{AssetLoadSettings, Disassemble, DisassembleSettings, Source, Split, Structure}};

use super::{gltf_collider_request, physics, Mesh3dAlignmentRequest, SchemaKind, TransformSchemaAlignRequest};

#[derive(Component, Debug)]
pub struct NodeId(pub usize);

#[derive(From, Clone, Deref, DerefMut, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfPrimitiveWrapper(pub GltfPrimitive);

impl Disassemble for GltfPrimitiveWrapper {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mat = value.material.clone().map(|n| MeshMaterial3d(n));
        Structure { 
            root: (
                Name::new(value.name.clone()),
                
                // Mesh3dAlignmentRequest(value.mesh.clone(), SchemaKind::GLTF),
                Mesh3d(value.mesh.clone()),
                Maybe(mat),
                Visibility::default(),
            ), 
            children: Vec::<()>::default(), 
            split: Split::default() 
        }
    }
}

#[derive(Deref, From, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfModel(#[deref] pub Gltf);

impl AssetLoadSettings for GltfModel {
    type LoadSettingsType = GltfLoaderSettings;

    fn load_settings() -> Option<Self::LoadSettingsType> {
        Some(GltfLoaderSettings {
            load_cameras: false,
            load_lights: false,
            include_source: true,
            load_meshes: RenderAssetUsages::default(),
            load_materials: RenderAssetUsages::default()
        })
    }
}

#[derive(From, Clone, Deref, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeWrapper(GltfNode);

impl AssetLoadSettings for GltfNodeWrapper {
    type LoadSettingsType = ();

    fn load_settings() -> Option<Self::LoadSettingsType> {
        None
    }
}

impl Disassemble for GltfNodeWrapper {
    fn components(value: &Self, settings: DisassembleSettings, source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mut children = Vec::new();

        let sub_nodes = &value.0.children;
        
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest::<GltfMeshWrapper>(
                DisassembleStage::Handle(n),
                DisassembleSettings::default(),
            )
        });

        for node in sub_nodes {
            children.push(
                DisassembleAssetRequest::<GltfNodeWrapper>::handle(node.clone(), None),
            )
        }
        println!("{:?} node id is {:#?}", value.name.clone(), value.index);
        Structure { 
            root: (
                Name::new(value.name.clone()),
                //TransformSchemaAlignRequest(value.transform.clone(), SchemaKind::GLTF),
                NodeId(value.index),
                value.transform.clone(),
                Maybe(mesh),
                Visibility::default(),
            ), 
            children: children, 
            split: Split { split: false, inheriet_transform: true }
        }
    }
}

impl Disassemble for GltfModel {
    fn components(value: &Self, settings: DisassembleSettings, source: Source) -> Structure<impl Bundle, impl Bundle> {
        //let nodes = value.nodes;

        println!("scenes: {:#?}", value.scenes);
        println!("nodes: {:#?}", value.nodes);
        //println!("gltf file: {:#?}", value.source);
        

        let mut name = "".to_owned();
        
        let Source::Asset(gltf_handle) = source else {
            panic!("Gltfs are always assets. How did this happen?");
        };
        
        let mut physics_extension_maps = PhysicsProperties::default();

        let gltf_handle = gltf_handle.typed::<Self::Target>();
        let mut node_physics_map = Vec::new();

        let top_node_candidates = &value.nodes;

        let mut top_node = None;

        if let Some(gltf) = &value.source {
            (physics_extension_maps, node_physics_map) = parse_gltf_physics(gltf).unwrap();
            
            let root_node = gltf.nodes().find(|n| n.children().len() > 0).unwrap();
            name = root_node.name().unwrap_or_default().to_owned();

            top_node = top_node_candidates.iter().nth(root_node.index());
            

        } else {
            warn!("gltf loaded without source. Cannot parse extensions. Offending gltf: {:#?}", gltf_handle.path())
        }

        let top_node = top_node.unwrap();

        Structure { 
            root: (
                GltfPhysicsRegistration {
                    physics: physics_extension_maps
                },
                GltfAssociation(gltf_handle),
                NodePhysicsRequest(node_physics_map),
                DisassembleAssetRequest::<GltfNodeWrapper>::handle(top_node.clone(), None),
            ),
            children: Vec::<()>::default(), 
            split: Split::default(), 
        }

    }
}

#[derive(From, Clone, Deref, DerefMut, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfMeshWrapper(pub GltfMesh);

impl AssetLoadSettings for GltfMeshWrapper {
    type LoadSettingsType = ();

    fn load_settings() -> Option<Self::LoadSettingsType> {
        None
    }
}

impl Disassemble for GltfMeshWrapper {
    fn components(value: &Self, settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mut children = Vec::new();
        for primitive in &value.0.primitives {
            children.push(DisassembleRequest(
                GltfPrimitiveWrapper(primitive.clone()),
                DisassembleSettings::default(),
            ))
        }
        Structure { 
            root: (
                Name::new(value.name.clone()),
                Visibility::default(),
            ), 
            children: children, 
            split: Split {
                split: settings.split,
                inheriet_transform: true,
            },
        }
    }
}




/// [`GltfMesh`] that will throw a warning and not initialize if there is more then 1/no primitive
///
/// Tempory hot-fix for spawning singular primitives.
/// Necessary due to physics with child primitives being unsupported in rapier and avain.
/// https://github.com/bevyengine/bevy/issues/17661
#[derive(From, Deref, Clone, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfPhysicsMeshPrimitive(pub GltfMesh);


impl AssetLoadSettings for GltfPhysicsMeshPrimitive {
    type LoadSettingsType = ();

    fn load_settings() -> Option<Self::LoadSettingsType> {
        None
    }
}

impl Disassemble for GltfPhysicsMeshPrimitive {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mesh = {
            if value.0.primitives.len() > 1 {
                //TODO: maybe replace this with some kind of mesh condenser system?
                warn!(
                    "Multiple primitives found for: {:#}. GltfMeshPrimtiveOne only supports one. Current count: {:#}",
                    value.0.name,
                    value.0.primitives.len()
                );
                None
            } else {
                value.0.primitives.first()
            }
        };

        let material = mesh
            .map(|n| n.material.clone())
            .and_then(|n| n)
            .and_then(|n| Some(MeshMaterial3d(n)));

        let primitive = mesh.map(|n| Mesh3d(n.mesh.clone()));

        let collider_request = if let Some(ref collider_kind) = value.0.extras {
            RequestCollider::from(gltf_collider_request(collider_kind))
        } else {
            RequestCollider::Convex
        };
        Structure { 
            root: (
                collider_request, Maybe(material), Maybe(primitive)
            ), 
            children: Vec::<()>::default(), 
            split: Split::default(),
        }
    }
}



#[derive(Clone, Deref, From, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeMeshOne(pub GltfNode);

impl Disassemble for GltfNodeMeshOne {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest::<GltfPhysicsMeshPrimitive>(
                DisassembleStage::Handle(n),
                DisassembleSettings::default(),
            )
        });

        let mut adjusted_transform = value.transform;
        let gltf_quat = value.transform.rotation;
        let adjust_quat = Quat::from_rotation_y(std::f32::consts::PI);
        adjusted_transform.rotation = adjust_quat * gltf_quat;

        Structure { 
            root: (
                Name::new(value.name.clone()),
                Maybe(mesh),
                //TransformSchemaAlignRequest(value.transform.clone(), SchemaKind::GLTF)
                adjusted_transform
            ), 
            children: Vec::<()>::default(), 
            split: Split::default()
        }
    }
}

impl AssetLoadSettings for GltfNodeMeshOne {
    type LoadSettingsType = ();

    fn load_settings() -> Option<Self::LoadSettingsType> {
        None
    }
}

/// GltfNode wrapper for spawning gltf nodes with a parent collider mesh, and children visual meshes.
/// This is for physics
#[derive(Clone, Deref, From, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeColliderVisualChilds(pub GltfNode);

#[derive(Clone, Deref, From, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeVisuals(pub Vec<Handle<GltfNode>>);

impl Disassemble for GltfNodeVisuals {
    fn components(value: &Self, settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mut children = Vec::new();

        for handle in value.0.clone() {
            children.push(DisassembleAssetRequest::<GltfNodeMeshOne>(
                DisassembleStage::Handle(handle),
                DisassembleSettings::default(),
            ))
        }
        Structure { 
            root: (), 
            children: children, 
            split: Split {
                split: settings.split,
                inheriet_transform: true,
            },
        }
    }
}


impl Disassemble for GltfNodeColliderVisualChilds {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle, impl Bundle> {
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest::<GltfPhysicsMeshPrimitive>(
                DisassembleStage::Handle(n),
                DisassembleSettings::default(),
            )
        });

        let collider_request = {
            if let Some(gltf_extras) = &value.0.extras {
                RequestCollider::from(gltf_collider_request(gltf_extras))
            } else {
                RequestCollider::Convex
            }
        };
        Structure { 
            root: (
                collider_request,
                Maybe(mesh),
                DisassembleRequest(
                    GltfNodeVisuals(value.0.children.clone()),
                    DisassembleSettings::default(),
                ),
            ), 
            children: Vec::<()>::default(), 
            split: Split::default(), 
        }
    }
}

