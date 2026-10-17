# Pattern layers and spirographs

All classes on this page are available from `turtles`. Signatures describe the
Python bindings, including their argument order and defaults. Rust configuration
defaults do not supply missing Python arguments.

The eight pattern layers are `FlinqueLayer`, `DiamantLayer`, `LimaconLayer`,
`DraperieLayer`, `PaonLayer`, `HuitEightLayer`, `ClousDeParisLayer`, and `CubeLayer`.
The three spirographs are `HorizontalSpirograph`, `VerticalSpirograph`, and
`SphericalSpirograph`.

Related reference: [Composition](composition.md), [Exports](exports.md),
[Errors](errors.md), [Rose engine](rose-engine.md), and [Rust API](rust.md).

(units-and-positioning)=

## Units and positioning

Lengths, Cartesian coordinates, amplitudes expressed as lengths, and export depths
are in millimeters. Ratios and counts are dimensionless. Polar angles, grid
rotations, `phase_shift`, and `cluster_spread` are in radians. `CubeLayer.leg_angle`
is in degrees. Paon's `fan_angle` is a phase amplitude in wave cycles, despite its
name; it is not a positioning angle.

Each of the eight pattern-layer constructors centers its pattern at `(0, 0)`.
Its static factories return a new layer:

| Factory            | Center                                                                  |
| ------------------ | ----------------------------------------------------------------------- |
| `with_center(...)` | Explicit `center_x`, `center_y`, in mm.                                 |
| `at_polar(...)`    | `x = distance * cos(angle)`, `y = distance * sin(angle)`.               |
| `at_clock(...)`    | Hour-hand position for `hour`, `minute`, at `distance` from the origin. |

Coordinates use positive x to the right and positive y downward in SVG. Polar
angle zero points right; positive angles turn clockwise on screen. Clock
positions put 12:00 at the top, 3:00 at the right, and 6:00 at the bottom.
`minute` advances the hour hand: 3:30 is halfway between 3 and 4, not the
minute-hand position at 6.

Clock conversion uses `hour % 12` and then adds `minute / 60` hours. The documented
clock convention is hours 1–12 and minutes 0–59, but these ranges are not
validated. Both arguments are unsigned 32-bit integers at the binding boundary.
Positioning distance is not required to be nonnegative, and placement is not
restricted to a dial boundary. The resulting center must be finite.

**Factory argument order differs by family.** The signatures below are complete;
there is no common positional constructor. For Clous de Paris and Cube,
`at_polar(angle, distance, ..., grid_angle=...)` separates the positioning angle
from the pattern rotation. Their other constructors call the rotation `angle`.

