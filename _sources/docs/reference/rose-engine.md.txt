# Rose engine Python API

The `turtles` package exports `RosettePattern`, `CuttingBit`,
`RoseEngineConfig`, `RoseEngineLathe`, and `RoseEngineLatheRun`.
`RoseEngineLathe` generates a single radial path with width-offset edges;
`RoseEngineLatheRun` generates multiple curves or line segments.

Distances, coordinates, widths, and depths are in millimeters. Angles are in
radians except `CuttingBit.v_shaped(angle, width)` and the `leg_angle` argument
of `RoseEngineLatheRun.cube()`, which use degrees. Frequencies, ratios,
exponents, and counts are dimensionless unless stated otherwise.

The signatures below describe the Python bindings, not the additional Rust
API. Arguments accept positional or keyword syntax. Count arguments are
Python integers converted to Rust `usize`; `wave_exponent` and
`phase_exponent` are converted to `u32`. Negative or out-of-range integers
raise `OverflowError` during conversion, independently of parameter
validation. Incompatible argument types raise `TypeError`.

Related references: [Exports](exports.md), [Errors](errors.md), and
[Pattern layers](patterns.md).

## `RosettePattern`

Rosette factories return a new `RosettePattern`. There is no public Python
constructor, property getter, or setter. Rosette parameters are selected at
creation; a config's primary pattern is replaced with `set_rosette()`.

### Rosette factories

- `RosettePattern.circular()`: zero radial displacement, independent of amplitude.
- `RosettePattern.elliptical(eccentricity, rotation=0.0)`: normalized elliptical displacement. `eccentricity` is the major-to-minor axis ratio, not the geometric eccentricity of an ellipse; `rotation` is in radians. Displacement is normalized to approximately -1 to 1 when the ratio is greater than 1. Ratios at or below 1 produce zero displacement, as do non-finite ratios or rotations. The implementation also returns zero when the normalized axis difference is no greater than floating-point epsilon.
- `RosettePattern.sinusoidal(frequency)`: `sin(angle * frequency)`; `frequency` is cycles per revolution.
- `RosettePattern.multi_lobe(lobes)`: `2 * abs(sin(angle * lobes / 2)) - 1`; `lobes` is an integer lobe count.
- `RosettePattern.epicycloid(petals)`: `cos(angle * petals)`; `petals` is the integer angular multiplier. A config adds this displacement to its base radius, so this does not by itself specify a standalone mathematical epicycloid.
- `RosettePattern.huit_eight(lobes)`: `sin(angle * lobes) * cos(angle / 2)`; radial modulation, not a Cartesian lemniscate.
- `RosettePattern.grain_de_riz(grain_size, rows)`: `abs(sin(angle * rows)) * sin(angle / grain_size)`. `grain_size` is an angular scale in radians, not a grain length in millimeters; `rows` is an integer multiplier. A zero or non-finite `grain_size` produces zero displacement.
- `RosettePattern.draperie(frequency, wave_exponent=1)`: a sine wave with `frequency` cycles per revolution. For `wave_exponent > 1`, applies `abs(sin(angle * frequency)) ** wave_exponent` with the sine's sign; exponents 0 and 1 both use the unmodified sine.
- `RosettePattern.paon(frequency)`: `sin(angle * frequency)`. The fan geometry of the run preset requires `RoseEngineLatheRun.paon()`, not this rosette alone.
- `RosettePattern.diamant(divisions)`: `abs(sin(angle * divisions)) + abs(sin(angle * divisions + pi / 4)) - 1`; `divisions` is an integer angular multiplier. This differs from the tangent-circle run preset.

These factories do not validate floating-point parameters or require positive
counts. The elliptical and rice-grain fallback behavior occurs when
displacement is evaluated. Other invalid values can produce non-finite
coordinates later; there is no general rosette-validation pass.

`repr(pattern)` identifies the factory and its parameters. Rust custom
rosettes and displacement evaluation are not exposed as Python methods:
there is no `custom()`, `from_function()`, or `displacement()` binding.

## `CuttingBit`

Bit factories return a new `CuttingBit`. There is no public Python
constructor. All dimensions below are in millimeters.

### Bit factories

