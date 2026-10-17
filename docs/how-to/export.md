# How to export patterns

Generate geometry before exporting, then choose the output for the next task:

- Use SVG to inspect paths, placement, and dial decorations.
- Use STEP to exchange sampled curves with CAD software where the exporter is
  implemented.
- Use STL to inspect the library's approximate triangle ribbons, not to obtain a
  watertight dial or finished engraving.
- Use a direct layer's or multi-pass run's `get_lines()` to process sampled
  polylines yourself.

These examples require [an installed package](install.md). Every Python block is
self-contained and writes only under the current working directory. Exporters
take string filenames; convert a `pathlib.Path` with `str()`.

## Export a watch-face composition

Add the desired layers, generate the composition, and call each exporter
explicitly. Create the output directory yourself before writing into it.

```python
from pathlib import Path

from turtles import WatchFace

output = Path("exports")
output.mkdir(exist_ok=True)

face = WatchFace(radius=38.0)
face.add_inner()
face.add_outer()
face.add_center_hole()
face.add_clous_de_paris(spacing=1.5, radius=30.0, resolution=2)
face.generate()
face.to_svg(str(output / "dial.svg"))
face.to_step(str(output / "dial.step"), depth=0.1)
face.to_stl(str(output / "dial.stl"), depth=0.1, base_thickness=2.0)
```

Use the SVG to check the arrangement before opening the other files in a CAD or
mesh viewer. STEP and STL include the generated paths from every stored pattern
family, trimmed to the dial circle. SVG uses a circular mask instead of removing
the outlying path coordinates.

Do not expect the SVG's dial circle, bezel, or filled holes to appear as physical
features in STEP or STL. Those exporters do not create a plate or subtract holes.
Changing STEP `depth` does not change the exported curve coordinates. Changing
STL `depth` changes ribbon height; `base_thickness` is accepted but does not build
a base plate.

## Export one direct layer or retrieve its lines

For an isolated motif, use its own `to_svg()` and `get_lines()` methods. The
returned value is a list of polylines, each containing `(x, y)` tuples in
millimeters. Keep the lines separate when processing them; joining them would
insert connecting segments that are not part of the pattern.

```python
import json
from pathlib import Path

from turtles import ClousDeParisLayer

layer = ClousDeParisLayer(spacing=1.0, radius=10.0, resolution=2)
layer.generate()
layer.to_svg("standalone-layer.svg")
lines = layer.get_lines()
assert lines
Path("standalone-lines.json").write_text(json.dumps(lines), encoding="utf-8")
```

JSON represents the coordinate tuples as arrays. If you need STEP or STL for a
direct layer, add it to a `WatchFace` and generate the face, as in the previous
section. Direct layer classes do not have standalone `to_step()` or `to_stl()`
methods. Use a face radius within its supported 26–44 mm range even if the layer
itself is smaller.

## Preserve a spherical spirograph's 3D coordinates

All three standalone spirograph classes support SVG, STEP, and STL. Export a
`SphericalSpirograph` directly if you need to retain its projected dome in STEP
or STL:

```python
from turtles import SphericalSpirograph

spirograph = SphericalSpirograph(
    outer_radius=12.0, radius_ratio=0.7, point_distance=0.8,
    rotations=7, resolution=120, dome_height=2.0,
)
spirograph.generate()
spirograph.to_svg("spherical-preview.svg")
spirograph.to_step("spherical-curves.step", depth=0.1)
spirograph.to_stl("spherical-ribbons.stl", depth=0.1, base_thickness=2.0)
```

The SVG remains a planar preview. Adding that spirograph to `WatchFace` or
`GuillochePattern` uses its 2D trace for combined exports, so combined STEP/STL
does not retain the dome. Standalone spirograph exports do not acquire the
composition's dial-boundary clipping.

If standalone STL shows a straight connecting wall that SVG does not show,
inspect the first and last samples: the standalone STL exporter connects the end
back to the start even when the trace is incomplete. SVG and STEP do not force
that closure. See [geometry and exports](../explanation/geometry-and-exports.md)
before relying on a mesh's apparent shape.

