# Exports

This page describes the public Python export methods and the geometry they
write. Rust exporters use the same implementations but accept `ExportConfig`
where applicable; see [Rust API](rust.md). Construction and composition APIs are
listed in [Patterns](patterns.md), [Composition](composition.md), and
[Rose engine](rose-engine.md).

## Available methods

“Not exposed” means the Python class has no such method. The single-lathe STEP
entry is an exposed method that fails, not a working exporter.

| Python class           | SVG        | STL         | STEP                                                    | Coordinate access | Batch export   |
| ---------------------- | ---------- | ----------- | ------------------------------------------------------- | ----------------- | -------------- |
| `FlinqueLayer`         | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `DiamantLayer`         | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `LimaconLayer`         | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `DraperieLayer`        | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `PaonLayer`            | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `HuitEightLayer`       | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `ClousDeParisLayer`    | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `CubeLayer`            | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |
| `HorizontalSpirograph` | `to_svg()` | `to_stl()`  | `to_step()`                                             | Not exposed       | Not exposed    |
| `VerticalSpirograph`   | `to_svg()` | `to_stl()`  | `to_step()`                                             | Not exposed       | Not exposed    |
| `SphericalSpirograph`  | `to_svg()` | `to_stl()`  | `to_step()`                                             | Not exposed       | Not exposed    |
| `WatchFace`            | `to_svg()` | `to_stl()`  | `to_step()`                                             | Not exposed       | Not exposed    |
| `GuillochePattern`     | `to_svg()` | `to_stl()`  | `to_step()`                                             | Not exposed       | `export_all()` |
| `RoseEngineLathe`      | `to_svg()` | `to_stl()`  | `to_step()` always raises `OSError` for valid arguments | Not exposed       | Not exposed    |
| `RoseEngineLatheRun`   | `to_svg()` | Not exposed | Not exposed                                             | `get_lines()`     | Not exposed    |

All run presets return `RoseEngineLatheRun` objects with the same SVG and
coordinate-access methods. They do not acquire STL or STEP methods.
Standalone decorative layers can contribute to combined STL and STEP through
`WatchFace`. Python `GuillochePattern` accepts only the subset listed in
[Composition](composition.md); its Rust implementation supports all eight
families.

## Signatures and filenames

The following are method signatures, not executable examples. Each signature
applies only to classes exposing that method in the table above.

```text
to_svg(filename: str) -> None
to_stl(filename: str, depth=0.1, base_thickness=2.0) -> None
to_step(filename: str, depth=0.1) -> None
GuillochePattern.export_all(base_name: str, depth=0.1, base_thickness=2.0) -> None
get_lines() -> list[list[tuple[float, float]]]
```

Exports write files and return `None`; they do not return SVG strings, meshes,
or file contents. `filename` and `base_name` must be strings. Bindings accept
Rust `&str`, not Python's general filesystem-path protocol; a `pathlib.Path`
requires conversion with `str()`. Paths are relative to the process's working
directory unless absolute. Exporters do not create parent directories.
Existing destinations can be overwritten. Single-file methods use the exact
filename supplied, without appending a suffix or selecting a format from it.

`export_all()` writes **three combined files**, not one file per layer:

1. `base_name + ".svg"`, using the combined SVG exporter.
1. `base_name + ".stl"`, using the combined STL exporter.
1. `base_name + ".stp"`, using the combined STEP exporter.

Suffixes are appended literally: a base name ending in `.svg` still receives
all three suffixes. An empty composition raises `OSError` before any batch
write. Otherwise, writes occur in the order above and stop at the first error;
there is no rollback or atomic batch replacement. Ungenerated layers can leave
a successfully written SVG followed by an STL failure.

## Generation and line access

Construction does not generate geometry. `generate()` replaces stored geometry;
no export method invokes it automatically. In Python, adding an existing layer
to a composition reconstructs an ungenerated internal copy. Generating the
composition does not populate the original object's lines, and generating the
original does not generate its internal copy. See [Composition](composition.md).

