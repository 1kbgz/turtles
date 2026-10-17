# Composition

`turtles.WatchFace` is the public Python wrapper for assembling a dial from all
eight standalone pattern-layer families and the three spirograph types.
`turtles.GuillochePattern` is a separate extension class whose Python API exposes
only spirographs, flinqué, diamant, limaçon, and paon. Rust methods do not
automatically become methods of either Python class.

Related reference: [Patterns](patterns.md), [Exports](exports.md),
[Errors](errors.md), [Rose engine](rose-engine.md), and [Rust API](rust.md).

## Shared units, validation, and lifecycle

Both constructors take a **radius in millimeters**, not a diameter. The radius
must be finite and in **[26, 44] mm inclusive**. A radius of 38 mm produces a
76 mm diameter dial. This restriction belongs to the compositor; smaller
standalone layers and spirographs are valid.

Cartesian positions, center distances, pattern lengths, and export depths are
in millimeters. Polar positioning and pattern rotations use radians;
`CubeLayer.leg_angle` uses degrees. Paon's `fan_angle` is a phase amplitude in
wave cycles, not a geometric angle. Clock placement uses the hour hand, including
the minute contribution, with 12:00 upward (negative y) and 3:00 rightward.
Hours are reduced modulo 12; the conventional hour 1–12 and minute 0–59 ranges
are not explicitly enforced. See {ref}`Units and positioning <units-and-positioning>`.

Adding a layer does not generate its geometry. Binding methods accepting an
existing layer reconstruct a fresh internal layer from its parameters; generated
points are not copied and the original Python object is not attached by
reference. Pattern-layer copies retain their configured centers. Spirograph
copies added as objects are reconstructed at the origin.

`generate()` returns `None` and regenerates all internal layers, replacing their
previous points. Generation of the compositor does not populate the original
Python layer's `get_lines()` result. After adding more layers, another
`generate()` call generates those layers and regenerates the earlier ones.
Exports do not generate automatically. Mutating methods and successful exports
return `None`, so calls are not chainable.

Layer validation follows the individual constructors in [Patterns](patterns.md),
not a universal frequency or resolution rule. Validation errors map to
`ValueError`; export failures map to `OSError`. Python type and unsigned-integer
conversion errors can occur before Rust validation. See [Errors](errors.md).

## `WatchFace`

### `WatchFace` constructor and property

```python
WatchFace(radius: float)
```

Creates an empty dial composition without an inner circle, bezel, center hole,
or pattern layers. `radius` is the only public read-only property and returns the
dial radius in mm. Its finite, inclusive 26–44 mm constraint is described above.

The signatures below are bound-method calls: `face` denotes a `WatchFace`
instance. They describe the public wrapper, not `turtles.RustWatchFace` or its
internal `_watch_face` object.

### Dial elements

| Signature                                                                  | Effect                                                                                                                                  |
| -------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| `face.add_inner()`                                                         | Enables the dial circle at `face.radius`, with fill `#fafaf5`, stroke `#2c2c2c`, and stroke width 0.3 mm.                               |
| `face.add_outer()`                                                         | Enables an unfilled bezel circle at `1.05 * face.radius`, with stroke `#1a1a1a` and stroke width 0.8 mm.                                |
| `face.add_center_hole()`                                                   | Appends a filled `#1a1a1a` circle at the origin with radius 0.8 mm.                                                                     |
| `face.add_hole(hour: int, minute: int, radius: float, hole_radius: float)` | Appends a filled `#1a1a1a` circle centered at the given clock position, `radius` mm from the dial center, with radius `hole_radius` mm. |

For `add_hole()`, `radius` means **center distance**, not hole size or dial size;
`hole_radius` supplies the hole size. The implementation performs clock
conversion but does not validate positive/finite hole size, positive/finite
distance, or containment within the dial. Repeated `add_inner()` or
`add_outer()` calls replace the corresponding configuration; repeated hole calls
append more circles.

These elements are SVG decorations. Combined STL and STEP delegate to the
pattern compositor; they do not construct the inner disk, bezel solid, or
physical hole cuts. The wrapper exposes no styling configuration arguments.

### Unimplemented elements

```python
face.add_subdial(hour: int, minute: int, radius: float, subdial_radius: float)
face.add_window(hour: int, minute: int, width: float, height: float,
                angle: float = 0.0)
```