- `CuttingBit.v_shaped(angle, width)`: stores `width`. Clamps `angle` to 1–179 degrees, then computes `depth = width / (2 * tan(angle_in_radians / 2))`.
- `CuttingBit.flat(width, depth)`: stores the supplied width and depth.
- `CuttingBit.round(diameter)`: `width = diameter`, `depth = diameter / 2`.
- `CuttingBit.elliptical(width, aspect_ratio)`: stores `width`; `depth = width / (2 * aspect_ratio)`. `aspect_ratio` is the major-to-minor axis ratio.

Clamping the V angle does not constitute general validation: widths, depths,
diameters, and aspect ratios are not checked for positivity or finiteness.
A zero elliptical aspect ratio can produce an infinite depth rather than a
Python division exception. Lathe construction does not add bit validation.

### Properties

- `width`: read-only `float`; stored width or diameter in millimeters.
- `depth`: read-only `float`; stored or derived depth in millimeters.

`repr(bit)` reports width and depth, not the bit shape. Shape access,
custom profiles, cross sections, footprints, and Rust's default bit are not
exposed in Python.

## `RoseEngineConfig`

### Constructor and defaults

```python
RoseEngineConfig(base_radius, amplitude)
```

Both arguments are required floating-point values in millimeters. The
constructor stores them without validation. It initializes the following
state:

- Base radius: supplied `base_radius`; read-only `base_radius` property.
- Primary amplitude: supplied `amplitude`; read-only `amplitude` property.
- Primary rosette: `RosettePattern.multi_lobe(12)`; replaced with `set_rosette(pattern)`, with no getter.
- Resolution: `1000`; read-only `resolution` property, mutated with `set_resolution(resolution)`.
- Primary phase: `0.0` radians; not exposed.
- Start and end angles: `0.0` and `2 * pi` radians; not exposed.
- Secondary rosette: absent; added with `with_secondary_rosette(rosette, amplitude)`, with no getter.
- Secondary amplitude and phase: `0.0` mm and `0.0` radians; no direct access.
- Depth modulation: disabled; enabled with `with_depth_modulation(amplitude, frequency)`.
- Depth-modulation amplitude and frequency: `0.0` and `1.0`; no direct access.

The three properties have types `float`, `float`, and `int`, respectively.
They have no assignment setters. There is no no-argument constructor or
Python `default()` factory.

For a standard radial path, the radius is the base radius plus primary
amplitude times primary displacement, plus secondary amplitude times
secondary displacement when present. The Python config API leaves both
phase offsets at zero. It does not enforce that the resulting radius stays
positive or that curves avoid intersections.

### Mutation methods

- `set_rosette(pattern) -> None`: replaces the primary rosette with a stored copy of `pattern`; leaves amplitude and resolution unchanged.
- `set_resolution(resolution) -> None`: stores an integer resolution. Standard radial generation uses `resolution + 1` points, including both angular endpoints.
- `with_secondary_rosette(rosette, amplitude) -> None`: adds or replaces the secondary rosette and its amplitude in millimeters; leaves the secondary phase at zero.
- `with_depth_modulation(amplitude, frequency) -> None`: enables depth modulation and replaces its parameters. `amplitude` is a fraction of bit depth, conventionally 0–1; `frequency` is depth cycles per revolution.

All four methods mutate the same config and perform no numeric validation
beyond Python-to-Rust argument conversion. Despite their names, the
`with_...` methods do not return the config and cannot be chained. There is
no bound method to remove the secondary rosette or disable depth modulation.

Depth modulation calculates bit depth times
`max(0, 1 + amplitude * sin(angle * frequency))`. Neither the conventional
0–1 amplitude range nor frequency finiteness is enforced. This affects
internal single-pass depth and shading data; the bound exports do not use
that depth map, and Python exposes no depth-map getter.

### Configuration factories

Each static factory returns a new `RoseEngineConfig`. All arguments shown
are required. Unlisted settings retain the constructor defaults; no factory
validates its numeric parameters.

