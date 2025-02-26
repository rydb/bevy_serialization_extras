use bevy_log::warn;
use bevy_math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere};
use bevy_rapier3d::{geometry::ComputedColliderShape, prelude::{AsyncCollider, Collider, ColliderView}};
use bevy_serialization_core::{prelude::mesh::{MeshPrefab, FALLBACK_MESH}, traits::ComponentWrapper};
use strum_macros::EnumIter;

use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;

use super::{
    collisiongroupfilter::CollisionGroupsFlag, continous_collision::CcdFlag,
    solvergroupfilter::SolverGroupsFlag,
};




#[derive(Component, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
#[require(CcdFlag, CollisionGroupsFlag, SolverGroupsFlag)]
pub struct PrimitiveColliderFlag(pub MeshPrefab);

impl From<&PrimitiveColliderFlag> for Collider {
    fn from(value: &PrimitiveColliderFlag) -> Self {
        match value.0 {
            MeshPrefab::Cuboid(cuboid) => Collider::cuboid(cuboid.half_size.x, cuboid.half_size.y, cuboid.half_size.z),
            MeshPrefab::Cylinder(cylinder) => Collider::cylinder(cylinder.half_height, cylinder.radius),
            MeshPrefab::Capsule(capsule3d) => {
                //TODO: double check that is is correct
                Collider::capsule_y(capsule3d.half_length, capsule3d.radius)
            },
            MeshPrefab::Sphere(sphere) => Collider::ball(sphere.radius),
            MeshPrefab::Unimplemented => {
                warn!("Attempted to convert unimplemented shape to collider. Using fallback instead.");
                
                // Fallback mesh is a cuboid as the (more accurate) alternative would be performance dropping to 0.1fps from a dozen thosand face trimesh collider.
                Collider::cuboid(FALLBACK_MESH.half_size.x, FALLBACK_MESH.half_size.z, FALLBACK_MESH.half_size.z)
            },
            MeshPrefab::Cone(cone) => Collider::cone(cone.height * 0.5, cone.radius),
        }
    }
}

impl From<&Collider> for PrimitiveColliderFlag {
    fn from(value: &Collider) -> Self {
        
        let collider = value.as_unscaled_typed_shape();
        //TODO: Implement unimplemented collider types.
        match collider {
            ColliderView::Ball(ball_view) => Self(Sphere::new(ball_view.radius()).into()),
            ColliderView::Cuboid(cuboid_view) => Self(Cuboid::from_size(cuboid_view.half_extents()).into()),
            ColliderView::Capsule(capsule_view) => Self(Capsule3d::new(capsule_view.radius(), capsule_view.height()).into()),
            ColliderView::Segment(_segment_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Triangle(_triangle_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::TriMesh(_tri_mesh_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Polyline(_polyline_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::HalfSpace(_half_space_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::HeightField(_height_field_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Compound(_compound_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::ConvexPolyhedron(_convex_polyhedron_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Cylinder(cylinder_view) => Self(Cylinder::new(cylinder_view.radius(), cylinder_view.half_height() * 2.0).into()),
            ColliderView::Cone(cone_view) => Self(Cone::new(cone_view.radius(), cone_view.half_height() * 2.0).into()),
            ColliderView::RoundCuboid(_round_cuboid_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundTriangle(_round_triangle_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundCylinder(_round_cylinder_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundCone(_round_cone_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundConvexPolyhedron(_round_convex_polyhedron_view) => Self(MeshPrefab::Unimplemented),
        }
    }
}

#[derive(Component, PartialEq, EnumIter, Reflect, Clone, Default, Debug)]
#[reflect(Component)]
#[require(CcdFlag, CollisionGroupsFlag, SolverGroupsFlag)]
///Wrapper for mesh colliders
pub enum AsyncColliderFlag {
    /// laggy: no-internal geometry(will clip through things)
    Trimesh,
    #[default]
    /// fast: accurate assuming mesh geometry is convex, inaccurate otherwise.
    Convex,
}

impl ComponentWrapper for AsyncColliderFlag {
    type WrapperTarget = AsyncCollider;
}

impl From<&AsyncCollider> for AsyncColliderFlag {
    fn from(_value: &AsyncCollider) -> Self {
        // TODO: implement a way to choose between trimesh and Convex.
        // In meantime, defaulting to convex to minimize lag.
        Self::Convex
    }
}

impl From<&AsyncColliderFlag> for AsyncCollider {
    fn from(value: &AsyncColliderFlag) -> Self {
        match value {
            //TODO: double check this is correct
            AsyncColliderFlag::Trimesh => AsyncCollider::default(),
            //TODO: double check this is correct
            AsyncColliderFlag::Convex => Self(
                ComputedColliderShape::ConvexHull,
            ),
        }
    }
}