`add_subdial()` always raises
`NotImplementedError("add_subdial is not implemented yet")`.
`add_window()` always raises
`NotImplementedError("add_window is not implemented yet")`.
Neither renders, mutates the composition, nor validates its intended geometry.
The intended window `angle` unit is degrees, but no rotation is implemented.
Positioned texture layers do not implement these dial elements.

`add_hour_marks()` and `add_minute_marks()` are not exposed methods.

### Adding existing layers

```python
face.add(layer)
face.add_flinque_layer(layer)
face.add_diamant_layer(layer)
face.add_limacon_layer(layer)
face.add_draperie_layer(layer)
face.add_paon_layer(layer)
face.add_huiteight_layer(layer)
face.add_clous_de_paris_layer(layer)
face.add_cube_layer(layer)
```

Each family-specific method accepts its matching standalone class, including
`HuitEightLayer` for `add_huiteight_layer()` and `ClousDeParisLayer` for
`add_clous_de_paris_layer()`. It copies configuration and center into a fresh,
ungenerated internal layer.

`add(layer)` dispatches the eight standalone classes to those methods. Other
objects are passed to the binding's spirograph adder, which accepts only
`HorizontalSpirograph`, `VerticalSpirograph`, or `SphericalSpirograph`; unsupported
objects raise `TypeError`. `add()` does not accept a `GuillochePattern`,
`RoseEngineLathe`, or `RoseEngineLatheRun`.

The public wrapper has no `add_layer()`, `add_layer_at_clock()`,
`add_layer_at_polar()`, or family-specific polar methods. Their presence in a
Rust implementation or raw binding is not part of this wrapper's API.

### Pattern convenience methods

All methods below append one layer at the hour-hand center given by `hour`,
`minute`, and `distance`. The default `distance=0.0` centers the layer regardless
of the clock position. `None` size defaults are resolved from `face.radius`
before construction; explicit values retain the standalone family's validation.
Parameter meanings and units not repeated here match [Patterns](patterns.md).

#### Flinqué

```python
face.add_flinque(radius=None, hour=12, minute=0, distance=0.0, num_petals=12,
                 num_waves=60, wave_amplitude=0.8, wave_frequency=20.0,
                 inner_radius_ratio=0.05)
```

`radius=None` resolves to `face.radius`. This radius is nominal ring size, not a
local clipping boundary; composed export applies the dial boundary separately.

#### Diamant

```python
face.add_diamant(num_circles=72, circle_radius=None, hour=12, minute=0,
                 distance=0.0, resolution=360)
```

`circle_radius=None` resolves to `face.radius`. Individual circles can reach
twice `circle_radius` from the layer center before dial clipping.

#### Limaçon

```python
face.add_limacon(num_curves=72, base_radius=None, amplitude=None, hour=12,
                 minute=0, distance=0.0, resolution=360)
```

`base_radius=None` and `amplitude=None` each resolve independently to
`face.radius`. Changing `base_radius` alone does not change the default
`amplitude` to that value.

#### Draperie

```python
face.add_draperie(num_rings=96, base_radius=None, radius_step=0.44,
                  wave_frequency=12.0, phase_shift=None,
                  phase_oscillations=2.5, hour=12, minute=0, distance=0.0,
                  resolution=1500, phase_exponent=3, wave_exponent=1,
                  circular_phase=2.0)
face.add_draperie_sharp(num_rings=96, base_radius=None, radius_step=0.44,
                        wave_frequency=12.0, phase_shift=None,
                        phase_oscillations=2.5, hour=12, minute=0,
                        distance=0.0, resolution=1500, wave_exponent=1)
```

`base_radius=None` resolves to `face.radius`; this is the middle of the ring
band, not its outer boundary. `phase_shift=None` resolves to π/12 radians.
Amplitude is automatic and has no Python argument.

`add_draperie_sharp()` calls `add_draperie()` with `phase_exponent=1` and
`circular_phase=0.0`. Other arguments pass through unchanged. It is a preset,
not a separate layer class.

#### Paon

```python
face.add_paon(num_lines=500, radius=None, amplitude=0.1, wave_frequency=15.0,
              phase_rate=9.0, hour=12, minute=0, distance=0.0, resolution=800,
              n_harmonics=5, fan_angle=3.0, vanishing_point=0.2)
```

`radius=None` resolves to `face.radius`, defining the layer's local circle.
The wrapper defaults differ from `PaonLayer`: amplitude 0.1 rather than 0.035 mm,
wave frequency 15.0 rather than 10.0, five rather than three harmonics,
`fan_angle=3.0` rather than 4.0, and `vanishing_point=0.2` rather than 0.3.
`fan_angle` is a phase amplitude in cycles; `vanishing_point` is a fraction of
the local diameter and must be strictly positive.

