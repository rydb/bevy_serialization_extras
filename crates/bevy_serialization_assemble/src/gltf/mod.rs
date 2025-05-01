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
use khr_implicit_shapes::{KHRImplicitShapesMap, Shape, KHR_IMPLICIT_SHAPES};
use khr_physics_rigid_bodies::{KhrPhysicsRigidBodiesMap, KHR_PHYSICS_RIGID_BODIES};
use khr_physics_rigid_bodies_nodes::KHRPhysicsRigidBodiesNodeProp;
use ref_cast::RefCast;
use strum::IntoEnumIterator;


mod khr_implicit_shapes;
mod khr_physics_rigid_bodies;
mod khr_physics_rigid_bodies_nodes;

use crate::{
    components::{DisassembleAssetRequest, DisassembleRequest, DisassembleStage, Maybe},
    traits::{AssetLoadSettings, Disassemble, DisassembleSettings, Split, Structure},
};

#[derive(From, Clone, Deref, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfNodeWrapper(GltfNode);

#[derive(From, Clone, Deref, DerefMut, TransparentWrapper)]
#[repr(transparent)]
pub struct GltfPrimitiveWrapper(pub GltfPrimitive);

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

// pub type GltfPhysicsModel = GltfModel<true>;
// pub type GltfVisualModel = GltfModel<false>;

// #[derive(Deref, From)]
// pub struct GltfModel<const PHYSICS: bool>(#[deref] pub GltfNode);

// impl<const PHYSICS: bool> Disassemble for GltfModel<PHYSICS> {
//     fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
//         let trans = value.transform.clone();
//         let visuals = value
//             .0
//             .mesh.clone()
//             .map(|n| DisassembleAssetRequest::<GltfMeshWrapper>::handle(n, None));

//         let collider_request = if PHYSICS {
//             if let Some(gltf_extras) = &value.0.extras {
//                 println!("gltf_extras are: {:#}", gltf_extras.value);
//                 Some(RequestCollider::from(gltf_collider_request(&gltf_extras)))
//             } else {
//                 Some(RequestCollider::Convex)
//             }
//         } else {
//             None
//         };
//         println!("node name: {:#}", value.0.name);
//         Structure::Root((
//             Maybe(collider_request),
//             Maybe(visuals),
//             Visibility::Visible,
//             TransformSchemaAlignRequest(trans, SchemaKind::GLTF),
//         ))
//     }
// }

// pub type GltfPhysicsModel = GltfModel<true>;
// pub type GltfVisualModel = GltfModel<false>;

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

#[derive(Default)]
pub struct PhysicsProperties {
    pub colliders: Vec<ColliderFlag>,
}

