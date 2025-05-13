
<div align="center">
    <h1> bevy_serialization_extras </h1>
    <h3> 
    A library for component oriented serialization. 

</div>
    <!-- A library that allows the editing of non-reflect components via wrapper components, and serialization of these components via  -->

## Features

- Out of the box serialization Through [`plugins`] for components

```Rust
// Component <-> WrapperComponent
.add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())

// Asset <-> WrapperComponent
.add_plugins(SerializeAssetFor::<MeshMaterial3d<StandardMaterial>, MaterialFlag3d>::default())
// Query -> Component, 
.add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
```

- Serialization of groups of enities that compose an asset into their singular asset equivillent, and vice-versa

### A visualization util to list serializable/unserializable components(W.I.P) [bevy_synonomize]

[demo_gif.webm](https://github.com/rydb/bevy_serialization_extras/assets/43288084/3bda45f1-c75a-437b-a02d-27e58bd3276e)

### Visualize and edit 3rd party components that do not [`Reflect`]
#### E.G: Edit Rapier's `ImpulseJoint` through `JointFlag` [bevy_synonyms_physics]
![edit_example.png](edit_example.png)

### Serialize a collection of entities into an [`Asset`] that is composed of them
#### E.G: serialize the parts of a robot into a [`Urdf`] [bevy_assemble]

```Rust
//(entity_0, ... entity_n) -> Asset
// [UNIMPLEMENTED] Asset -> (entity_0, ... entity_n)
.add_plugins(SerializeManyAsOneFor::<LinkQuery, Urdf>::default())
```

https://github.com/user-attachments/assets/fb1a1b09-db3f-4476-9b0d-800b296ccb8a


## Why bevy_serialization_extras?

- bevy_serialization_extras is built ontop of `bevy_reflect`, not serde. No need to double dip to serialize.

- bevy_serialization_extras allows regular serialization into .ron via [`moonshine_save`] + allows converting serializables into more stable file formats via bevy_assemble. 

serde serialization: 
> world <--> scene.json

bevy_serialziation_extras:
```
(object) <--> (partA, partB) <--> .file
(person) (body + arm_0...arm_n + leg_0..leg_n) <--> .file
world <--> scene.ron
```

This is good for creating editors in bevy.

## Credits

[`moonshine_save`] for the save/load backend of this library


## Usage

See sub-crate `/examples` files for example usage.
