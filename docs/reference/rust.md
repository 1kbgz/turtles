# Rust API

The native library is the `turtles` crate. Its public modules and root
re-exports are defined in the
[crate root](https://github.com/1kbgz/turtles/blob/main/rust/src/lib.rs).
This page maps those entry points, configuration types, geometry accessors,
and export behavior. Generated rustdoc provides the complete signatures,
fields, trait implementations, and method lists.

The Python API is a separate binding and wrapper layer. Its exposed methods,
defaults, copying behavior, and properties are documented in
[Patterns](patterns.md), [Composition](composition.md), and
[Rose engine](rose-engine.md); a public Rust method is not automatically a
Python method.

## Packages and manifests

| Manifest                                                                          | Package and library                                                                | Role                                                                                                                      |
| --------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| [Core manifest](https://github.com/1kbgz/turtles/blob/main/rust/Cargo.toml)       | Package `turtles`, library `turtles`, edition 2021, `rlib`                         | Native Rust API, implemented under the core crate's source directory. Depends on `svg`, `stl_io`, and `chrono`, not PyO3. |
| [Repository-root manifest](https://github.com/1kbgz/turtles/blob/main/Cargo.toml) | Package `turtles_py`, library `turtles`, edition 2021, `cdylib`, `publish = false` | PyO3 extension with its entry point in the binding module. Depends on the core package through `path = "./rust"`.         |

These are separate Cargo packages, **not a Cargo workspace**. Both library
targets are named `turtles`, but the root target builds the Python extension;
the nested manifest selects the native `rlib`. A Rust consumer's dependency
must point to the core package directory, not the repository-root package.
For a local checkout, the dependency's `path` is the checkout's `rust`
subdirectory.

From the repository root, `cargo doc --manifest-path rust/Cargo.toml --no-deps`
builds the native API documentation. This command selects the core crate
explicitly; a root-only documentation build is not its replacement. Build,
test, and binding setup commands are in
[How to develop and check changes](../how-to/develop.md).

## Public modules and root re-exports

Every module in this table is public. Listed root names can be imported as
`turtles::Name` as well as through their defining module.

| Module           | Names re-exported at the crate root                                                                                                                                                                                                                |
| ---------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `common`         | `Point2D`, `Point3D`, `SpirographError`, `ExportConfig`, `clock_to_cartesian`, `polar_to_cartesian`, `validate_finite`, `validate_positive`, `validate_non_negative`, `validate_range`, `validate_radius`, `checked_element_count`, `MAX_ELEMENTS` |
| `clous_de_paris` | `ClousDeParisConfig`, `ClousDeParisLayer`                                                                                                                                                                                                          |
| `cube`           | `CubeConfig`, `CubeLayer`                                                                                                                                                                                                                          |
| `diamant`        | `DiamantConfig`, `DiamantLayer`                                                                                                                                                                                                                    |
| `draperie`       | `DraperieConfig`, `DraperieLayer`                                                                                                                                                                                                                  |
| `flinque`        | `FlinqueConfig`, `FlinqueLayer`                                                                                                                                                                                                                    |
| `guilloche`      | `GuillochePattern`                                                                                                                                                                                                                                 |
| `huiteight`      | `HuitEightConfig`, `HuitEightLayer`                                                                                                                                                                                                                |
| `limacon`        | `LimaconConfig`, `LimaconLayer`                                                                                                                                                                                                                    |
| `paon`           | `PaonConfig`, `PaonLayer`, `paon_wave_fn`                                                                                                                                                                                                          |
| `spirograph`     | `HorizontalSpirograph`, `VerticalSpirograph`, `SphericalSpirograph`                                                                                                                                                                                |
| `rose_engine`    | `Arc`, `BitShape`, `CuttingBit`, `RenderedOutput`, `RoseEngineConfig`, `RoseEngineLathe`, `RoseEngineLatheRun`, `RosettePattern`, `ToolPathOutput`                                                                                                 |
| `watch_face`     | `BezelConfig`, `DialConfig`, `HoleConfig`, `WatchFace`                                                                                                                                                                                             |
| `step`           | No root re-export; the public writer is `turtles::step::write_step_polylines`.                                                                                                                                                                     |

Additional public items available through module paths include:

- `turtles::common::compute_bounds`, `clip_polyline_to_circle`, and
  `clip_lines_to_radius`. They are not root re-exports.
- `turtles::guilloche::SpirographLayer`, with `Horizontal`, `Vertical`, and
  `Spherical` variants, and `turtles::guilloche::GuillocheLayer`, with
  `Spirograph` and `Flinque` variants. `GuillocheLayer` is not an enum of all
  decorative families; the compositor stores those families separately.
- Public rose-engine submodules `config`, `cutting_bit`, `lathe`, `lathe_run`,
  and `rosette`. Their principal types are also re-exported by `rose_engine`
  and the crate root.
- Common point, export-config, error, clock-conversion, and radius-validation
  names re-exported by `spirograph` for compatibility. Internal spirograph SVG,
  STL, and STEP adapter modules are private; their public interface is the
  spirograph methods.

## Construction and generation

Validated constructors return `Result<Self, SpirographError>` and do not
generate geometry. Config factories generally return an unchecked config;
validation occurs when a layer or generator is constructed.

| Family                                       | Native construction                                                                                                                                                                                                      |
| -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Seven decorative families other than flinqué | `Layer::new(config)`, `new_with_center(config, center_x, center_y)`, `new_at_polar(config, angle, distance)`, and `new_at_clock(config, hour, minute, distance)`, with the matching family config type.                  |
| Flinqué                                      | Same constructor names, but `radius` precedes `FlinqueConfig` and the positioning arguments.                                                                                                                             |
| Horizontal spirograph                        | `new(outer_radius, radius_ratio, point_distance, rotations, resolution)`.                                                                                                                                                |
| Vertical spirograph                          | Same five arguments followed by required `wave_amplitude` and `wave_frequency`.                                                                                                                                          |
| Spherical spirograph                         | Same five arguments followed by required `dome_height`.                                                                                                                                                                  |
| Positioned spirographs                       | `new_with_center`, `new_at_polar`, and `new_at_clock`; variant-specific arguments precede the positioning arguments.                                                                                                     |
| Compositors                                  | `GuillochePattern::new(radius)` and `WatchFace::new(radius)`.                                                                                                                                                            |
| Single rose-engine lathe                     | `RoseEngineLathe::new(config, cutting_bit)` or `new_with_center(config, cutting_bit, center_x, center_y)`.                                                                                                               |
| General rose-engine run                      | `new(config, cutting_bit, num_passes)`, `new_with_center(...)`, or `new_with_segments(config, cutting_bit, num_passes, segments_per_pass, center_x, center_y)`. `new` and `new_with_center` select 24 segments per pass. |
| Run family presets                           | `new_draperie`, `new_paon`, `new_diamant`, `new_limacon`, `new_flinque`, `new_huiteight`, `new_clous_de_paris`, and `new_cube`. These construct their own configs and bits; complete argument lists are in rustdoc.      |

Rust has no optional positional arguments: Python defaults do not supply
missing Rust arguments. Decorative configs are passed by value. Rust layer
adders take ownership of the supplied layer, including any stored points and
center; they do not reconstruct it as the Python bindings do. A subsequent
compositor `generate()` regenerates all its layers.

`generate()` requires mutable access and replaces previous geometry. It returns
`()` for decorative layers, compositors, and rose-engine generators.
Horizontal and vertical spirograph generation returns `&Vec<Point2D>`;
spherical generation returns `&Vec<Point3D>`. Export methods borrow the object
and do not generate automatically.

This complete example constructs a layer from a native config, inspects its
generated geometry, and moves it into a watch face:

```rust
use std::error::Error;
use turtles::{ClousDeParisConfig, ClousDeParisLayer, WatchFace};

fn main() -> Result<(), Box<dyn Error>> {
    let config = ClousDeParisConfig::new(1.0, 10.0).with_resolution(2);
    let mut layer = ClousDeParisLayer::new(config)?;
    layer.generate();
    assert!(!layer.lines().is_empty());

    let mut face = WatchFace::new(38.0)?;
    face.add_inner();
    face.add_clous_de_paris_layer(layer);
    face.generate();
    face.to_svg("dial.svg")?;
    Ok(())
}
```

The face radius is constrained to 26–44 mm inclusive. The standalone layer's
10 mm local radius is valid. Coordinates and lengths use millimeters; polar
angles and grid rotations use radians, while Cube `leg_angle` and V-bit angle
use degrees. Clock placement uses the hour hand, including minutes. See
[Coordinates](../explanation/coordinates.md).

## Native configuration types

All eight decorative config types implement `Default` and expose public
fields. Their default values are native config values, not a promise about a
Python wrapper's convenience defaults.

| Config               | `Default` fields                                                                                                                                                                                                                  |
| -------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `FlinqueConfig`      | `num_petals=12`, `num_waves=60`, `wave_amplitude=0.8`, `wave_frequency=20.0`, `inner_radius_ratio=0.05`. Radius is a separate layer-constructor argument.                                                                         |
| `DiamantConfig`      | `num_circles=72`, `circle_radius=20.0`, `resolution=360`.                                                                                                                                                                         |
| `LimaconConfig`      | `num_curves=72`, `base_radius=20.0`, `amplitude=20.0`, `resolution=360`.                                                                                                                                                          |
| `DraperieConfig`     | `num_rings=96`, `base_radius=22.0`, `radius_step=0.44`, `wave_frequency=12.0`, `amplitude=None`, `phase_shift=PI / 12`, `phase_oscillations=2.5`, `resolution=1500`, `phase_exponent=3`, `wave_exponent=1`, `circular_phase=2.0`. |
| `PaonConfig`         | `num_lines=500`, `radius=22.0`, `amplitude=0.035`, `wave_frequency=10.0`, `phase_rate=9.0`, `resolution=800`, `n_harmonics=3`, `fan_angle=4.0`, `vanishing_point=0.3`.                                                            |
| `HuitEightConfig`    | `num_curves=36`, `scale=20.0`, `resolution=360`, `num_clusters=0`, `cluster_spread=0.0`.                                                                                                                                          |
| `ClousDeParisConfig` | `spacing=1.0`, `radius=22.0`, `angle=PI / 4`, `resolution=200`.                                                                                                                                                                   |
| `CubeConfig`         | `spacing=0.5`, `radius=22.0`, `angle=0.0`, `resolution=200`, `cuts_per_group=8`, `gap_per_group=8`, `amplitude=0.0`, `leg_angle=30.0`.                                                                                            |

The seven non-flinqué configs also expose `new(...)` and consuming
`with_resolution(...) -> Self` builders. `HuitEightConfig` additionally has
`with_clusters(...) -> Self`; `DraperieConfig::safe_amplitude()` returns the
automatic amplitude calculation. Flinqué uses `Default` or a struct literal,
not a `FlinqueConfig::new()` method. Draperie permits an explicit
`Some(amplitude)` in Rust; Python always chooses `None`. Explicit amplitude is
not clamped to the automatic value.

The direct-layer defaults and Python conveniences can differ. For example,
`CubeConfig` defaults to eight cuts/eight gap spacings, while
`WatchFace.add_cube()` in Python defaults to twelve/six. Paon's `fan_angle`
field is phase amplitude in full wave cycles, not angular fan spread.
Cube's `resolution` is retained and validated but does not control its exact
zigzag vertices. See [Pattern models](../explanation/pattern-models.md).

### Rose-engine configuration and bit types

`RoseEngineConfig::new(base_radius, amplitude)` returns an unchecked config.
`Default` calls `new(20.0, 2.0)`. Public fields include the primary and optional
secondary rosettes, their amplitudes/phases, base radius, angular interval,
resolution, and depth-modulation controls. Constructor defaults are:

- `rosette = RosettePattern::MultiLobe { lobes: 12 }`;
  `phase=0.0`, `resolution=1000`, `start_angle=0.0`, `end_angle=2 * PI`.
- `secondary_rosette=None`, `secondary_amplitude=0.0`, `secondary_phase=0.0`.
- `depth_modulation=false`, `depth_modulation_amplitude=0.0`,
  `depth_modulation_frequency=1.0`.

`with_secondary_rosette()` and `with_depth_modulation()` mutate through
`&mut self` and return `()`, unlike the consuming decorative config builders.
`validate()` checks the numeric config subset described in [Errors](errors.md).
`radius_at_angle()` and `depth_at_angle()` evaluate the configured motions.
Config presets are `classic_multi_lobe`, `sunburst`, `wave`, `rose_curve`,
`compound`, `huit_eight`, `grain_de_riz`, `draperie`, and `diamant`; their
factory settings are described in [Rose engine](rose-engine.md).

`RosettePattern` is a public enum with `Circular`, `Elliptical`, `Sinusoidal`,
`MultiLobe`, `Epicycloid`, `HuitEight`, `GrainDeRiz`, `Draperie`, `Paon`,
`Diamant`, and `Custom` variants. Rust can construct variants directly, evaluate
`displacement(angle)`, or sample a closure with `from_function(func, samples)`.
The custom lookup table is periodically interpolated using its actual length;
an empty table returns zero displacement. Rosette parameters are not generally
validated by config validation.

`CuttingBit` exposes `shape`, `width`, and `depth`. `BitShape` has `VShaped`,
`Flat`, `Round`, `Elliptical`, and `Custom` variants. Bit constructors are
`v_shaped`, `flat`, `round`, `elliptical`, and `custom`; `Default` is a
60-degree V-bit of width `0.5` mm. Rust also exposes `cross_section()` and
`footprint()`. The footprint currently returns two single offset points and
ignores its point-count argument; it is not a swept tool model. Bit dimensions
are not generally validated.

### Dial and export configuration

| Config         | Public fields and defaults                                            |
| -------------- | --------------------------------------------------------------------- |
| `DialConfig`   | `fill_color="#fafaf5"`, `stroke_color="#2c2c2c"`, `stroke_width=0.3`. |
| `BezelConfig`  | `radius_ratio=1.05`, `stroke_color="#1a1a1a"`, `stroke_width=0.8`.    |
| `HoleConfig`   | `center_x=0.0`, `center_y=0.0`, `radius=0.8`, `fill_color="#1a1a1a"`. |
| `ExportConfig` | `depth=0.1`, `base_thickness=2.0`, `tool_radius=0.0`, all in mm.      |

These four types implement `Default`. Native `WatchFace` accepts dial/bezel
styles through `add_inner_with_config()` and `add_outer_with_config()`, and
hole configuration through `add_hole()`. Those styling configs are not exposed
by the public Python wrapper. Dial/hole configs and export configs do not
provide a general numeric-validation pass.

## Composition and geometry access

`GuillochePattern` has separate typed adders for all three spirographs and all
eight decorative families. Decorative families also have polar/clock adders;
native positioned spirographs are constructed first and then moved into a
typed spirograph adder. `WatchFace` forwards typed adders for all families and
decorative clock adders. It exposes its compositor as the public `guilloche`
field. Native APIs do not use the Python `WatchFace.add(layer)` dispatcher.

| Object                         | Generated data access                                                                                                                                                                                                                                |
| ------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Point2D`, `Point3D`           | `new(...)` constructors and public `x`, `y`, and, for `Point3D`, `z` fields; coordinates are `f64`.                                                                                                                                                  |
| Decorative layers              | `lines() -> &Vec<Vec<Point2D>>`; Diamant also has `circles()`, Limaçon/Huit-eight `curves()`, and Draperie `rings()` aliases. Layer configs and center coordinates are public; flinqué radius is also public.                                        |
| Horizontal/vertical spirograph | `points() -> &Vec<Point2D>`; parameters and center coordinates are public.                                                                                                                                                                           |
| Spherical spirograph           | `points_2d()` and `points_3d()`; stores both the planar trace and projected dome points.                                                                                                                                                             |
| `GuillochePattern`             | `layer_count()`, `spirograph_points()`, and family accessors such as `flinque_lines()` through `cube_lines()`. `all_paths()` collects owned planar paths; `clipped_paths()` returns them geometrically clipped to the dial radius. Radius is public. |
| `WatchFace`                    | `radius()`, `layer_count()`, and public `guilloche` for native geometry access. Its SVG-rendering helper getters are private.                                                                                                                        |
| `RoseEngineLathe`              | `tool_path() -> &ToolPathOutput`, `rendered_output() -> &RenderedOutput`. Config, cutting bit, and center coordinates are public.                                                                                                                    |
| `RoseEngineLatheRun`           | `lines()`, `passes()`, `num_passes()`, and `generation_errors()`. General run config, bit, pass/segment counts, radius/phase controls, and center coordinates are public.                                                                            |

`ToolPathOutput` has public `center_line`, `cut_edges`, and `arcs` fields.
`Arc` has `center`, `radius`, `start_angle`, and `end_angle`. Current generation
clears `arcs` but does not fit or populate machining arcs. `RenderedOutput`
has `lines`, `depth_map`, and `shading`; the latter two are populated when
depth modulation is enabled, not used by the bound exporters.

Run `lines()` is the retained/segmented output, not an array of passes.
Specialized generation branches can populate it without constructing single
lathe objects in `passes()`. The configured `num_passes` is not a guarantee
about either collection's length. `generation_errors()` records failed
single-pass construction during general generation; it is not exposed in
Python.

Public field mutation does not automatically revalidate, regenerate, or
invalidate stored output. Constructors' guarantees can be bypassed by later
mutation. Rust ownership transfers and explicit `clone()` calls also differ
from the Python bindings' config-copy behavior.

## Native export entry points

| Object                  | Methods                                                                                                                                                                                                                                  |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Eight decorative layers | `to_svg(filename)`; no native standalone STL or STEP methods.                                                                                                                                                                            |
| Three spirographs       | `to_svg(filename)`, `to_stl(filename, &config)`, `to_step(filename, &config)`.                                                                                                                                                           |
| `WatchFace`             | `to_svg(filename)`, `to_stl(filename, &config)`, `to_step(filename, &config)`.                                                                                                                                                           |
| `GuillochePattern`      | `export_combined_svg(filename)`, `export_combined_stl(filename, &config)`, `export_combined_step(filename, &config)`, `export_all(base_name, &config)`. Its native single-format methods are not named `to_svg`, `to_stl`, or `to_step`. |
| `RoseEngineLathe`       | `to_svg(filename)`, `to_stl(filename, &config)`, and exposed but unimplemented `to_step(filename, &config)`.                                                                                                                             |
| `RoseEngineLatheRun`    | `to_svg(filename)` only.                                                                                                                                                                                                                 |
| `step` module           | `write_step_polylines(filename, description, polylines)` accepts borrowed `&[Vec<Point3D>]` and writes an AP214 polyline curve set.                                                                                                      |

Export methods return `Result<(), SpirographError>` and take filename strings
as `&str`. Current STL uses only `ExportConfig.depth`; base thickness and tool
radius are unused. Working STEP adapters ignore the config entirely.
Single-lathe STEP returns `ExportError`, including after generation.
Batch export writes combined SVG, STL, and `.stp` in that order and stops on
the first failure without rollback.

Combined STL/STEP clip planar paths to the dial circle. Standalone spherical
STL/STEP retain the dome; combined exports use its planar trace. SVG
decorations are not plate or hole geometry. STEP contains curves; STL contains
uncapped, zero-width ribbons with placeholder normals, not a watertight or
swept solid. Full contents, closure behavior, dimensions, and preconditions
are in [Exports](exports.md) and
[Geometry and exports](../explanation/geometry-and-exports.md).

## Errors and common helpers

`SpirographError` implements `Debug`, `Clone`, `PartialEq`, `Display`, and
`std::error::Error`. Its variants are `InvalidRadius(String)`,
`InvalidParameter(String)`, and `ExportError(String)`, with corresponding
display prefixes `Invalid radius:`, `Invalid parameter:`, and `Export error:`.
Validated constructors and file exporters propagate these errors through
`Result`; most `generate()` methods do not return `Result`.

`validate_finite`, `validate_positive`, `validate_non_negative`, and
`validate_range` implement the named checks, with inclusive range endpoints.
`validate_radius` adds the finite inclusive 26–44 mm dial rule.
`checked_element_count` rejects non-finite, negative, or greater-than-
`MAX_ELEMENTS` counts before conversion; `MAX_ELEMENTS` is 1,000,000 and is
not a universal memory or output-size cap. `clock_to_cartesian` and
`polar_to_cartesian` perform coordinate conversion without enforcing dial
containment or conventional clock ranges. `paon_wave_fn` evaluates the sine
or normalized triangle-wave approximation.

Native config validation does not cover every rosette, bit, run-preset, or
export parameter. Failed general-run passes are skipped and recorded rather
than returned by `generate()`. Full checks and gaps are listed in
[Errors and validation](errors.md). Python maps the first two error variants
to `ValueError` and the export variant to `OSError`; those exception types are
binding behavior, not native Rust errors.