- `RoseEngineConfig.classic_multi_lobe(base_radius, lobes, amplitude)`: `multi_lobe(lobes)`; resolution `1000`.
- `RoseEngineConfig.sunburst(base_radius, rays, amplitude)`: `multi_lobe(rays)`; resolution `2000`.
- `RoseEngineConfig.wave(base_radius, frequency, amplitude)`: `sinusoidal(frequency)`; resolution `1000`.
- `RoseEngineConfig.rose_curve(base_radius, petals, amplitude)`: `epicycloid(petals)`; resolution `1000`.
- `RoseEngineConfig.compound(base_radius, primary_lobes, primary_amplitude, secondary_frequency, secondary_amplitude)`: `multi_lobe(primary_lobes)` with primary amplitude `primary_amplitude`; adds `sinusoidal(secondary_frequency)` at `secondary_amplitude`; resolution `1000`.
- `RoseEngineConfig.huit_eight(base_radius, amplitude)`: `huit_eight(8)`; resolution `1500`. This is radial modulation, not the `huiteight()` run preset's lemniscate.
- `RoseEngineConfig.grain_de_riz(base_radius, grain_size, amplitude)`: `grain_de_riz(grain_size, rows=12)`; resolution `2000`.
- `RoseEngineConfig.draperie(base_radius, wave_frequency, amplitude)`: `draperie(wave_frequency, wave_exponent=1)`; resolution `1500`. Does not configure multiple rings, phase oscillation, or automatic amplitude selection.
- `RoseEngineConfig.diamant(base_radius, divisions, amplitude)`: `diamant(divisions)`; resolution `2000`. Does not configure tangent circles.

`repr(config)` reports base radius, amplitude, and resolution; it omits the
rosette, secondary motion, and depth-modulation settings.

### Validation and copying

`RoseEngineLathe(config, bit)`, its `with_center()` factory, and the general
`RoseEngineLatheRun` constructors validate the config when they copy it into
a generator. They require:

- A finite, strictly positive `base_radius`.
- Finite, nonnegative primary and secondary amplitudes.
- Finite primary and secondary phases and start/end angles.
- `resolution >= 10`.

They also require finite center coordinates. Invalid values in these checks
raise `ValueError`; constructing or mutating a config alone does not.
There is no Python `validate()` method. These checks do not inspect rosette
parameters, the bit, or depth-modulation parameters.

Both config and bit are cloned during generator construction. Later config
mutations do not update an existing lathe or run, even if it has not yet
generated geometry. Creating a new generator is necessary to use the changed
config. The run presets below construct their own configs and have different
validation coverage.

## `RoseEngineLathe`

### Construction

- `RoseEngineLathe(config, bit)`: returns a new `RoseEngineLathe` centered at `(0.0, 0.0)`.
- `RoseEngineLathe.with_center(config, bit, center_x, center_y)`: returns a new `RoseEngineLathe` with the supplied center coordinates in millimeters; both coordinates are required.

`config` must be a `RoseEngineConfig` and `bit` a `CuttingBit`. Construction
copies both objects and applies the checks above. It does not generate
geometry. Python exposes no config, bit, center, path, or rendered-output
properties on the lathe.

### Generation and exports

- `generate() -> None`: replaces previously generated geometry with the radial centerline and two perpendicular offset edges at half the bit width. Generates internal depth/shading arrays if depth modulation is enabled.
- `to_svg(filename) -> None`: writes the centerline with stroke width `0.1` and the two offset edges with stroke width `0.05`.
- `to_stl(filename, depth=0.1, base_thickness=2.0) -> None`: writes two triangles per centerline segment, spanning from `z=0` to `z=-depth`. `depth` is in millimeters; `base_thickness` is accepted but unused.
- `to_step(filename, depth=0.1)`: never succeeds. Before generation, raises `OSError` for missing geometry. After generation, raises `OSError` containing `STEP export not yet implemented`.

Every export requires an earlier `generate()` call; export methods do not
generate automatically. `filename` is a string path. SVG dimensions are in
millimeters, with a 5 mm margin around the generated bounds. Missing
generation, invalid SVG bounds, and file-write failures raise `OSError`.
The STL method does not perform the SVG bounds checks or validate its export
parameters.

The STL output is an open strip of vertical triangles along the centerline,
not a closed solid or a bit-profile groove. It does not apply bit width,
bit shape, bit depth, or depth modulation, and it does not add a base plate.
These methods do not produce machine instructions or establish machining
suitability. See [Exports](exports.md) for format limitations.