/// parse gltf physics extensions into their extension property maps.
pub fn parse_gltf_physics(gltf: &gltf::Gltf) -> Result<PhysicsProperties, String> {
    
    // let error_message = &gltf.document.as_json();
    let Some(external_extensions) = gltf.extensions() else {
        return Err("gltf external extensions evaluated to none".to_owned())
    };
    // let Some((khr_physics_rigid_bodies_key, khr_physics_rigid_bodies_values)) = external_extensions.iter()
    // .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES)) else {
    //     return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
    // };
    let Some((_, khr_implicit_shapes_values)) = external_extensions.iter()
    .find(|(n, _)| n.eq_ignore_ascii_case(KHR_IMPLICIT_SHAPES)) else {
        return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
    };
    let khr_implict_shapes_extension_props = serde_json::from_value::<KHRImplicitShapesMap>(khr_implicit_shapes_values.clone()).unwrap();

    // println!("khr implicit shapes are: {:#?}", khr_implicit_shapes);
    // println!("khr implicit shape values of type {:#?}", khr_implicit_shapes_values);

    // let khr_physics_extension_props = serde_json::from_value(khr_physics_rigid_bodies_values.clone());

    // println!("khr physics is: {:#?}", khr_physics_extension_props);
    // println!("khr physics values of type {:#?}", khr_physics_rigid_bodies_values);

    let nodes = gltf.nodes();


    let mut colliders = Vec::new();

    for node in nodes {
        let Some(node_properties) = node.extensions() else {
            continue;
        };
        let Some((key, values)) = node_properties.iter()
        .find(|(n, val)| n.eq_ignore_ascii_case(KHR_PHYSICS_RIGID_BODIES)) else {
            warn!("node: {:#?} does not have a rigid_body extension", node.name());
            continue;
            // return Err(format!("could not find khr_physics_rigid_bodies in extensions: {:#?}", external_extensions))
        };
        let khr_physics_node_prop = serde_json::from_value::<KHRPhysicsRigidBodiesNodeProp>(values.clone()).unwrap();
        
        let collider = khr_implict_shapes_extension_props.shapes.iter().nth(khr_physics_node_prop.collider.geometry.shape_index as usize).unwrap();

        let collider: MeshPrefab = match collider {
            Shape::Box(box_shape) => Cuboid::from_size(Vec3::from(box_shape.size.size.map(|n| n as f32))).into(),
            Shape::Cylinder(cylinder_shape) => {
                warn!("khr_physics_rigid_body cylinder -> core wrapper conversion not 1:1. These may desync");
                Cylinder::new(cylinder_shape.dimensions.radius_top as f32, cylinder_shape.dimensions.height as f32).into()
            },
        };
        let collider = ColliderFlag::Prefab(collider);
        colliders.push(collider);
        
        // println!("khr node props are: {:#?}", khr_physics_node_prop);
        // println!("khr node prop values are {:#?}", values);
        
        // println!("node extensions for {:#} are {:#?}", node.name().unwrap_or_default(), extensions);
    }
    Ok(
        PhysicsProperties { colliders: colliders }
    )

    // Ok((khr_physics_extension_props.unwrap_or_default(), khr_implict_shapes_extension_props))

}

#[derive(Component, Default)]
pub struct RootNode {
    handle: Handle<GltfNode>,
}


/// Registry for pyhsics properties for a given gltf.
/// [`Gltf`] does not hold handles to non-intrinsic extensions. This is where gltf physics are accessed.
pub struct PhysicsPropertyRegistry {
    physics: HashMap<Handle<Gltf>, PhysicsProperties> 
}

impl Disassemble for GltfModel {
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
        //let nodes = value.nodes;

        println!("scenes: {:#?}", value.scenes);
        println!("nodes: {:#?}", value.nodes);
        //println!("gltf file: {:#?}", value.source);
        

        let mut name = "".to_owned();
        
        
        //let mut gltf_properties = GltfMore::default();
        if let Some(gltf) = &value.source {
            //gltf_properties.physics = parse_gltf_physics(gltf).unwrap();
            
            let root_node = gltf.nodes().find(|n| n.children().len() > 0).unwrap();
            name = root_node.name().unwrap_or_default().to_owned();

        } else {
            warn!("gltf loaded without source. Cannot parse extensions. Offending gltf nodes: {:#?}", value.named_nodes)
        }
        
        //println!("gltf extensions: {:#?}", value.source)
        Structure::Root(
            (
                Name::new(name),
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

/// request to re-align geoemtry to match bevy.
/// TODO: replace with normal Mesh3d if gltf mesh loading is improved to not have this done at the [`Gltf`] level.
#[derive(Component)]
pub struct Mesh3dAlignmentRequest(pub Handle<Mesh>, pub SchemaKind);

impl Disassemble for GltfPrimitiveWrapper {
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, settings: DisassembleSettings) -> Structure<impl Bundle> {
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
    fn components(value: &Self, _settings: DisassembleSettings) -> Structure<impl Bundle> {
        let mesh = value.0.mesh.clone().map(|n| {
            DisassembleAssetRequest(
                DisassembleStage::Handle::<GltfMeshWrapper>(n),
                DisassembleSettings::default(),
            )
        });

        Structure::Root((Maybe(mesh),))
    }
}

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
