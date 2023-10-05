// use bevy::prelude::*;
// use bevy::render::mesh::MeshVertexAttribute;
// use bevy::render::mesh::VertexAttributeValues;
// use bevy::utils::HashMap;
// use std::{collections::BTreeMap, hash::Hash, iter::FusedIterator};

// #[derive(Debug, Reflect, Clone)]
// pub enum VertexFormatWrapper {
//     /// Two unsigned bytes (u8). `uvec2` in shaders.
//     Uint8x2 = 0,
//     /// Four unsigned bytes (u8). `uvec4` in shaders.
//     Uint8x4 = 1,
//     /// Two signed bytes (i8). `ivec2` in shaders.
//     Sint8x2 = 2,
//     /// Four signed bytes (i8). `ivec4` in shaders.
//     Sint8x4 = 3,
//     /// Two unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec2` in shaders.
//     Unorm8x2 = 4,
//     /// Four unsigned bytes (u8). [0, 255] converted to float [0, 1] `vec4` in shaders.
//     Unorm8x4 = 5,
//     /// Two signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec2` in shaders.
//     Snorm8x2 = 6,
//     /// Four signed bytes (i8). [-127, 127] converted to float [-1, 1] `vec4` in shaders.
//     Snorm8x4 = 7,
//     /// Two unsigned shorts (u16). `uvec2` in shaders.
//     Uint16x2 = 8,
//     /// Four unsigned shorts (u16). `uvec4` in shaders.
//     Uint16x4 = 9,
//     /// Two signed shorts (i16). `ivec2` in shaders.
//     Sint16x2 = 10,
//     /// Four signed shorts (i16). `ivec4` in shaders.
//     Sint16x4 = 11,
//     /// Two unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec2` in shaders.
//     Unorm16x2 = 12,
//     /// Four unsigned shorts (u16). [0, 65535] converted to float [0, 1] `vec4` in shaders.
//     Unorm16x4 = 13,
//     /// Two signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec2` in shaders.
//     Snorm16x2 = 14,
//     /// Four signed shorts (i16). [-32767, 32767] converted to float [-1, 1] `vec4` in shaders.
//     Snorm16x4 = 15,
//     /// Two half-precision floats (no Rust equiv). `vec2` in shaders.
//     Float16x2 = 16,
//     /// Four half-precision floats (no Rust equiv). `vec4` in shaders.
//     Float16x4 = 17,
//     /// One single-precision float (f32). `float` in shaders.
//     Float32 = 18,
//     /// Two single-precision floats (f32). `vec2` in shaders.
//     Float32x2 = 19,
//     /// Three single-precision floats (f32). `vec3` in shaders.
//     Float32x3 = 20,
//     /// Four single-precision floats (f32). `vec4` in shaders.
//     Float32x4 = 21,
//     /// One unsigned int (u32). `uint` in shaders.
//     Uint32 = 22,
//     /// Two unsigned ints (u32). `uvec2` in shaders.
//     Uint32x2 = 23,
//     /// Three unsigned ints (u32). `uvec3` in shaders.
//     Uint32x3 = 24,
//     /// Four unsigned ints (u32). `uvec4` in shaders.
//     Uint32x4 = 25,
//     /// One signed int (i32). `int` in shaders.
//     Sint32 = 26,
//     /// Two signed ints (i32). `ivec2` in shaders.
//     Sint32x2 = 27,
//     /// Three signed ints (i32). `ivec3` in shaders.
//     Sint32x3 = 28,
//     /// Four signed ints (i32). `ivec4` in shaders.
//     Sint32x4 = 29,
//     /// One double-precision float (f64). `double` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
//     Float64 = 30,
//     /// Two double-precision floats (f64). `dvec2` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
//     Float64x2 = 31,
//     /// Three double-precision floats (f64). `dvec3` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
//     Float64x3 = 32,
//     /// Four double-precision floats (f64). `dvec4` in shaders. Requires [`Features::VERTEX_ATTRIBUTE_64BIT`].
//     Float64x4 = 33,
// }

