use std::cmp;

use bevy_asset::Assets;
use bevy_ecs::prelude::*;
use bevy_hierarchy::Children;
use bevy_log::warn;
use bevy_math::primitives::{Cuboid, Sphere};
use bevy_rapier3d::prelude::{AsyncCollider, ComputedColliderShape};
use bevy_render::{prelude::*};
use bevy_serialization_core::prelude::mesh::MeshPrefab;
use derive_more::From;
use glam::Vec3;
use rapier3d::parry::either::Either;

use crate::prelude::{ColliderFlag, RequestCollider, RequestColliderFromChildren};

#[derive(Clone, Copy)]
pub struct FarthestPoints {
    pub positive: Vec3,
    pub negative: Vec3,
}

pub fn farthest_points(positions: &[[f32; 3]]) -> FarthestPoints{
    let mut farthest_x_positive = 0.0;
    let mut farthest_x_negative = 0.0;

    let mut farthest_y_positive = 0.0;
    let mut farthest_y_negative = 0.0;

    let mut farthest_z_positive = 0.0;
    let mut farthest_z_negative = 0.0;

    for position in positions {
        let x = position[0];
        let y = position[1];
        let z = position[2];
        if x > farthest_x_positive {
            farthest_x_positive = x;
        }
        if x < farthest_x_negative {
            farthest_x_negative = x;
        }

        if y > farthest_y_positive {
            farthest_y_positive = y;
        }
        if y < farthest_y_negative {
            farthest_y_negative = y;
        }

        if z > farthest_z_positive {
            farthest_z_positive = z;
        }
        if z < farthest_z_negative {
            farthest_z_negative = z;
        }
    }
    FarthestPoints {
        positive: Vec3::new(farthest_x_positive, farthest_y_positive, farthest_z_positive),
        negative: Vec3::new(farthest_x_negative, farthest_y_negative, farthest_z_negative),
    }

}

pub fn collider_from_farthest_points(request_kind: &RequestCollider, farthest_points: FarthestPoints) -> Either<ColliderFlag, AsyncCollider> {
    match request_kind {
        RequestCollider::Cuboid => {
            let half_size = Vec3 {
                x: (f32::abs(farthest_points.positive.x) + farthest_points.positive.x),
                y: (f32::abs(farthest_points.negative.y) + farthest_points.positive.y),
                z: (f32::abs(farthest_points.negative.z) + farthest_points.positive.z),
            };
            let collider = Cuboid { half_size };
            Either::Left(ColliderFlag::Prefab(MeshPrefab::from(collider)))
        }
        //TODO: Until: https://github.com/dimforge/rapier/issues/778 is resolved
        //This solution uses the sphere method for generating a primitive.
        RequestCollider::Wheel => {
            let mut largest = 0.0;
            for candidate in [
                farthest_points.positive.x,
                f32::abs(farthest_points.negative.x),
                farthest_points.positive.y,
                f32::abs(farthest_points.negative.y),
                farthest_points.positive.z,
                f32::abs(farthest_points.negative.z),
            ] {
                if candidate > largest {
                    largest = candidate;
                }
            }
            Either::Left(ColliderFlag::Prefab(MeshPrefab::Sphere(Sphere::new(largest))))
        }
        RequestCollider::Convex => {
            Either::Right(AsyncCollider(ComputedColliderShape::ConvexHull))
        }
        RequestCollider::Sphere => {
            let mut largest = 0.0;
            for candidate in [
                farthest_points.positive.x,
                f32::abs(farthest_points.negative.x),
                farthest_points.positive.y,
                f32::abs(farthest_points.negative.y),
                farthest_points.positive.z,
                f32::abs(farthest_points.negative.z),
            ] {
                if candidate > largest {
                    largest = candidate;
                }
            }
            Either::Left(ColliderFlag::Prefab(MeshPrefab::Sphere(Sphere::new(largest))))
        }
    }
}

pub fn generate_collider_from_children(
    requests: Query<(Entity, &RequestColliderFromChildren, &Children)>,
    meshes: Query<&Mesh3d>,
    mesh_assets: Res<Assets<Mesh>>,
    mut commands: Commands,
) {
    let mut points = FarthestPoints {
        positive: Vec3::ZERO,
        negative: Vec3::ZERO
    };
    
    for (e, request, children) in &requests {
        for child in children {
            let Ok(mesh) = meshes.get(*child) else {
                continue;
            };
            let Some(mesh) = mesh_assets.get(&mesh.0) else {
                // If a mesh is still loading, don't try to approximate the collider until after its loaded!
                return;
            };
            let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
                warn!("Expected positions. Skipping {:#}", child );
                continue;
            };
            let Some(positions) = positions.as_float3() else {
                warn!("Expected positions ot be float3. Skipping {:#}", child );
                continue;
            };
            let farthest_points = farthest_points(positions);
            points.positive = points.positive.max(farthest_points.positive);
            points.negative = points.negative.max(farthest_points.negative);
        }
        let collider = collider_from_farthest_points(&request.0, points);
        match collider {
            Either::Left(n) => commands.entity(e).insert(n),
            Either::Right(n) => commands.entity(e).insert(n),
        };
        commands.entity(e).remove::<RequestColliderFromChildren>();

    }
}

/// generate a collider primitive from a primitive request
pub fn generate_primitive_for_request(
    requests: Query<(Entity, &RequestCollider, &Mesh3d)>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
) {
    for (e, collider, mesh) in requests.iter() {
        let Some(mesh) = meshes.get(&mesh.0) else {
            return;
        };
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            warn!("Expected positions. Exiting");
            return;
        };
        let Some(positions) = positions.as_float3() else {
            warn!("Expected positions ot be float3. Exiting");
            return;
        };

        let farthest_points = farthest_points(positions);

        let collider = collider_from_farthest_points(collider, farthest_points);
        match collider {
            Either::Left(n) => commands.entity(e).insert(n),
            Either::Right(n) => commands.entity(e).insert(n),
        };
        commands.entity(e).remove::<RequestCollider>();
    }
}