`repr(lathe)` reports center coordinates and base radius. There is no
single-pass Python `get_lines()` method.

## `RoseEngineLatheRun`

### Constructor

```python
RoseEngineLatheRun(
    config,
    bit,
    num_passes,
    segments_per_pass=24,
    radius_step=0.0,
    phase_shift=0.0,
    phase_oscillations=1.0,
    circular_phase=0.0,
    phase_exponent=1,
)
```

Copies `config` and `bit`, centers the pattern at `(0.0, 0.0)`, and starts
with no generated lines. The arguments have these effects:

- `config`: base `RoseEngineConfig` copied for each standard radial pass.
- `bit`: `CuttingBit` copied into the run. Exported run lines are centerlines, not width-offset cut edges.
- `num_passes`: integer requested pass count; must be at least 1.
- `segments_per_pass`: integer segmentation count; must be at least 1. A value of 1 retains each whole path. Larger values draw approximately 70% of each sample-index cycle and omit 30% as a gap.
- `radius_step`: change in base radius per pass in millimeters. Any nonzero value selects concentric-ring mode, including a negative value. Zero selects phase-rotation mode.
- `phase_shift`: phase-envelope amplitude in radians, used only in concentric-ring mode.
- `phase_oscillations`: number of phase-envelope cycles across the ring stack, used only in concentric-ring mode.
- `circular_phase`: dimensionless dome exponent. Positive values select the dome envelope; values at or below zero select the sine-power envelope.
- `phase_exponent`: integer exponent for the sine-power phase envelope; ignored when `circular_phase > 0`.

In phase-rotation mode, pass `i` adds `2 * pi * i / num_passes` to the primary
rosette phase, not to the coordinate angle or secondary phase. The phase
envelope arguments have no effect in this mode. This is not a guarantee that
every pass produces a geometrically distinct curve.

In concentric-ring mode, pass `i` has base radius
`base_radius + (i - (num_passes - 1) / 2) * radius_step`. Its primary phase
adds `phase_shift * envelope(t)`, where
`t = 2 * pi * phase_oscillations * i / num_passes`. For positive
`circular_phase`, the envelope is
`sign(sin(t)) * (1 - (1 - abs(sin(t))) ** circular_phase)`; otherwise it is
`sign(sin(t)) * abs(sin(t)) ** phase_exponent`.

Construction checks pass and segment counts, the copied config, and center
coordinates. The binding then assigns `radius_step`, `phase_shift`,
`phase_oscillations`, `circular_phase`, and `phase_exponent` without additional
numeric validation. There are no Python getters or setters for these run
settings.

Segmentation divides `resolution + 1` samples using integer arithmetic;
line lengths and counts can be reduced by rounding. If there are too few
samples per requested segment, generation can produce no lines. A concentric
pass whose derived config is invalid, such as a nonpositive base radius, is
skipped during `generate()` rather than raised as a Python exception. Rust
records these failures, but Python does not expose `generation_errors()`.

### Centered factory

```python
RoseEngineLatheRun.with_center(
    config,
    bit,
    num_passes,
    segments_per_pass=24,
    center_x=0.0,
    center_y=0.0,
)
```

Returns a new run with copied config and bit, validated finite center
coordinates, and the same pass/segment/config checks as the constructor.
Unlike the constructor, this factory accepts no ring or phase-envelope
arguments: `radius_step=0.0`, `phase_shift=0.0`,
`phase_oscillations=1.0`, `circular_phase=0.0`, and `phase_exponent=1` remain
internal defaults. It does not generate geometry.

### Generation, data, and export

- `generate() -> None`: clears previous passes, lines, and internal generation errors, then regenerates the run. It does not export.
- `num_passes`: read-only `int` property. Returns the configured pass count before or after generation, not the number of generated polylines. Access is `run.num_passes`, not `run.num_passes()`.
- `get_lines() -> list[list[tuple[float, float]]]`: returns a fresh copy of the generated polylines. Each outer item is a line; each line contains `(x, y)` tuples in millimeters. Returns `[]` before generation.
- `to_svg(filename) -> None`: writes the generated lines with stroke width `0.05`, millimeter document dimensions, and a 5 mm bounds margin. `filename` is a string path.