#### Huit-eight

```python
face.add_huiteight(num_curves=36, scale=None, hour=12, minute=0, distance=0.0,
                   resolution=360, num_clusters=0, cluster_spread=0.0)
```

`scale=None` resolves to `face.radius`. `scale` is each figure-eight's half-width
in mm. Zero clusters select uniform rotations; automatic spread is used only
when clustering is active. See {ref}`HuitEightLayer <huiteightlayer>`.

#### Clous de Paris

```python
face.add_clous_de_paris(spacing=1.0, radius=None, angle=None, hour=12,
                        minute=0, distance=0.0, resolution=200)
```

`radius=None` resolves to `face.radius`; `angle=None` resolves to π/4 radians.
`angle` rotates the grid, not the center position. `radius` defines the local
clipping circle.

(cube)=

#### Cube

```python
face.add_cube(spacing=0.5, radius=None, angle=0.0, hour=12, minute=0,
               distance=0.0, resolution=200, cuts_per_group=12,
               gap_per_group=6, amplitude=0.0, leg_angle=30.0)
```

`radius=None` resolves to `face.radius`, defining the local clipping circle.
`angle` rotates the pattern in radians; `leg_angle` is in degrees.
`amplitude=0.0` selects automatic amplitude. `resolution` is validated but does
not control zigzag-vertex generation.

The public wrapper defaults to **12 cuts and 6 gap spacings**, whereas
`CubeLayer` and the raw binding default to 8 and 8. An existing `CubeLayer`
passed to `add_cube_layer()` or `add()` retains its own configuration.

### `WatchFace` generation and export

```python
face.generate()
face.to_svg(filename: str)
face.to_stl(filename: str, depth: float = 0.1, base_thickness: float = 2.0)
face.to_step(filename: str, depth: float = 0.1)
```

