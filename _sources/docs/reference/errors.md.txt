# Errors and validation

This page records Python exception types, constructor checks, and known gaps.
Parameter meanings and signatures are listed in [Patterns](patterns.md),
[Composition](composition.md), and [Rose engine](rose-engine.md).
[Exports](exports.md) describes file contents and export preconditions;
[Rust API](rust.md) describes native error handling.

## Python exception mapping

The bindings convert `SpirographError` variants through one shared mapping.
Python argument conversion happens before those Rust checks.

| Python exception      | Source or condition                                                                                                                                                                                                                          |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ValueError`          | Rust `InvalidRadius` or `InvalidParameter`. Also an unrecognized `spiro_type` in positioned spirograph composition methods.                                                                                                                  |
| `OSError`             | Rust `ExportError`: missing generated geometry, rejected SVG bounds or STEP coordinates, no drawable paths after clipping, file creation/write failures, or unimplemented single-lathe STEP export. `PyIOError` is Python's `OSError` alias. |
| `TypeError`           | Incompatible binding argument types, missing/extra arguments, unexpected keywords, or unsupported objects passed to a compositor's spirograph adder. Filename parameters require strings, not `pathlib.Path` objects.                        |
| `OverflowError`       | A negative Python integer or an integer outside the target Rust unsigned range supplied to a `usize` or `u32` parameter. This occurs before constructor value validation.                                                                    |
| `NotImplementedError` | Public Python `WatchFace.add_subdial()` and `WatchFace.add_window()` always raise this without rendering or validating the intended geometry.                                                                                                |

Rust error messages retain the `Display` prefix: `Invalid radius:`,
`Invalid parameter:`, or `Export error:`. For example, a finite dial radius
outside 26–44 mm produces `InvalidRadius`; a non-finite dial radius is rejected
by `validate_finite()` as `InvalidParameter`. Both become `ValueError`.

`GuillochePattern.add_layer()` and the binding behind `WatchFace.add()` accept
only the three spirograph classes when handling spirographs. The public
`WatchFace.add()` dispatches the eight decorative classes before that call;
other object types raise `TypeError`. Family-specific adders require their
matching layer class. Existing layer bindings reconstruct and validate a fresh
internal object rather than copying its generated geometry.

Positioned `GuillochePattern` spirograph adders lowercase `spiro_type` and accept
only `horizontal`, `vertical`, or `spherical`; whitespace is not stripped.
Unused variant options still undergo binding type conversion, but do not
receive another variant's numeric checks.

Missing methods, such as `RoseEngineLatheRun.to_stl()` or a direct layer's
`to_step()`, are absent APIs and normally cause `AttributeError` on lookup.
They are not `NotImplementedError` stubs. Conversely,
`RoseEngineLathe.to_step()` exists and reports its unimplemented state as
`OSError`, not `NotImplementedError`, for type-correct arguments.

## Constructor checks

In the tables below, **positive** means finite and strictly greater than zero;
**nonnegative** means finite and greater than or equal to zero. “Finite” rejects
NaN and both infinities. Counts bind to `usize` unless stated otherwise;
clock hours/minutes and draperie/rose-engine phase and wave exponents bind to
`u32`. Unsigned conversion is separate from these value checks.

The checks apply to construction, including the corresponding centered,
polar, and clock factories where exposed. They are not universal rules for
every API with a similarly named parameter.

### Compositors and spirographs

| API                                | Checks                                                                                                                                                                                                                                                        |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `WatchFace`, `GuillochePattern`    | Finite `radius` in `[26, 44]` mm inclusive. This is a radius, not a diameter, and does not restrict standalone layer sizes.                                                                                                                                   |
| All three spirographs              | Positive `outer_radius`; finite `radius_ratio` strictly between 0 and 1; nonnegative `point_distance`; finite center coordinates; `rotations >= 1`; `resolution >= 1`; checked product `rotations * resolution <= 1_000_000`, with integer overflow rejected. |
| Vertical spirograph, additionally  | Nonnegative `wave_amplitude`; finite `wave_frequency`.                                                                                                                                                                                                        |
| Spherical spirograph, additionally | Positive `dome_height`.                                                                                                                                                                                                                                       |

`point_distance > 1` is accepted. Zero and negative vertical wave frequencies
are accepted. In Python, standalone spirographs have no positioning factories;
positioned internal instances are available through `GuillochePattern`.

### Eight decorative layers

All eight validate finite `center_x` and `center_y`. The remaining checks are:

| Layer               | Checks                                                                                                                                                                                                                                                                                                                             |
| ------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `FlinqueLayer`      | Positive `radius`; nonnegative `wave_amplitude`; finite `wave_frequency`; `num_petals >= 1`; `num_waves >= 1`; `inner_radius_ratio` in `[0, 1]` inclusive, rejecting non-finite values.                                                                                                                                            |
| `DiamantLayer`      | Positive `circle_radius`; `num_circles >= 1`; `resolution >= 10`.                                                                                                                                                                                                                                                                  |
| `LimaconLayer`      | Positive `base_radius`; finite `amplitude`; `num_curves >= 1`; `resolution >= 10`. Negative amplitude is accepted.                                                                                                                                                                                                                 |
| `DraperieLayer`     | `num_rings >= 1`; positive `radius_step` and `base_radius`; finite `wave_frequency`, resolved `phase_shift`, and `phase_oscillations`; nonnegative `circular_phase`; `resolution >= 10`. Rust's optional explicit `amplitude`, when present, must be nonnegative; Python always uses automatic amplitude.                          |
| `PaonLayer`         | `num_lines >= 1`; positive `radius`, `fan_angle`, and `vanishing_point`; nonnegative `amplitude`; finite `wave_frequency` and `phase_rate`; `resolution >= 10`.                                                                                                                                                                    |
| `HuitEightLayer`    | `num_curves >= 1`; positive `scale`; nonnegative `cluster_spread`; `resolution >= 10`.                                                                                                                                                                                                                                             |
| `ClousDeParisLayer` | Positive `spacing` and `radius`; finite grid `angle`; `resolution >= 2`; finite `ceil(radius / spacing) <= 1_000_000`.                                                                                                                                                                                                             |
| `CubeLayer`         | Positive `spacing` and `radius`; nonnegative `amplitude`; finite grid `angle` and `leg_angle`; `resolution >= 2`; positive integer `cuts_per_group` and `gap_per_group`; `0 < leg_angle < 90` degrees; finite positive group cycle `(cuts_per_group + gap_per_group) * spacing`; finite `ceil(radius / group_cycle) <= 1_000_000`. |

There is no further positive-frequency requirement, upper curve/ring count,
positive draperie-exponent check, or maximum harmonic count in these
constructors. Huit-eight cluster count need not divide the curve count or be
smaller than it; inactive clustering selects uniform rotation. Cube validates
`resolution` even though its zigzag generation does not use that sample count.

### Rose-engine config, single lathe, and general run

`RoseEngineConfig` construction, its factories, and its Python mutation methods
store values without validating them. `RoseEngineLathe` and the general run
constructors validate the copied config:

- Positive `base_radius`.
- Nonnegative primary `amplitude` and `secondary_amplitude`.
- Finite primary `phase`, `secondary_phase`, `start_angle`, and `end_angle`.
- `resolution >= 10`.
- Finite generator center coordinates.

General runs also require `num_passes >= 1` and `segments_per_pass >= 1`.
The Python run constructor assigns `radius_step`, `phase_shift`,
`phase_oscillations`, `circular_phase`, and `phase_exponent` after those checks,
without further numeric validation. Python exposes no config `validate()`
method; Rust exposes `RoseEngineConfig::validate()`.

These config checks do not inspect rosette parameters, cutting-bit dimensions,
depth-modulation controls, angular ordering, or the minimum radius reached by
the curve. Bit factories do not reject invalid dimensions or aspect ratios.
`CuttingBit.v_shaped()` clamps the angle to 1–179 degrees, but does not validate
width; this is not a general bit-validation pass. Rosette factories do not
require positive counts or finite floating-point parameters. Elliptical and
rice-grain displacement have specific zero-displacement fallbacks described in
[Rose engine](rose-engine.md).

### Run presets have different validation coverage

Each preset constructs an internal radial config and calls the run constructor;
it does **not** construct the matching standalone decorative layer. All presets
check finite centers and a positive nominal radius or scale. Additional checks
and omissions are:

| Run preset         | Checks and omissions                                                                                                                                                                                                                          |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `draperie()`       | Positive ring count, requested `resolution >= 10`, valid derived config amplitude and phase. Zero frequency produces a non-finite derived phase and is rejected. Spacing and envelope arguments are not each validated as in `DraperieLayer`. |
| `paon()`           | Positive line count and nonnegative amplitude. Requested specialized resolution, frequencies, phase controls, `fan_angle`, and `vanishing_point` do not receive `PaonLayer` checks.                                                           |
| `diamant()`        | Positive circle count. Requested specialized resolution is not checked against the minimum of 10.                                                                                                                                             |
| `limacon()`        | Positive curve count, nonnegative amplitude, requested `resolution >= 10`. Unlike `LimaconLayer`, negative amplitude is rejected.                                                                                                             |
| `flinque()`        | Positive ring count and nonnegative half-amplitude (`wave_amplitude / 2`). Petal count, wave frequency, and inner-radius ratio do not receive `FlinqueLayer` checks.                                                                          |
| `huiteight()`      | Positive curve count. Requested specialized resolution and cluster spread do not receive `HuitEightLayer` checks.                                                                                                                             |
| `clous_de_paris()` | Internal pass count is one. Spacing, grid angle, requested resolution, and derived line count do not receive `ClousDeParisLayer` checks.                                                                                                      |
| `cube()`           | Internal pass count is one. Spacing, group counts, amplitude, rotation, leg angle, and derived group count do not receive `CubeLayer` checks.                                                                                                 |

In presets that store a specialized resolution separately, the internal radial
config retains its valid default resolution; validating that config does not
validate the requested specialized resolution. Zero or extremely small spacing
in the run grid presets can cause excessive iteration rather than an early
`ValueError`.

## Count limits and arithmetic gaps

Rust defines `MAX_ELEMENTS = 1_000_000`, but this is **not a universal geometry
or memory limit**. Its enforced uses are the spirograph sample product and the
two direct-layer derived counts above. `checked_element_count()` rejects
non-finite, negative, or greater-than-limit floating-point counts and converts
accepted values to `usize`; the layer callers supply a ceiling.

The Clous de Paris limit applies to one half-extent count, not all grooves in
both directions or their sample count. Cube checks a group count, not the
number of cuts or zigzag vertices; generation also pads that count by two
groups on each side. Neither check bounds every allocation. Other layer and
run counts do not acquire this limit.

Not every integer operation uses checked arithmetic. For example, Cube's
constructor adds the two `usize` group counts before conversion to `f64`, and
flinqué generation multiplies petal count by 80. Extremely large accepted
integers can therefore overflow internal arithmetic or exhaust resources;
they are not guaranteed to become a documented `ValueError` or Python
`OverflowError`. Binding `OverflowError` describes conversion failure, not
arbitrary Rust arithmetic failure.

## Generation and export failures

Python `generate()` methods return `None`, not a validation report. General
rose-engine run generation skips passes whose derived configs fail validation,
such as concentric rings with nonpositive base radius. Rust records messages in
`generation_errors()`; Python does not expose those messages. Segmentation,
local clipping, and skipped rings can also reduce or empty line output without
a Python exception. A configured pass count is not a generated-line count.

| Export check                                | Scope                                                                                                                                                                                                       |
| ------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Missing generation/empty stored collections | Standalone layer and spirograph exporters; lathe and run exporters also check their generated flag.                                                                                                         |
| No drawable geometry after dial clipping    | Combined STL and STEP. A path needs at least two points; identical points are not rejected as degenerate.                                                                                                   |
| No layers in batch                          | `GuillochePattern.export_all()` rejects this before writing. With layers present, it writes SVG, STL, then STEP and can leave partial output.                                                               |
| Empty or non-finite SVG bounds              | Shared `compute_bounds()` used by standalone spirographs, standalone flinqué, single lathe, and multi-pass run. Other decorative SVG exporters use local bounds calculations without this shared rejection. |
| Non-finite STEP coordinates                 | Shared STEP writer checks coordinates of retained paths with at least two points. Shorter paths are skipped before coordinate serialization.                                                                |
| Unimplemented STEP                          | Single lathe always fails for valid arguments: missing-generation error first, unimplemented-export error after generation.                                                                                 |
| Filesystem failure                          | Exporters convert file creation or write errors into `ExportError`, hence `OSError`. A preceding geometry check can fail before any filesystem access.                                                      |

Combined SVG does not require generated paths and does not use the shared
bounds check. It can write decorations alone. STL exporters do not provide a
general finite-coordinate, positive-depth, or watertight-mesh check. Export
`depth`, `base_thickness`, and Rust `tool_radius` have no numeric validation;
unused options can accept non-finite values without changing output.

## Other validation gaps

- Clock positions use `hour % 12` and include the minute contribution. The
  conventional hour 1–12 and minute 0–59 ranges are not enforced beyond `u32`
  conversion. Placement distance need not be nonnegative or within a dial;
  layer constructors check the resulting center coordinates for finiteness.
- Watch-face hole addition does not validate finite/positive hole radius,
  finite distance, or containment. Hole circles are SVG decorations, not
  physical cuts in STL or STEP.
- `DraperieLayer` accepts zero frequency, but generation divides by frequency
  when computing phase and can produce non-finite coordinates. It also does
  not require positive innermost ring radius.
- Finite input checks do not guarantee finite derived arithmetic for extreme
  values, nonintersecting curves, valid stock, or tool clearance. SVG masks and
  geometric dial clipping are not manufacturing validation.

For call sequences and output checks, see [How to export patterns](../how-to/export.md)
and [How to position and compose patterns](../how-to/position-and-compose.md).
The [coordinate model](../explanation/coordinates.md) and
[geometry model](../explanation/geometry-and-exports.md) describe boundary and
representation differences.

Implementation sources: [shared checks and errors](https://github.com/1kbgz/turtles/blob/main/rust/src/common.rs),
[exception mapping](https://github.com/1kbgz/turtles/blob/main/rust/python/errors.rs),
[run constructors and generation](https://github.com/1kbgz/turtles/blob/main/rust/src/rose_engine/lathe_run.rs),
and [public Python wrapper](https://github.com/1kbgz/turtles/blob/main/turtles/__init__.py).