Changing the list returned by `get_lines()` does not change the run or later
exports. Saved lists also remain unchanged when the run is regenerated.
Segments are separate lines; the outer list is not a list of pass objects.
Clipping, segmentation, skipped rings, and specialized presets can make its
length differ from `num_passes`.

`to_svg()` requires a prior `generate()` call and raises `OSError` if the
pattern is ungenerated, has no points, contains non-finite coordinates, or
cannot be written. `get_lines()` does not perform these export checks.
There are no Python `to_stl()` or `to_step()` methods on a run, including
runs returned by presets. See [Exports](exports.md) and [Errors](errors.md).

`repr(run)` reports its center and configured pass count. Individual pass
objects and their width-offset edges are not exposed in Python.

### Run presets

The following eight static factories return new, ungenerated
`RoseEngineLatheRun` objects. They create their own config and a
`CuttingBit.v_shaped(30.0, 0.02)`; they do not accept a config or bit argument.
All use one unsegmented path per standard pass, or specialized line
generation. They share the run's SVG-only export interface.

These factories do not call the corresponding pattern-layer constructors,
so they do not inherit all of those layers' validation. The checks below
describe the run bindings specifically; see [Pattern layers](patterns.md)
for the separate layer API.

#### `draperie()`

```python
RoseEngineLatheRun.draperie(
    num_rings=96,
    base_radius=22.0,
    radius_step=0.44,
    wave_frequency=12.0,
    phase_shift=None,
    phase_oscillations=2.5,
    resolution=1500,
    phase_exponent=3,
    wave_exponent=1,
    circular_phase=2.0,
    center_x=0.0,
    center_y=0.0,
)
```

Creates concentric wavy rings centered as a band around `base_radius`, with
`num_passes == num_rings`. Radii and spacing are in millimeters;
`wave_frequency` is undulations per revolution. `phase_shift=None` resolves
to `pi / 12` radians, not zero. The initial primary phase is
`pi / 2 + pi / (2 * wave_frequency)` for peak alignment at SVG 12 o'clock.
Phase-envelope arguments have the meanings defined for the general run;
`wave_exponent` shapes each ring's wave independently. Each successful ring
has `resolution + 1` samples.

There is no amplitude argument. The factory uses `DraperieConfig`'s
automatic amplitude calculation: 60% of the smaller of the sampled
adjacent-ring phase limit and an innermost-radius limit of 90% of that
radius (zero if the innermost radius is nonpositive). This is the
implementation's amplitude-selection rule, not a guarantee for arbitrary
unchecked parameters.

Construction checks positive ring count, finite positive base radius,
`resolution >= 10`, finite centers, and the derived config amplitude and
phase. It does not separately validate every spacing or envelope argument.
Zero `wave_frequency` makes the derived phase non-finite and is rejected.
Nonpositive inner-ring radii can still cause passes to be skipped during
generation. With `radius_step=0`, the run uses phase-rotation mode rather
than concentric-ring mode.

#### `paon()`

```python
RoseEngineLatheRun.paon(
    num_lines=500,
    radius=22.0,
    amplitude=0.035,
    wave_frequency=10.0,
    phase_rate=9.0,
    resolution=800,
    n_harmonics=3,
    fan_angle=4.0,
    vanishing_point=0.3,
    center_x=0.0,
    center_y=0.0,
)
```

Generates fan lines sampled from SVG top to bottom, with horizontal
oscillation, retaining samples inside a circle of `radius`.
`num_passes == num_lines`, but lines with fewer than two retained samples
are omitted. Each candidate line has `resolution + 1` samples before
clipping.

`amplitude` is in millimeters. `wave_frequency` multiplies a logarithmic
distance coordinate from the vanishing point; it is not a fixed number of
cycles along every clipped line. `phase_rate` controls the absolute-sine
phase envelope across the fan. Despite its name, `fan_angle` is phase
amplitude in full wave cycles, not an angle in radians: the per-line phase
is `-2 * pi * fan_angle * abs(sin(pi * phase_rate * fraction))`.
`vanishing_point` is distance beyond the SVG bottom edge as a fraction of
the diameter; the actual fan spread is computed from this point and the
circle. A finite positive value avoids the logarithm's zero or negative
reference distance.

