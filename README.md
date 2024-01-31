# bevy_serialization_extras

A library for bevy which contains wrappers/systems for managing serialization in bevy with [`moonshine-save`](https://github.com/Zeenobit/moonshine_save) for a more "hands off" serialization workflow

[demo_gif.webm](https://github.com/rydb/bevy_serialization_extras/assets/43288084/3bda45f1-c75a-437b-a02d-27e58bd3276e)

## Features

- Out of the box serialization Through [`plugins`] for components

```Rust
// Component <-> WrapperComponent
.add_plugins(SerializeComponentFor::<AsyncCollider, ColliderFlag>::default())

// AssetHandle <-> WrapperComponent
.add_plugins(SerializeAssetFor::<StandardMaterial, MaterialFlag>::default())

// WrapperComponent -> AssetHandle
.add_plugins(DeserializeAssetFrom::<GeometryFlag, Mesh>::default())

// Query -> Component, 
.add_plugins(SerializeQueryFor::<Linkage, ImpulseJoint, JointFlag>::default())
```

- Serialization of groups of enities that compose an asset into their singular asset equivillent, and vice-versa

See: <https://github.com/rydb/bevy_serialization_urdf>
```Rust
//Query <-> Asset
.add_plugins(SerializeManyAsOneFor::<LinkQuery, Urdf>::default())
```

- Allows edititing unserializable(non-reflect) implementing components through the wrappers that convert to them

    E.G: Edit Rapier's `ImpulseJoint` through `JointFlag`

![edit_example.png](edit_example.png)

- type registration for wrappers through `ManagedTypeRegistration` trait

- A visualization util to list serializable/unserializable components(W.I.P)

## Usage

For implementations of plugin trait bounds, see `/Wrappers`

### TODO

- Add a mechanism for updating from old save versions to new save versions as to not break old save files.
