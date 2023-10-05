use bevy::prelude::*;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::mesh::VertexAttributeValues;

#[derive(Debug, Reflect, Clone)]

pub struct MeshAttributeDataWrapper {
    attribute: MeshVertexAttribute,
    values: VertexAttributeValues,
}

#[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct MeshVertexAttributeIdWrapper(usize);

#[derive(Reflect, Clone, Default)]
pub enum PrimitiveTopologyWrapper {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    #[default]
    TriangleList = 3,
    TriangleStrip = 4,
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct GeometryFlag{
    primitive_topology: PrimitiveTopologyWrapper
    attributes: BtreeMap<MeshVertexAttributeIdWrapper, 
    // Primitive(MeshPrimitive),
    // Mesh {
    //     filename: String,
    //     scale: Option<Vec3>,
    // },
}


impl From<&GeometryFlag> for Mesh {
    fn from(value: &GeometryFlag) -> Self {
        Self {
            base_color: value.color,
            ..default()
        }
    }
}

impl From<&Mesh> for GeometryFlag {
    fn from(value: &Mesh) -> Self {
        Self {
            color: value.base_color,
            ..default()
        }
    }
}