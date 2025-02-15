use bevy_log::warn;
use bevy_math::primitives::{Capsule3d, Cone, Cuboid, Cylinder, Sphere};
use bevy_rapier3d::{geometry::ComputedColliderShape, math::Vect, prelude::{AsyncCollider, Collider, ColliderView, VHACDParameters}};
use bevy_render::mesh::Meshable;
use bevy_serialization_core::prelude::mesh::{MeshPrefab, FALLBACK_MESH};
use derive_more::derive::From;
use rapier3d::prelude::{SharedShape, TriMeshFlags};
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
/// Wrapper for primitive colliders
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
            ColliderView::Segment(segment_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Triangle(triangle_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::TriMesh(tri_mesh_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Polyline(polyline_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::HalfSpace(half_space_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::HeightField(height_field_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Compound(compound_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::ConvexPolyhedron(convex_polyhedron_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::Cylinder(cylinder_view) => Self(Cylinder::new(cylinder_view.radius(), cylinder_view.half_height() * 2.0).into()),
            ColliderView::Cone(cone_view) => Self(Cone::new(cone_view.radius(), cone_view.half_height() * 2.0).into()),
            ColliderView::RoundCuboid(round_cuboid_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundTriangle(round_triangle_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundCylinder(round_cylinder_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundCone(round_cone_view) => Self(MeshPrefab::Unimplemented),
            ColliderView::RoundConvexPolyhedron(round_convex_polyhedron_view) => Self(MeshPrefab::Unimplemented),
        }
    }
}

#[derive(Component, EnumIter, Reflect, Clone, Default, Debug)]
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

impl From<&AsyncCollider> for AsyncColliderFlag {
    fn from(value: &AsyncCollider) -> Self {
        // TODO: implement a way to choose between trimesh and Convex.
        // In meantime, defaulting to convex to minimize lag.
        Self::Convex
    }
}

impl From<&AsyncColliderFlag> for AsyncCollider {
    fn from(value: &AsyncColliderFlag) -> Self {
        match value {
            //TODO: double check this is correct
            AsyncColliderFlag::Trimesh => Self(ComputedColliderShape::TriMesh(TriMeshFlags::default())),
            //TODO: double check this is correct
            AsyncColliderFlag::Convex => Self(ComputedColliderShape::ConvexDecomposition(VHACDParameters::default())),
        }
    }
}