`n_harmonics=0` selects a sine wave. Positive values use a normalized
triangle-wave Fourier approximation with `n_harmonics + 1` odd-harmonic
terms. `n_harmonics` is an integer.

Construction checks positive line count, finite positive radius, finite
nonnegative amplitude, and finite centers. The requested `resolution`,
wave/phase controls, `fan_angle`, and `vanishing_point` are stored without
the separate `PaonLayer` checks; in particular, zero resolution or a
nonpositive vanishing point is not rejected here. The internal radial
config retains resolution `1000`, which does not validate the specialized
line resolution.

#### `diamant()`

```python
RoseEngineLatheRun.diamant(
    num_circles=72,
    circle_radius=20.0,
    resolution=360,
    center_x=0.0,
    center_y=0.0,
)
```

Generates `num_circles` circles, each of radius `circle_radius` and passing
through the pattern center. Circle centers are at distance `circle_radius`
from that center, evenly distributed over a full turn. The overall extent
can reach twice `circle_radius`; this is not a clipping radius.
`num_passes == num_circles` and each circle has `resolution + 1` samples.

The preset uses Cartesian circle sampling, not
`RosettePattern.diamant()` or `RoseEngineConfig.diamant()`. Construction
checks positive circle count, finite positive circle radius, and finite
centers. The requested resolution is stored separately and is not checked
against the config's minimum of 10.

#### `limacon()`

```python
RoseEngineLatheRun.limacon(
    num_curves=72,
    base_radius=20.0,
    amplitude=20.0,
    resolution=360,
    center_x=0.0,
    center_y=0.0,
)
```

Uses the standard phase-rotation mode with a frequency-1 sinusoidal rosette
and no segmentation. Curve `i` follows
`r = base_radius + amplitude * sin(angle + 2 * pi * i / num_curves)`.
`num_passes == num_curves`; each curve has `resolution + 1` samples.
Base radius and amplitude are in millimeters. Amplitude may equal or exceed
base radius; zero or negative instantaneous radii are not clamped.

Construction checks positive curve count, finite positive base radius,
finite nonnegative amplitude, `resolution >= 10`, and finite centers. Unlike
the Cartesian diamant and huit-eight presets, this preset assigns the
requested resolution to the config that is validated.

#### `flinque()`

```python
RoseEngineLatheRun.flinque(
    radius=10.0,
    num_petals=12,
    num_waves=60,
    wave_amplitude=0.8,
    wave_frequency=20.0,
    inner_radius_ratio=0.05,
    center_x=0.0,
    center_y=0.0,
)
```

Generates concentric chevron rings with fine radial ripple.
`num_passes == num_waves`. Base ring radii use half-step positions uniformly
between `radius * inner_radius_ratio` and `radius`. A ring is skipped when
its base radius is less than `0.1 * wave_amplitude`.

`num_petals` is the chevron lobe count. Each retained ring has
`num_petals * 80 + 1` samples; there is no resolution argument.
`wave_amplitude` is in millimeters. The radial addition is
`wave_amplitude * abs(sin(petal_phase))` plus a ripple of amplitude
`0.05 * wave_amplitude` and phase `petal_phase * wave_frequency`, where
`petal_phase = angle * num_petals / 2`. Thus `wave_frequency` is a multiplier,
and `radius` controls the base ring range rather than clipping the peaks.

Construction checks positive ring count, finite positive radius, finite
nonnegative half-amplitude, and finite centers. It does not check positive
`num_petals`, the range of `inner_radius_ratio`, or finite `wave_frequency`.
Zero petals can produce non-finite coordinates during generation.

#### `huiteight()`

```python
RoseEngineLatheRun.huiteight(
    num_curves=72,
    scale=20.0,
    resolution=360,
    center_x=0.0,
    center_y=0.0,
    num_clusters=0,
    cluster_spread=0.0,
)
```

Generates Cartesian lemniscates of Bernoulli through the pattern center.
`scale` is the half-width in millimeters. `num_passes == num_curves`, and
each curve has `resolution + 1` samples. The spelling is `huiteight()`,
unlike the rosette and config factories named `huit_eight()`; those radial
factories do not generate this lemniscate.