## Export a single rose-engine pass

Configure the rosette and cutting bit before constructing `RoseEngineLathe`,
then generate and export it. Its implemented file exporters are SVG and STL.

```python
from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

config = RoseEngineConfig.classic_multi_lobe(
    base_radius=12.0, lobes=8, amplitude=0.5,
)
config.set_resolution(120)
bit = CuttingBit.v_shaped(angle=60.0, width=0.3)
lathe = RoseEngineLathe(config, bit)
lathe.generate()
lathe.to_svg("single-pass.svg")
lathe.to_stl("single-pass.stl", depth=0.1, base_thickness=2.0)
```

`RoseEngineLathe.to_step()` exists but currently raises `OSError` because STEP
export is unimplemented. Its STL follows the centerline; the cutting bit's width
and the SVG's approximate cut edges do not become a swept groove in that mesh.
The single-lathe Python class does not expose `get_lines()`.

## Export a multi-pass run and inspect its lines

Use `RoseEngineLatheRun.to_svg()` for the combined drawing and `get_lines()` for
its sampled paths. The run does not expose STEP or STL methods and cannot be
added directly to a `WatchFace`.

```python
import json
from pathlib import Path

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun

config = RoseEngineConfig.classic_multi_lobe(
    base_radius=12.0, lobes=8, amplitude=0.5,
)
config.set_resolution(120)
bit = CuttingBit.v_shaped(angle=60.0, width=0.3)
run = RoseEngineLatheRun(config, bit, num_passes=4, segments_per_pass=8)
run.generate()
run.to_svg("multi-pass.svg")
lines = run.get_lines()
assert lines
Path("multi-pass-lines.json").write_text(json.dumps(lines), encoding="utf-8")
```

If you need a direct pattern's geometry through the engine model rather than a
general segmented run, choose one of the dedicated family factories listed in
the [rose-engine reference](../reference/rose-engine.md). Those factory results
have the same SVG and line-inspection interface, not additional CAD exporters.

## Write all combined formats with one call

For a `GuillochePattern`, use `export_all()` with an extension-free base name.
`WatchFace`, standalone layers, spirographs, and rose-engine objects do not expose
this convenience method.

```python
from turtles import FlinqueLayer, GuillochePattern

pattern = GuillochePattern(radius=38.0)
pattern.add_flinque_layer(FlinqueLayer(radius=12.0, num_waves=8))
pattern.generate()
pattern.export_all("combined-pattern", depth=0.1, base_thickness=2.0)
```

This writes combined SVG, STL, and STEP files in that order. The STEP suffix is
`.stp`; it does not write a separate file per layer. A failure can leave earlier
files on disk, so check all expected outputs after an interrupted export rather
than treating one existing SVG as proof that the batch completed.

## Check output before using it downstream

If an SVG contains decorations but no texture, call `generate()` after adding
the layers. An ungenerated composition can still write SVG decorations; STL and
STEP reject a composition without drawable paths after clipping. For an isolated
layer or run, generate that original object rather than assuming that generating
a composition also populated it.

If export raises `OSError`, check the error message, generation state, destination
directory, and write permissions. Invalid construction parameters normally raise
`ValueError` instead. Refer to [the error reference](../reference/errors.md) for
the distinction; the unimplemented single-lathe STEP exporter is also reported
as an export error.

Before CAD exchange or manufacturing work:

1. Confirm scale in the receiving application. SVG and STEP declare millimeters;
   STL contains coordinate values intended as millimeters but has no unit field.
1. Inspect clipping, overlaps, endpoint connections, and sample density. Increase
   the relevant family's sampling parameter before regenerating if its curves
   appear faceted; not every `resolution` argument changes generated vertices.
1. Treat STEP as sampled polyline curves and STL as uncapped, zero-width ribbons.
   Neither output supplies finished stock, swept cuts, or a watertight solid.
1. Build and validate the required CAD/CAM model separately. These exporters do
   not produce machine programs or certify tool clearance.

See [the export reference](../reference/exports.md) for available options and
[geometry and exports](../explanation/geometry-and-exports.md) for the geometric
limits of each format.