// #[derive(Debug, Reflect, Clone)]
// pub enum VertexAttributeValuesWrapper {
//     Float32(Vec<f32>),
//     Sint32(Vec<i32>),
//     Uint32(Vec<u32>),
//     Float32x2(Vec<[f32; 2]>),
//     Sint32x2(Vec<[i32; 2]>),
//     Uint32x2(Vec<[u32; 2]>),
//     Float32x3(Vec<[f32; 3]>),
//     Sint32x3(Vec<[i32; 3]>),
//     Uint32x3(Vec<[u32; 3]>),
//     Float32x4(Vec<[f32; 4]>),
//     Sint32x4(Vec<[i32; 4]>),
//     Uint32x4(Vec<[u32; 4]>),
//     Sint16x2(Vec<[i16; 2]>),
//     Snorm16x2(Vec<[i16; 2]>),
//     Uint16x2(Vec<[u16; 2]>),
//     Unorm16x2(Vec<[u16; 2]>),
//     Sint16x4(Vec<[i16; 4]>),
//     Snorm16x4(Vec<[i16; 4]>),
//     Uint16x4(Vec<[u16; 4]>),
//     Unorm16x4(Vec<[u16; 4]>),
//     Sint8x2(Vec<[i8; 2]>),
//     Snorm8x2(Vec<[i8; 2]>),
//     Uint8x2(Vec<[u8; 2]>),
//     Unorm8x2(Vec<[u8; 2]>),
//     Sint8x4(Vec<[i8; 4]>),
//     Snorm8x4(Vec<[i8; 4]>),
//     Uint8x4(Vec<[u8; 4]>),
//     Unorm8x4(Vec<[u8; 4]>),
// }


// #[derive(Debug, Clone, Reflect)]
// pub struct MeshVertexAttributeWrapper {
//     /// The friendly name of the vertex attribute
//     //pub name: &'static str,

//     /// The _unique_ id of the vertex attribute. This will also determine sort ordering
//     /// when generating vertex buffers. Built-in / standard attributes will use "close to zero"
//     /// indices. When in doubt, use a random / very large usize to avoid conflicts.
//     pub id: MeshVertexAttributeIdWrapper,

//     /// The format of the vertex attribute.
//     pub format: VertexFormatWrapper,
// }


// #[derive(Debug, Reflect, Clone)]

// pub struct MeshAttributeDataWrapper {
//     attribute: MeshVertexAttributeWrapper,
//     values: VertexAttributeValuesWrapper,
// }

// #[derive(Debug, Reflect, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
// pub struct MeshVertexAttributeIdWrapper(usize);

// #[derive(Reflect, Clone, Default)]
// pub enum PrimitiveTopologyWrapper {
//     PointList = 0,
//     LineList = 1,
//     LineStrip = 2,
//     #[default]
//     TriangleList = 3,
//     TriangleStrip = 4,
// }

// #[derive(Component, Reflect, Clone, Default)]
// #[reflect(Component)]
// pub struct GeometryFlag{
//     primitive_topology: PrimitiveTopologyWrapper,
//     attributes: HashMap<MeshVertexAttributeIdWrapper, MeshAttributeDataWrapper>, 
//     indicies: Option<Indicies>,
//     // Primitive(MeshPrimitive),
//     // Mesh {
//     //     filename: String,
//     //     scale: Option<Vec3>,
//     // },
// }


// impl From<&GeometryFlag> for Mesh {
//     fn from(value: &GeometryFlag) -> Self {
//         Self {
//             base_color: value.color,
//             ..default()
//         }
//     }
// }

// impl From<&Mesh> for GeometryFlag {
//     fn from(value: &Mesh) -> Self {
//         Self {
//             color: value.base_color,
//             ..default()
//         }
//     }
// }