`generate()` regenerates all stored pattern layers. `to_svg()` includes enabled
dial elements and the generated patterns; it can write a decoration-only or
empty SVG without generated pattern geometry. STL and STEP require drawable
pattern geometry and raise `OSError` if clipping leaves no paths of at least two
points. Their output contains all stored pattern families, not SVG decorations.
See [Clipping and draw order](#clipping-and-draw-order) and
[Export limitations](#export-limitations).

The wrapper exposes no `get_lines()`, `layer_count()`, or `export_all()` method.

### `WatchFace` example

```python
from turtles import FlinqueLayer, WatchFace

face = WatchFace(radius=38.0)
face.add_inner()
face.add_outer()
face.add_center_hole()
face.add(FlinqueLayer.at_clock(radius=8.0, hour=3, minute=30, distance=15.0))
face.add_clous_de_paris(spacing=1.2, radius=10.0, hour=9, distance=15.0)
face.generate()
face.to_svg("dial.svg")
```

## `GuillochePattern`

### `GuillochePattern` constructor and property

```python
GuillochePattern(radius)
```

Creates an empty layer composition. `radius` is a read-only Python property in
mm, with the same finite, inclusive 26–44 mm constraint as `WatchFace`.
Combined SVG always adds the default dial circle, bezel, and 0.8 mm center-hole
circle; this class has no Python methods to configure or disable them.

The signatures below use `pattern` for a `GuillochePattern` instance. This
section lists its exposed Python composition methods, not every public Rust
method.

### Adding existing objects

```python
pattern.add_layer(spiro)
pattern.add_flinque_layer(flinque)
pattern.add_diamant_layer(diamant)
pattern.add_limacon_layer(limacon)
pattern.add_paon_layer(paon)
```

`add_layer(spiro)` accepts only `HorizontalSpirograph`, `VerticalSpirograph`, or
`SphericalSpirograph`. Other values raise `TypeError`. It reconstructs an
origin-centered, ungenerated spirograph from its parameters.

The other methods accept their corresponding standalone layer class and copy
its configuration and center into a new, ungenerated internal layer. Their
keyword parameter names are `flinque`, `diamant`, `limacon`, and `paon`, not
`layer`.

There is no generic `add()` method. Draperie, huit-eight, Clous de Paris, and
Cube adders are **not exposed** on Python `GuillochePattern`, even though the
Rust compositor supports them. `WatchFace` exposes those families.

(positioned-spirographs)=

### Positioned spirographs

```python
pattern.add_layer_at_polar(spiro_type, outer_radius, radius_ratio, point_distance,
                           rotations, resolution, angle, distance,
                           wave_amplitude=1.0, wave_frequency=5.0,
                           dome_height=5.0)
pattern.add_layer_at_clock(spiro_type, outer_radius, radius_ratio, point_distance,
                           rotations, resolution, hour, minute, distance,
                           wave_amplitude=1.0, wave_frequency=5.0,
                           dome_height=5.0)
```

`spiro_type` is a case-insensitive string: `"horizontal"`, `"vertical"`, or
`"spherical"`. Other strings raise `ValueError`. No whitespace stripping is
performed. All base spirograph and positioning arguments are required.

`wave_amplitude` and `wave_frequency` are used only for the vertical variant;
`dome_height` is used only for the spherical variant. Unused options do not
undergo that variant's constructor checks, though Python-to-Rust type
conversion still applies. Base parameter validation matches the
{ref}`standalone spirographs <shared-parameters-and-validation>`.
The resulting Cartesian center must be finite. There is no positive-distance
or dial-containment check.

These methods construct internal positioned spirographs directly; they do not
return a standalone spirograph object.

### Positioned flinqué

```python
pattern.add_flinque_at_polar(radius, angle, distance, num_petals=12,
                             num_waves=60, wave_amplitude=0.8,
                             wave_frequency=20.0, inner_radius_ratio=0.05)
pattern.add_flinque_at_clock(radius, hour, minute, distance, num_petals=12,
                             num_waves=60, wave_amplitude=0.8,
                             wave_frequency=20.0, inner_radius_ratio=0.05)
```

`radius` and positioning arguments are required. Other defaults and validation
match {ref}`FlinqueLayer <flinquelayer>`; `radius` does not default to
the compositor's radius.

### Positioned diamant

```python
pattern.add_diamant_at_polar(num_circles, circle_radius, angle, distance,
                             resolution=360)
pattern.add_diamant_at_clock(num_circles, circle_radius, hour, minute, distance,
                             resolution=360)
```

All arguments except `resolution` are required. Parameter meaning and validation
match {ref}`DiamantLayer <diamantlayer>`.

### Positioned limaçon

```python
pattern.add_limacon_at_polar(num_curves, base_radius, amplitude, angle, distance,
                             resolution=360)
pattern.add_limacon_at_clock(num_curves, base_radius, amplitude, hour, minute,
                             distance, resolution=360)
```

All arguments except `resolution` are required. In particular, `amplitude` has
no dial-derived default. Validation matches {ref}`LimaconLayer <limaconlayer>`.

### Positioned paon

```python
pattern.add_paon_at_polar(angle, distance, num_lines=500, radius=22.0,
                          amplitude=0.035, wave_frequency=10.0, phase_rate=9.0,
                          resolution=800, n_harmonics=3, fan_angle=4.0,
                          vanishing_point=0.3)
pattern.add_paon_at_clock(hour, minute, distance, num_lines=500, radius=22.0,
                          amplitude=0.035, wave_frequency=10.0, phase_rate=9.0,
                          resolution=800, n_harmonics=3, fan_angle=4.0,
                          vanishing_point=0.3)
```

Positioning arguments come first. Remaining defaults and validation match
{ref}`PaonLayer <paonlayer>`, not `WatchFace.add_paon()`. In particular,
`radius=22.0` is a fixed local radius, not the compositor radius; `None` is not a
supported size default here.

### `GuillochePattern` generation and export

```python
pattern.generate()
pattern.to_svg(filename)
pattern.to_stl(filename, depth=0.1, base_thickness=2.0)
pattern.to_step(filename, depth=0.1)
pattern.export_all(base_name, depth=0.1, base_thickness=2.0)
```

`generate()` regenerates all stored layers and returns `None`. `to_svg()` writes
one combined SVG with default dial decorations; it can succeed without generated
pattern lines. `to_stl()` and `to_step()` require drawable geometry after dial
clipping.

`export_all()` writes three **combined** files, in order:

1. `base_name + ".svg"`
1. `base_name + ".stl"`
1. `base_name + ".stp"`

It does not write a file per layer or strip an extension from `base_name`. With
no layers, it raises `OSError` before writing. With ungenerated or fully clipped
layers, SVG can be written before STL fails. Export is not transactional: a
failure does not remove files already written. STEP uses the `.stp` suffix in
this method, not `.step`.

`repr(pattern)` includes radius and stored layer count, but Python exposes no
`layer_count()`, `get_lines()`, `all_paths()`, or `clipped_paths()` method.

### `GuillochePattern` example

```python
from turtles import GuillochePattern

pattern = GuillochePattern(radius=38.0)
pattern.add_layer_at_clock(
    spiro_type="horizontal", outer_radius=8.0, radius_ratio=0.7,
    point_distance=0.8, rotations=7, resolution=360,
    hour=3, minute=0, distance=15.0,
)
pattern.add_limacon_at_polar(
    num_curves=24, base_radius=6.0, amplitude=3.0, angle=0.0, distance=0.0,
)
pattern.generate()
pattern.to_svg("combined.svg")
```

(clipping-and-draw-order)=

## Clipping and draw order

Layer size and dial radius are independent. Adding a layer does not resize it,
reject it for crossing the dial edge, or change its stored coordinates.

| Output        | Dial-boundary behavior                                                                                                                                                    |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Combined SVG  | All pattern content is inside a circular SVG `clip-path` centered at the origin with the compositor's radius. Underlying path coordinates can extend outside that circle. |
| Combined STL  | Every family's planar polylines are geometrically clipped to the dial circle before ribbon generation.                                                                    |
| Combined STEP | Every family's planar polylines are geometrically clipped to the dial circle before serialization at `z=0`.                                                               |

Geometric clipping splits a stroke when it exits and re-enters the dial. The
resulting pieces remain separate open paths; no closing chord is added across
the clipped gap. Local circular boundaries in Paon, Clous de Paris, and Cube are
separate from this dial boundary. The standalone layer's `get_lines()` is not
modified by combined export.

SVG painting order is **fixed by family, not by global insertion order**.
Insertion order is retained within a family; all three spirograph variants share
one insertion-ordered family. Adding a spirograph after a Cube layer still
renders the spirograph first.

| SVG                | Pattern painting order, first to last                                                             |
| ------------------ | ------------------------------------------------------------------------------------------------- |
| `WatchFace`        | Spirographs → flinqué → diamant → draperie → huit-eight → limaçon → paon → Clous de Paris → Cube. |
| `GuillochePattern` | Spirographs → flinqué → diamant → huit-eight → draperie → paon → limaçon → Clous de Paris → Cube. |

The second row describes the combined Rust renderer used by Python
`GuillochePattern`; its Python adders expose only spirographs, flinqué, diamant,
paon, and limaçon. Unsupported families in that row do not imply Python methods.
Among the exposed families, paon is painted before limaçon, unlike `WatchFace`.

For `WatchFace`, the optional inner circle is painted first, followed by the
clipped pattern group, optional bezel, and holes in hole-insertion order.
`GuillochePattern` always paints its inner circle first and its default bezel
and center-hole circle after the pattern group. Holes are filled SVG circles,
not masks or clipping exclusions for pattern geometry.

Both combined SVG renderers use a square canvas of side `5 * radius` mm with
viewBox `(-2.5 * radius, -2.5 * radius, 5 * radius, 5 * radius)`, rather than
fitting to the pattern bounds. Pattern strokes have no fill and use round caps
and joins. Spirograph colors cycle through `#1a1a1a`, `#2d2d2d`, `#3a3a3a`,
`#454545`, `#505050`, and `#5a5a5a`; their widths cycle through 0.04, 0.035, 0.03,
0.03, 0.025, and 0.025 mm. Other families use `#1a1a1a` at 0.03 mm. These styles
are not configurable through the documented Python composition methods.

(export-limitations)=

## Export limitations

- Combined exporters include every stored family's generated lines. They use
  planar coordinates for spherical spirographs; only standalone spherical STL
  and STEP use the projected 3D points.
- STL emits zero-thickness side ribbons from `z=0` to `z=-depth`, with
  placeholder normals. It does not produce a watertight dial, groove width, or
  base plate. `base_thickness` is accepted but unused by the mesh implementation.
- STEP writes an AP214 geometric curve set of polylines at `z=0`, not a solid or
  cut grooves. `depth` is accepted but does not alter these coordinates.
- Export `depth` and `base_thickness` are converted to floats, but these
  composition exporters do not add finite/positive-value validation for them.
- Inner circles, bezels, and holes are SVG-only decorations. They do not become
  physical features in combined STL or STEP.
- SVG export without generated pattern lines is allowed; that is not evidence
  that layers have been generated. STL and STEP reject an empty drawable result.

See [Exports](exports.md) for format details and [Errors](errors.md) for exception
mapping. Rose-engine objects use a separate API described in
[Rose engine](rose-engine.md).