The spirograph classes expose only origin-centered Python constructors. Their
Rust positioning constructors are not Python class methods. Positioned Python
spirographs are available through
[`GuillochePattern.add_layer_at_polar()` and `add_layer_at_clock()`](composition.md#positioned-spirographs).

## Lifecycle and supported output

Construction validates parameters but does not generate geometry. `generate()`
replaces previously generated geometry and returns `None`; exports do not call it
automatically.

| Python API                                        | Eight pattern layers                       | Three spirographs |
| ------------------------------------------------- | ------------------------------------------ | ----------------- |
| `generate()`                                      | Supported.                                 | Supported.        |
| `get_lines()`                                     | Returns `list[list[tuple[float, float]]]`. | Not exposed.      |
| `to_svg(filename)`                                | Supported.                                 | Supported.        |
| `to_stl(filename, depth=0.1, base_thickness=2.0)` | Not exposed.                               | Supported.        |
| `to_step(filename, depth=0.1)`                    | Not exposed.                               | Supported.        |
| Read-only parameter properties                    | Listed per class below.                    | None exposed.     |
| `with_center()`, `at_polar()`, `at_clock()`       | Supported as listed below.                 | Not exposed.      |

For pattern layers, `get_lines()` returns a copy of the generated polylines in
absolute coordinates, including the center offset. It returns `[]` before
generation; changing the returned lists does not change the layer. A polyline
represents a ring, curve, or retained line segment, depending on the family.

Successful export methods return `None` and accept a filename string. Standalone
exports reject missing geometry with `OSError`; a generated pattern that retains
no lines is also empty. Standalone SVGs fit the generated bounds with a 5 mm
margin and do not add dial, bezel, or hole decorations.

The eight pattern layers have **no standalone STL or STEP methods**. Their
geometry can enter combined exports through `WatchFace`; `GuillochePattern`
exposes only a subset of layer families. See [Composition](composition.md).
Spirograph STL is a zero-thickness ribbon mesh, not a watertight plate. STEP
contains polylines, not a solid; `depth` does not change STEP geometry, and
`base_thickness` does not create an STL base. See [Exports](exports.md).

### Validation conventions

The constraints in each class section are the checks performed by its Rust
constructor. “Positive” means strictly greater than zero. Where finiteness is
specified, NaN and either infinity are rejected. Constructor validation failures
map to `ValueError`; binding type and unsigned-integer conversion failures occur
before Rust validation. See [Errors](errors.md).

Counts and resolutions bind to unsigned integers; `phase_exponent` and
`wave_exponent` bind to unsigned 32-bit integers. There is no universal positive
frequency rule, resolution minimum, or one-million-element limit across all
layers. The 26–44 mm dial-radius constraint applies to compositors, not these
standalone classes.

(flinquelayer)=

## `FlinqueLayer`

Concentric chevron rings with radial petal peaks and a fine ripple.

```python
FlinqueLayer(radius, num_petals=12, num_waves=60, wave_amplitude=0.8,
             wave_frequency=20.0, inner_radius_ratio=0.05)
FlinqueLayer.with_center(radius, center_x, center_y, num_petals=12,
                         num_waves=60, wave_amplitude=0.8,
                         wave_frequency=20.0, inner_radius_ratio=0.05)
FlinqueLayer.at_polar(radius, angle, distance, num_petals=12, num_waves=60,
                      wave_amplitude=0.8, wave_frequency=20.0,
                      inner_radius_ratio=0.05)
FlinqueLayer.at_clock(radius, hour, minute, distance, num_petals=12,
                      num_waves=60, wave_amplitude=0.8,
                      wave_frequency=20.0, inner_radius_ratio=0.05)
```

| Parameter            | Meaning                                                    |
| -------------------- | ---------------------------------------------------------- |
| `radius`             | Nominal outer ring radius, in mm; not a clipping boundary. |
| `num_petals`         | Chevron peaks per ring; also controls sampling density.    |
| `num_waves`          | Number of candidate concentric rings.                      |
| `wave_amplitude`     | Radial chevron amplitude, in mm.                           |
| `wave_frequency`     | Fine-ripple multiplier of the petal phase.                 |
| `inner_radius_ratio` | Starting ring radius as a fraction of `radius`.            |

Each retained ring has `num_petals * 80 + 1` points. Rings with base radius less
than `0.1 * wave_amplitude` are skipped. Chevron and ripple modulation can extend
beyond `radius`; standalone output is not clipped to it.

**Read-only properties:** `radius`, `center_x`, `center_y`. Configuration values
such as `num_petals` are not Python properties.

**Constraints:** `radius` finite and positive; `wave_amplitude` finite and
nonnegative; `wave_frequency`, `center_x`, and `center_y` finite;
`num_petals >= 1`; `num_waves >= 1`; `inner_radius_ratio` in `[0, 1]` inclusive
(rejects non-finite values). No constructor-level upper count limit is imposed.

(diamantlayer)=

## `DiamantLayer`

Equal-radius circles tangent to the layer center, with their circle centers
distributed around a full turn.

```python
DiamantLayer(num_circles, circle_radius, resolution=360)
DiamantLayer.with_center(num_circles, circle_radius, center_x, center_y,
                         resolution=360)
DiamantLayer.at_polar(num_circles, circle_radius, angle, distance, resolution=360)
DiamantLayer.at_clock(num_circles, circle_radius, hour, minute, distance,
                      resolution=360)
```

`num_circles` is the number of circles; `circle_radius` is each circle's radius
in mm, not the outer extent of the whole pattern. Each circle center is one
`circle_radius` from the layer center, so the pattern can reach
`2 * circle_radius` from that center. Each circle has `resolution + 1` points.
There is no standalone circular clipping boundary.

**Read-only properties:** `num_circles`, `circle_radius`, `center_x`, `center_y`.
`resolution` is not a Python property.

**Constraints:** `num_circles >= 1`; `circle_radius` finite and positive;
`resolution >= 10`; `center_x` and `center_y` finite.

(limaconlayer)=

## `LimaconLayer`

Limaçon curves with evenly spaced phase offsets, using
$r = a + b\\sin(\\theta + \\phi)$, where `a` is `base_radius`, `b` is `amplitude`,
and $\\phi$ is the curve's phase offset.

```python
LimaconLayer(num_curves, base_radius, amplitude, resolution=360)
LimaconLayer.with_center(num_curves, base_radius, amplitude, center_x, center_y,
                         resolution=360)
LimaconLayer.at_polar(num_curves, base_radius, amplitude, angle, distance,
                      resolution=360)
LimaconLayer.at_clock(num_curves, base_radius, amplitude, hour, minute, distance,
                      resolution=360)
```

`num_curves` is the number of phase-shifted curves. `base_radius` and `amplitude`
are lengths in mm. Each curve has `resolution + 1` points. `amplitude` may be
negative or exceed `base_radius`; it is not restricted to a ratio. With
`amplitude == base_radius`, each curve passes through the layer center. There
is no standalone circular clipping boundary.

**Read-only properties:** `num_curves`, `base_radius`, `amplitude`, `center_x`,
`center_y`. `resolution` is not a Python property.

**Constraints:** `num_curves >= 1`; `base_radius` finite and positive;
`amplitude`, `center_x`, and `center_y` finite; `resolution >= 10`.

(draperielayer)=

## `DraperieLayer`

Concentric wavy rings with a phase envelope across the ring stack.

```python
DraperieLayer(num_rings=96, base_radius=22.0, radius_step=0.44,
              wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5,
              resolution=1500, phase_exponent=3, wave_exponent=1,
              circular_phase=2.0)
DraperieLayer.with_center(center_x, center_y, num_rings=96, base_radius=22.0,
                          radius_step=0.44, wave_frequency=12.0,
                          phase_shift=None, phase_oscillations=2.5,
                          resolution=1500, phase_exponent=3, wave_exponent=1,
                          circular_phase=2.0)
DraperieLayer.at_polar(angle, distance, num_rings=96, base_radius=22.0,
                       radius_step=0.44, wave_frequency=12.0, phase_shift=None,
                       phase_oscillations=2.5, resolution=1500,
                       phase_exponent=3, wave_exponent=1, circular_phase=2.0)
DraperieLayer.at_clock(hour, minute, distance, num_rings=96, base_radius=22.0,
                       radius_step=0.44, wave_frequency=12.0, phase_shift=None,
                       phase_oscillations=2.5, resolution=1500,
                       phase_exponent=3, wave_exponent=1, circular_phase=2.0)
```

| Parameter            | Meaning                                                              |
| -------------------- | -------------------------------------------------------------------- |
| `num_rings`          | Number of rings.                                                     |
| `base_radius`        | Radius at the middle of the ring band, in mm; not a clipping radius. |
| `radius_step`        | Spacing between ring base radii, in mm.                              |
| `wave_frequency`     | Wave undulations per revolution.                                     |
| `phase_shift`        | Peak angular phase shift, in radians; `None` resolves to π/12.       |
| `phase_oscillations` | Phase-envelope cycles across the stack.                              |
| `resolution`         | Sampling intervals per ring; each ring has `resolution + 1` points.  |
| `phase_exponent`     | Sin-power phase-envelope exponent when `circular_phase == 0`.        |
| `wave_exponent`      | Signed sine-power exponent for each ring's wave.                     |
| `circular_phase`     | Dome-envelope exponent when positive; zero selects sin-power mode.   |

Ring base radii are centered on `base_radius`; the innermost is
`base_radius - (num_rings - 1) * radius_step / 2`. Python always selects automatic
wave amplitude, computed from adjacent phase differences and the innermost base
radius. There is no Python `amplitude` argument or property, and no standalone
clipping to `base_radius`.

**Read-only properties:** `num_rings`, `base_radius`, `radius_step`,
`wave_frequency`, `phase_exponent`, `wave_exponent`, `circular_phase`, `center_x`,
`center_y`. `phase_shift`, `phase_oscillations`, and `resolution` are not Python
properties.

**Constraints:** `num_rings >= 1`; `radius_step` and `base_radius` finite and
positive; `wave_frequency`, resolved `phase_shift`, `phase_oscillations`,
`center_x`, and `center_y` finite; `circular_phase` finite and nonnegative;
`resolution >= 10`. Neither exponent has an additional positive-value check.
The constructor accepts `wave_frequency=0`, but generation divides by that value
when computing the base phase and can produce non-finite coordinates. Validation
does not require the innermost base radius to be positive.

(paonlayer)=

## `PaonLayer`

Oscillating lines radiating from a vanishing point below the circle, producing
peacock-feather arch bands. The implementation uses a logarithmic distance scale
for the oscillation phase and retains sampled points inside the local circle.

```python
PaonLayer(num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0,
          phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0,
          vanishing_point=0.3)
PaonLayer.with_center(center_x, center_y, num_lines=500, radius=22.0,
                      amplitude=0.035, wave_frequency=10.0, phase_rate=9.0,
                      resolution=800, n_harmonics=3, fan_angle=4.0,
                      vanishing_point=0.3)
PaonLayer.at_polar(angle, distance, num_lines=500, radius=22.0, amplitude=0.035,
                   wave_frequency=10.0, phase_rate=9.0, resolution=800,
                   n_harmonics=3, fan_angle=4.0, vanishing_point=0.3)
PaonLayer.at_clock(hour, minute, distance, num_lines=500, radius=22.0,
                   amplitude=0.035, wave_frequency=10.0, phase_rate=9.0,
                   resolution=800, n_harmonics=3, fan_angle=4.0,
                   vanishing_point=0.3)
```

| Parameter         | Meaning                                                                                                                  |
| ----------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `num_lines`       | Number of candidate fan lines; lines retaining fewer than two points are omitted.                                        |
| `radius`          | Local circular boundary radius, in mm.                                                                                   |
| `amplitude`       | Horizontal oscillation amplitude, in mm.                                                                                 |
| `wave_frequency`  | Multiplier of the logarithmic distance phase.                                                                            |
| `phase_rate`      | Multiplier of the per-line absolute-sine phase envelope; controls arch columns.                                          |
| `resolution`      | Sampling intervals per candidate line, before retaining points inside the circle.                                        |
| `n_harmonics`     | Zero selects sine; positive values select a normalized triangle-wave approximation with `n_harmonics + 1` Fourier terms. |
| `fan_angle`       | Per-line phase-envelope amplitude, in full wave cycles; controls arch height, not angular fan spread.                    |
| `vanishing_point` | Distance below the local circle bottom as a fraction of its diameter.                                                    |

The vanishing point's local y coordinate is
`radius + vanishing_point * (2 * radius)`. Fan spread is calculated from this
geometry. `fan_angle` does not set it. Output may contain fewer than `num_lines`
polylines and fewer than `resolution + 1` points per line.

**Read-only properties:** `num_lines`, `radius`, `amplitude`, `wave_frequency`,
`phase_rate`, `n_harmonics`, `fan_angle`, `vanishing_point`, `center_x`, `center_y`.
`resolution` is not a Python property.

**Constraints:** `num_lines >= 1`; `radius`, `fan_angle`, and `vanishing_point`
finite and positive; `amplitude` finite and nonnegative; `wave_frequency`,
`phase_rate`, `center_x`, and `center_y` finite; `resolution >= 10`.
`vanishing_point=0` is rejected. Neither frequency nor phase rate is required to
be positive. No additional restriction is imposed on `n_harmonics` beyond its
unsigned integer type.

(huiteightlayer)=

## `HuitEightLayer`

Rotated figure-eight curves (lemniscates of Bernoulli), optionally grouped into
clusters. Rotations span a half turn because rotating a lemniscate by π produces
the same curve.

```python
HuitEightLayer(num_curves, scale, resolution=360, num_clusters=0,
               cluster_spread=0.0)
HuitEightLayer.with_center(num_curves, scale, center_x, center_y, resolution=360,
                           num_clusters=0, cluster_spread=0.0)
HuitEightLayer.at_polar(num_curves, scale, angle, distance, resolution=360,
                        num_clusters=0, cluster_spread=0.0)
HuitEightLayer.at_clock(num_curves, scale, hour, minute, distance,
                        resolution=360, num_clusters=0, cluster_spread=0.0)
```

`scale` is each lemniscate's half-width in mm. `num_curves` is the number of
curves, each with `resolution + 1` points. Clustering applies only when
`0 < num_clusters < num_curves`; all other values select uniform rotation.
Cluster sectors span `π / num_clusters` radians. `cluster_spread=0.0` selects
half a sector, `π / (2 * num_clusters)` radians, when clustering is active.
Other nonnegative spreads are used as given, without a sector-width bound.

**Read-only properties:** `num_curves`, `scale`, `num_clusters`, `cluster_spread`,
`center_x`, `center_y`. `cluster_spread` returns the configured value, including
the `0.0` automatic sentinel. `resolution` is not a Python property.

**Constraints:** `num_curves >= 1`; `scale` finite and positive;
`cluster_spread` finite and nonnegative; `center_x` and `center_y` finite;
`resolution >= 10`. `num_clusters` is not required to divide `num_curves` or to
be less than it.

(clousdeparislayer)=

## `ClousDeParisLayer`

Two perpendicular sets of straight grooves, analytically clipped to a local
circle. A rotation of π/4 produces the diagonal hobnail grid.

The expressions `pi / 4` below denote the numeric default π/4 radians.

```python
ClousDeParisLayer(spacing=1.0, radius=22.0, angle=pi / 4, resolution=200)
ClousDeParisLayer.with_center(center_x, center_y, spacing=1.0, radius=22.0,
                              angle=pi / 4, resolution=200)
ClousDeParisLayer.at_polar(angle, distance, spacing=1.0, radius=22.0,
                           grid_angle=pi / 4, resolution=200)
ClousDeParisLayer.at_clock(hour, minute, distance, spacing=1.0, radius=22.0,
                           angle=pi / 4, resolution=200)
```

`spacing` is groove spacing in mm; `radius` is the clipping-circle radius in mm.
`angle` is grid rotation except in `at_polar()`, where it positions the center
and `grid_angle` rotates the grid. Each generated groove has `resolution + 1`
sampled points.

**Read-only properties:** `spacing`, `radius`, `angle`, `resolution`, `center_x`,
`center_y`. The `angle` property always reports grid rotation.

**Constraints:** `spacing` and `radius` finite and positive; grid rotation,
`center_x`, and `center_y` finite; `resolution >= 2`. The derived count
`ceil(radius / spacing)` must be finite and at most 1,000,000. This bounds that
derived count, not the total number of points or grooves across both directions.

(cubelayer)=

## `CubeLayer`

Grouped parallel zigzag grooves with alternating phase offsets and gaps,
clipped to a local circle, producing a tumbling-blocks pattern.

```python
CubeLayer(spacing=0.5, radius=22.0, angle=0.0, resolution=200, cuts_per_group=8,
           gap_per_group=8, amplitude=0.0, leg_angle=30.0)
CubeLayer.with_center(center_x, center_y, spacing=0.5, radius=22.0, angle=0.0,
                       resolution=200, cuts_per_group=8, gap_per_group=8,
                       amplitude=0.0, leg_angle=30.0)
CubeLayer.at_polar(angle, distance, spacing=0.5, radius=22.0, grid_angle=0.0,
                    resolution=200, cuts_per_group=8, gap_per_group=8,
                    amplitude=0.0, leg_angle=30.0)
CubeLayer.at_clock(hour, minute, distance, spacing=0.5, radius=22.0, angle=0.0,
                    resolution=200, cuts_per_group=8, gap_per_group=8,
                    amplitude=0.0, leg_angle=30.0)
```

| Parameter              | Meaning                                                                                              |
| ---------------------- | ---------------------------------------------------------------------------------------------------- |
| `spacing`              | Distance between adjacent zigzag lines, in mm.                                                       |
| `radius`               | Local clipping-circle radius, in mm.                                                                 |
| `angle` / `grid_angle` | Pattern rotation, in radians; `at_polar()` reserves `angle` for positioning.                         |
| `resolution`           | Retained for API compatibility; generation uses exact zigzag vertices rather than this sample count. |
| `cuts_per_group`       | Number of zigzag lines in each cutting group.                                                        |
| `gap_per_group`        | Empty gap measured in line spacings.                                                                 |
| `amplitude`            | Half peak-to-trough height in mm; zero selects `(gap_per_group + 1) * spacing / 2`.                  |
| `leg_angle`            | Zigzag-leg angle from horizontal, in degrees.                                                        |

**Read-only properties:** `spacing`, `radius`, `angle`, `resolution`,
`cuts_per_group`, `gap_per_group`, `amplitude`, `leg_angle`, `center_x`, `center_y`.
`amplitude` reports the configured value, including the `0.0` automatic sentinel.

**Constraints:** `spacing` and `radius` finite and positive; `amplitude` finite
and nonnegative; pattern rotation, `leg_angle`, `center_x`, and `center_y` finite;
`resolution >= 2`; `cuts_per_group >= 1`; `gap_per_group >= 1`;
`0 < leg_angle < 90`. The group cycle
`(cuts_per_group + gap_per_group) * spacing` must be finite and positive, and
`ceil(radius / group_cycle)` must be finite and at most 1,000,000. This is not a
bound on every allocation or on total zigzag vertices.

The standalone defaults are eight cuts and eight gap spacings.
[`WatchFace.add_cube()`](composition.md#cube) instead defaults to twelve cuts and
six gap spacings.

(spirographs)=

## Spirographs

All three spirographs use a hypotrochoid with fixed-circle radius `outer_radius`,
rolling-circle radius `outer_radius * radius_ratio`, and pen offset
`point_distance * outer_radius * radius_ratio`.

### `HorizontalSpirograph`

```python
HorizontalSpirograph(outer_radius, radius_ratio, point_distance, rotations,
                      resolution)
```

Produces the planar hypotrochoid without additional modulation. All arguments
are required.

### `VerticalSpirograph`

```python
VerticalSpirograph(outer_radius, radius_ratio, point_distance, rotations,
                    resolution, wave_amplitude=1.0, wave_frequency=5.0)
```

Adds `wave_amplitude * sin(wave_frequency * t)` to the planar curve's y
coordinate. `wave_amplitude` is in mm; `wave_frequency` is cycles per revolution
of the curve parameter. “Vertical” refers to y modulation, not a z coordinate.

### `SphericalSpirograph`

```python
SphericalSpirograph(outer_radius, radius_ratio, point_distance, rotations,
                     resolution, dome_height=5.0)
```

Computes both the planar hypotrochoid and a spherical projection.
`dome_height` is a positive length in mm. The projection derives sphere radius
`(outer_radius**2 + dome_height**2) / (2 * dome_height)` and clamps the projection's
inverse-sine argument to its domain.

Standalone SVG uses the planar points. Standalone STL and STEP use the projected
3D points. [Composition](composition.md) uses the planar points even for combined
STL and STEP; a spherical layer does not make a composed dial domed.

(shared-parameters-and-validation)=

### Shared parameters and validation

| Parameter                | Meaning and constraint                                                                           |
| ------------------------ | ------------------------------------------------------------------------------------------------ |
| `outer_radius`           | Fixed-circle radius in mm; finite and positive, without the 26–44 mm dial restriction.           |
| `radius_ratio`           | Rolling-circle radius divided by fixed-circle radius; finite, strictly between 0 and 1.          |
| `point_distance`         | Pen offset divided by rolling-circle radius; finite and nonnegative. Values above 1 are allowed. |
| `rotations`              | Positive integer number of parameter revolutions.                                                |
| `resolution`             | Positive integer samples per revolution.                                                         |
| `rotations * resolution` | Must not overflow and must be at most 1,000,000.                                                 |
| `wave_amplitude`         | Vertical variant only: finite and nonnegative.                                                   |
| `wave_frequency`         | Vertical variant only: finite; zero and negative values are accepted.                            |
| `dome_height`            | Spherical variant only: finite and positive.                                                     |

Generation stores exactly `rotations * resolution` samples, without an additional
endpoint sample. Curves need not return to their start. SVG does not forcibly
close the path; standalone STL currently connects the final point back to the
first. STEP exports the sampled polyline without adding that closing segment.

These Python classes expose `generate()`, `to_svg(filename)`,
`to_stl(filename, depth=0.1, base_thickness=2.0)`, `to_step(filename, depth=0.1)`,
and `repr()`. They expose no read-only parameter properties, `get_lines()`,
point getters, or positioning factories. `generate()` returns `None`, not a
Python point list. None of the standalone spirograph exporters clips the curve
to `outer_radius`.
