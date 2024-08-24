 

# Material Overrides Plugin for Bevy

This Rust crate provides functionality for loading materials from a `.glb` file and additional material configuration data from a separate file. It supports hot-swapping materials in a Bevy scene using two custom components: `MaterialOverrideComponent` and `MaterialOverrideWhenSceneReadyComponent`.

## Features

- **Material Loading**: Automatically load materials from a specified `.glb` file and apply additional configurations, such as UV scaling and color tinting, from a separate configuration file.
- **Hot-Swapping**: Dynamically swap materials in the scene using the `MaterialOverrideComponent` and `MaterialOverrideWhenSceneReadyComponent`.
- **Flexible Configuration**: Customize materials using a configuration file that allows for UV scale adjustments and diffuse color tinting.

## Getting Started

### 1. Installation

To use this crate, add it to your `Cargo.toml`:

```toml
[dependencies]
your_crate_name = "0.1.0"
bevy = "0.11"
serde = "1.0"
```

### 2. Usage

#### Setting Up the Plugin

First, set up the `material_overrides_plugin` in your Bevy application:

```rust
use bevy::prelude::*;
use your_crate_name::material_overrides_plugin;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(material_overrides_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(YourSceneBundle::default());
    // Add additional setup as needed
}
```

#### Configuring Material Overrides

The plugin requires two components to manage material overrides:

1. **`MaterialOverrideComponent`**: Attach this component to entities that require material replacement.
   
   ```rust
   commands.entity(entity)
       .insert(MaterialOverrideComponent {
           material_override: "Wall1".to_string(),
       });
   ```

2. **`MaterialOverrideWhenSceneReadyComponent`**: Use this component to defer material overrides until the scene is ready.

   ```rust
   commands.entity(entity)
       .insert(MaterialOverrideWhenSceneReadyComponent {
           material_override: "StoneToon1".to_string(),
       });
   ```

#### Loading Materials

The materials are loaded from a `.glb` file specified in the `MaterialOverridesResource`:

```rust
commands.insert_resource(MaterialOverridesResource {
    doodad_materials_gltf_path: "assets/materials/doodad_materials.glb".to_string(),
    material_types_config_path: "assets/config/material_types_config.ron".to_string(),
    doodad_materials_gltf: None,
    extracted_materials_map: HashMap::new(),
});
```

### 3. Configuration File

The material configuration file is a RON (Rusty Object Notation) file that defines the material types and their properties, such as UV scale and color tint.

Example `material_types_config.ron`:

```ron
(
  material_types: {
    "Wall1": (
      material_name: "Wall1",
      uv_scale_factor: 16.0,
      diffuse_color_tint: Some(LinearRgba(
        red: 1.0,
        green: 1.0,
        blue: 1.0,
        alpha: 1.0,
      )),
    ),
    "StoneToon1": (
      material_name: "StoneToon1",
      uv_scale_factor: 2.0,
    ),
    "StoneToon2": (
      material_name: "StoneToon2",
      uv_scale_factor: 2.0,
    ),
    "DirtToon1": (
      material_name: "DirtToon1",
      uv_scale_factor: 2.0,
    ),
    "TiledWall1": (
      material_name: "Wall1",
      uv_scale_factor: 1.0,
    ),
  }
)
```

### 4. Advanced Usage

#### MaterialOverridesLoadingState

The crate defines a state machine to manage material loading and processing:

- **`Init`**: Initial state.
- **`Extracting`**: Extracts materials from the `.glb` file.
- **`Building`**: Builds materials based on the configuration file.
- **`Complete`**: All materials are loaded and ready.

You can customize or extend these states for more complex material management workflows.

### 5. Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

### 6. License

The code in this project is licensed under the MIT License.  Texture/material assets are not included in this license.  See the [LICENSE](LICENSE) file for details.

 