Uniform orientations span a half-turn, `pi * i / num_curves`, because a
lemniscate has 180-degree rotational symmetry. Clustering is active only
when `0 < num_clusters < num_curves`. Curves are distributed evenly among
clusters, with any remainder assigned to the earliest clusters. Cluster
centers are separated by `pi / num_clusters` radians. A positive
`cluster_spread` is the full angular span of each cluster; otherwise the
span is automatically `pi / (2 * num_clusters)`. Zero clusters, or a cluster
count at least as large as the curve count, selects uniform orientations.

Construction checks positive curve count, finite positive scale, and finite
centers. It does not validate the separately stored resolution or cluster
spread. The Python run preset defaults to 72 curves; the Rust
`HuitEightConfig` default is not this factory's default.

#### `clous_de_paris()`

```python
RoseEngineLatheRun.clous_de_paris(
    spacing=1.0,
    radius=22.0,
    angle=0.7853981633974483,
    resolution=200,
    center_x=0.0,
    center_y=0.0,
)
```

Generates two perpendicular sets of parallel straight lines clipped to a
circle. `spacing` is perpendicular line spacing in millimeters; `radius`
is the clipping radius in millimeters. `angle` rotates the grid in radians;
its default is `pi / 4` (45 degrees). Each line has `resolution + 1` samples,
including its circle-boundary endpoints. Tangent lines may contain
coincident points.

`num_passes` is always `1` for this preset, not the grid's line count.
Construction checks finite positive radius and finite centers. It does not
validate `spacing`, `angle`, the requested resolution, or the derived line
count. Finite positive spacing and a nonzero resolution are required for
the generation formulas; zero or extremely small spacing can cause
excessive iteration rather than an early `ValueError`.

#### `cube()`

```python
RoseEngineLatheRun.cube(
    spacing=0.5,
    radius=22.0,
    angle=0.0,
    resolution=200,
    cuts_per_group=8,
    gap_per_group=8,
    amplitude=0.0,
    leg_angle=30.0,
    center_x=0.0,
    center_y=0.0,
)
```

Generates parallel zigzag lines grouped in sets of `cuts_per_group`, with
`gap_per_group` line spacings between groups. Alternating groups shift by
half a zigzag period. Lines are clipped geometrically to the circle of
`radius`, then rotated by `angle` radians and translated to the supplied
center. The implementation does not generate three independent line
families or a 3D cube mesh.

`spacing`, `radius`, and `amplitude` are in millimeters. `amplitude` is half
the peak-to-trough zigzag height. Positive amplitude is used directly;
nonpositive amplitude selects `(gap_per_group + 1) * spacing / 2`.
`leg_angle` is in degrees, with period
`4 * effective_amplitude / tan(leg_angle_in_radians)`.
`resolution` is retained for API compatibility but does not control the
zigzag vertices or clipping. `cuts_per_group` and `gap_per_group` are
integer counts.

`num_passes` is always `1`, not the number of clipped polylines.
Construction checks finite positive radius and finite centers. It does not
perform `CubeLayer`'s checks on spacing, group counts, amplitude, rotation,
leg angle, or derived group count. Finite positive spacing, positive group
counts, and a finite leg angle strictly between 0 and 90 degrees describe
the intended nondegenerate geometry, not enforced run-constructor checks.
Invalid or very small spacing can cause excessive iteration during
generation.

### Call sequence examples

Config mutation and generation are separate calls. Export follows
generation:

```python
from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe, RosettePattern

config = RoseEngineConfig(base_radius=20.0, amplitude=0.5)
config.set_rosette(RosettePattern.multi_lobe(lobes=12))
config.set_resolution(1500)
config.with_secondary_rosette(RosettePattern.sinusoidal(3.0), amplitude=0.1)
config.with_depth_modulation(amplitude=0.5, frequency=2.0)
bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

lathe = RoseEngineLathe(config, bit)
lathe.generate()
lathe.to_svg("rose.svg")
lathe.to_stl("rose.stl", depth=0.1)
```

Run data access returns polylines; the pass count is a property:

```python
from turtles import RoseEngineLatheRun

run = RoseEngineLatheRun.huiteight(num_curves=36, scale=10.0)
assert run.num_passes == 36
assert run.get_lines() == []
run.generate()
lines = run.get_lines()
x, y = lines[0][0]
run.to_svg("huit-eight.svg")
```