`get_lines()` on a decorative layer or run returns a fresh copy of its stored
polylines, in millimeters and including center offsets. It returns `[]` before
generation. Each inner list is a separate path; joining lists would add
connections absent from the stored geometry. Editing the result does not alter
the object. This method does not perform the file exporters' validation.

| Export route                       | Missing or empty geometry                                                                                                                                                     |
| ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Standalone decorative SVG          | Raises `OSError` when its stored line collection is empty, including when generation retained no lines.                                                                       |
| Standalone spirograph SVG/STL/STEP | Raises `OSError` when the relevant point collection is empty. STEP additionally requires at least two points in a path.                                                       |
| `WatchFace.to_svg()`               | Can write an empty drawing or enabled decorations without generated pattern paths.                                                                                            |
| `GuillochePattern.to_svg()`        | Can write its default dial decorations without generated pattern paths.                                                                                                       |
| Combined STL/STEP                  | Raises `OSError` when dial clipping leaves no path of at least two points.                                                                                                    |
| `RoseEngineLathe` SVG/STL          | Raises `OSError` before generation; SVG also checks bounds.                                                                                                                   |
| `RoseEngineLathe.to_step()`        | Before generation, raises `OSError` containing `Pattern not generated`. After generation, raises `OSError` containing `STEP export not yet implemented`. Writes no STEP file. |
| Run SVG                            | Raises `OSError` before generation or if its generated lines contain no points.                                                                                               |

Generation can skip rings or retain no lines. A successful `generate()` does not
guarantee nonempty or finite output. Bounds and coordinate validation also differ
by exporter; see [Errors](errors.md).

## Dimensions and export options

Lengths, coordinates, bit widths, `depth`, and `base_thickness` are expressed in
millimeters. SVG sets physical `width` and `height` in `mm`. STEP declares a
millimeter length unit. STL has no unit declaration; its numeric coordinates
are intended as millimeters. SVG document dimensions describe the canvas, not
necessarily the motif's diameter.

| Option                          | Default  | Implemented effect                                                                                                                   |
| ------------------------------- | -------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| `depth`                         | `0.1` mm | STL lowers a second copy of each path by this amount in z. Working STEP adapters ignore it. Single-lathe STEP remains unimplemented. |
| `base_thickness`                | `2.0` mm | Accepted by STL methods and `export_all()`, but unused by current mesh construction. No base plate is added.                         |
| Rust `ExportConfig.tool_radius` | `0.0` mm | Unused by current exporters. Python bindings always set it to zero and expose no corresponding argument.                             |

Python does not expose an `ExportConfig` class. STL bindings construct a config
from their arguments; STEP bindings set the unused base thickness and tool
radius to `2.0` and `0.0`. Export arguments are not checked for positivity or
finiteness. Negative, zero, or non-finite `depth` values are not rejected by an
explicit parameter check. Successful serialization does not validate the
resulting mesh.

## SVG contents and bounds

SVG contains stroked, sampled paths. Straight segments connect stored vertices;
exporters do not fit analytic circles or splines. Standalone decorative paths
use black strokes of width `0.05` mm. Spirograph paths use `0.1` mm.
Single-lathe SVG includes its centerline at `0.1` mm and two approximate
width-offset edges at `0.05` mm. Run SVG includes retained centerlines at
`0.05` mm, not every pass's offset edges.

| Exporter                     | Canvas and decorations                                                                                                               |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| Eight standalone layers      | Bounds of generated points plus a 5 mm margin on each side; no dial decorations.                                                     |
| Three standalone spirographs | Planar point bounds plus a 5 mm margin; each dimension's pre-margin extent has a floor of `0.1 * outer_radius`. No dial decorations. |
| Single lathe                 | Bounds of centerline and both edges plus a 5 mm margin.                                                                              |
| Multi-pass run               | Bounds of retained lines plus a 5 mm margin.                                                                                         |
| `WatchFace`                  | Fixed square of side `5 * radius`, centered at the origin. Includes enabled inner circle, bezel, and hole circles.                   |
| `GuillochePattern`           | Same fixed square; always includes the dial circle, bezel at `1.05 * radius`, and filled center-hole circle of radius `0.8` mm.      |

