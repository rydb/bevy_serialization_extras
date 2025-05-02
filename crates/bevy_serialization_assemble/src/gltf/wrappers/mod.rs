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
use physics::{khr_implicit_shapes::khr_implicit_shapes::{KHRImplicitShapesMap, Shape, KHR_IMPLICIT_SHAPES}, khr_physics_rigid_bodies::{khr_physics_rigid_bodies::KHR_PHYSICS_RIGID_BODIES, khr_physics_rigid_bodies_nodes::KHRPhysicsRigidBodiesNodeProp}};
use ref_cast::RefCast;
use strum::IntoEnumIterator;

use crate::{components::{DisassembleAssetRequest, DisassembleRequest, DisassembleStage, Maybe}, gltf::physics::{parse_gltf_physics, GltfPhysicsRegistration, PhysicsProperties}, traits::{AssetLoadSettings, Disassemble, DisassembleSettings, Source, Split, Structure}};

use super::{gltf_collider_request, physics, Mesh3dAlignmentRequest, SchemaKind};

#[derive(From, Clone, Deref, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeWrapper(GltfNode);

#[derive(From, Clone, Deref, DerefMut, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfPrimitiveWrapper(pub GltfPrimitive);

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

impl Disassemble for GltfModel {
    fn components(value: &Self, _settings: DisassembleSettings, source: Source) -> Structure<impl Bundle> {
        //let nodes = value.nodes;

        println!("scenes: {:#?}", value.scenes);
        println!("nodes: {:#?}", value.nodes);
        //println!("gltf file: {:#?}", value.source);
        

        let mut name = "".to_owned();
        
        
        let mut physics = PhysicsProperties::default();
        if let Some(gltf) = &value.source {
            physics = parse_gltf_physics(gltf).unwrap();
            
            let root_node = gltf.nodes().find(|n| n.children().len() > 0).unwrap();
            name = root_node.name().unwrap_or_default().to_owned();

        } else {
            warn!("gltf loaded without source. Cannot parse extensions. Offending gltf nodes: {:#?}", value.named_nodes)
        }
        let Source::Asset(gltf_handle) = source else {
            panic!("Gltfs are always assets. How did this happen?");
        };
        let gltf_handle = gltf_handle.typed::<Self::Target>();

        //println!("gltf extensions: {:#?}", value.source)
        Structure::Root(
            (
                Name::new(name),
                GltfPhysicsRegistration {
                    gltf_handle,
                    physics
                }
                //gltf_properties
                       
            )
        )
        // let collider_request = if PHYSICS {
        //     if let Some(gltf_extras) = &value.0.extras {
        //         println!("gltf_extras are: {:#}", gltf_extras.value);
        //         Some(RequestCollider::from(gltf_collider_request(&gltf_extras)))
        //     } else {
        //         Some(RequestCollider::Convex)
        //     }
        // } else {
        //     None
        // };
        // let trans = value.transform.clone();
        // let visuals = value
        //     .0
        //     .mesh.clone()
        //     .map(|n| DisassembleAssetRequest::<GltfMeshWrapper>::handle(n, None));

        // let collider_request = if PHYSICS {
        //     if let Some(gltf_extras) = &value.0.extras {
        //         println!("gltf_extras are: {:#}", gltf_extras.value);
        //         Some(RequestCollider::from(gltf_collider_request(&gltf_extras)))
        //     } else {
        //         Some(RequestCollider::Convex)
        //     }
        // } else {
        //     None
        // };
        // println!("node name: {:#}", value.0.name);
        // Structure::Root((
        //     Maybe(collider_request),
        //     Maybe(visuals),
        //     Visibility::Visible,
        //     TransformSchemaAlignRequest(trans, SchemaKind::GLTF),
        // ))
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
    fn components(value: &Self, settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
        let mut children = Vec::new();
        for primitive in &value.0.primitives {
            children.push(DisassembleRequest(
                GltfPrimitiveWrapper(primitive.clone()),
                DisassembleSettings::default(),
            ))
        }
        Structure::Children(
            children,
            Split {
                split: settings.split,
                inheriet_transform: true,
            },
        )
    }
}

impl Disassemble for GltfNodeWrapper {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest(
                DisassembleStage::Handle::<GltfMeshWrapper>(n),
                DisassembleSettings::default(),
            )
        });

        Structure::Root((Maybe(mesh),))
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
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
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
        Structure::Root((collider_request, Maybe(material), Maybe(primitive)))
    }
}

impl Disassemble for GltfPrimitiveWrapper {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
        let mat = value.material.clone().map(|n| MeshMaterial3d(n));
        Structure::Root((
            Mesh3dAlignmentRequest(value.mesh.clone(), SchemaKind::GLTF),
            Maybe(mat),
        ))
    }
}

#[derive(Clone, Deref, From, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeMeshOne(pub GltfNode);

impl Disassemble for GltfNodeMeshOne {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest::<GltfPhysicsMeshPrimitive>(
                DisassembleStage::Handle(n),
                DisassembleSettings::default(),
            )
        });
        Structure::Root((Maybe(mesh),))
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
    fn components(value: &Self, settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
        let mut children = Vec::new();

        for handle in value.0.clone() {
            children.push(DisassembleAssetRequest::<GltfNodeMeshOne>(
                DisassembleStage::Handle(handle),
                DisassembleSettings::default(),
            ))
        }

        Structure::Children(
            children,
            Split {
                split: settings.split,
                inheriet_transform: true,
            },
        )
    }
}


impl Disassemble for GltfNodeColliderVisualChilds {
    fn components(value: &Self, _settings: DisassembleSettings, _source: Source) -> Structure<impl Bundle> {
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
        Structure::Root((
            collider_request,
            Maybe(mesh),
            DisassembleRequest(
                GltfNodeVisuals(value.0.children.clone()),
                DisassembleSettings::default(),
            ),
        ))
    }
}