Both compositions mask pattern strokes to the origin-centered dial circle using
an SVG `clip-path`; underlying path coordinates remain untrimmed. Dial, bezel,
and hole decorations are outside that pattern group. Holes are filled circles
drawn over the paths, not Boolean removals of stored geometry. Draw order and
styling are listed in [Composition](composition.md).

Standalone output has no global dial clipping. Local boundaries depend on the
family: Clous de Paris and Cube trim to their local circles, Paon retains
samples inside its local circle, and the other five decorative families do not
clip to a nominal size parameter. Standalone spirographs and a single lathe
also have no dial-boundary clipping.

## STL contents

STL is a triangle mesh of vertical ribbons with zero width across each path.
Each segment produces two triangles between the original path and a second
copy lowered by `depth`. For planar paths the top is at `z=0` and the bottom
at `z=-depth`. Standalone spherical spirographs use their projected 3D points
and a copy at `point.z - depth`.

These ribbons have no stock, finite groove width, top/bottom caps, Boolean
unions, or watertight solid. All written triangle normals are the placeholder
`(0, 0, 1)`, not computed surface normals. Vertices are converted from Rust
`f64` to `f32` for STL writing.

- Standalone spirograph STL connects the final point back to the first, even
  for an incomplete trace. Its SVG and STEP do not add that closing segment.
- Combined STL clips each sampled segment geometrically to the dial circle,
  inserts circle intersections, and splits paths at exits and reentries. Each
  resulting piece is treated as open; no extra closing chord is added.
- Single-lathe STL follows consecutive centerline points without an extra
  closing segment. It ignores the generated offset edges, bit shape, bit width,
  bit depth, and depth modulation.

`WatchFace` delegates STL to its guilloché compositor. Enabled SVG dial, bezel,
and hole circles do not become stock or physical cuts. Combined exports include
every stored layer family, but use planar traces even for spherical
spirographs. The dome is retained only by standalone spherical STL/STEP.

## STEP contents

The working STEP writer emits ISO 10303-21 text with the AP214
`AUTOMOTIVE_DESIGN` schema. It creates referenced `CARTESIAN_POINT` entities,
one `POLYLINE` per drawable path, and a `GEOMETRIC_CURVE_SET` connected to the
product's shape representation. Paths with fewer than two points are skipped;
no drawable paths raises `OSError`. Non-finite coordinates in drawable paths
also raise `OSError`.

| Export route                        | Coordinates                                                                                                                         |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| Horizontal and vertical spirographs | Sampled planar trace at `z=0`. Vertical modulation changes y, not z.                                                                |
| Standalone spherical spirograph     | Generated dome-projected x, y, and z coordinates.                                                                                   |
| `GuillochePattern` and `WatchFace`  | All stored pattern families, geometrically clipped to the dial circle and placed at `z=0`. Spherical layers use their planar trace. |
| Single lathe                        | No STEP output; method is unimplemented.                                                                                            |

STEP output contains curves, not swept grooves, faces, shells, or
boundary-representation solids. It does not include SVG decorations, cutter
compensation, or a plate. Export configuration does not change its coordinates.
Repeated endpoints can describe a closed sampled path, but the writer does not
add an endpoint or fit an analytic curve.

None of these exporters produces CNC instructions, feeds, spindle settings,
entry/exit moves, or collision checks. File creation does not establish tool
clearance or machining suitability. See
[Geometry and exports](../explanation/geometry-and-exports.md) for the geometry
model and [How to export patterns](../how-to/export.md) for complete Python
examples.

Implementation sources: [spirograph exporters](https://github.com/1kbgz/turtles/blob/main/rust/src/spirograph.rs),
[combined exporters](https://github.com/1kbgz/turtles/blob/main/rust/src/guilloche.rs),
[watch-face exporter](https://github.com/1kbgz/turtles/blob/main/rust/src/watch_face.rs),
[STEP writer](https://github.com/1kbgz/turtles/blob/main/rust/src/step.rs), and
[single-lathe exporters](https://github.com/1kbgz/turtles/blob/main/rust/src/rose_engine/lathe.rs).
