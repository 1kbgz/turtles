# How to position and compose patterns

Use `WatchFace` to combine direct pattern layers and spirographs on one dial. Use
a standalone layer when you only need one motif or want to inspect its sampled
lines. Install the package first using [the installation guide](install.md).

All lengths below are in millimeters. `WatchFace` requires a radius from 26 to
44 mm inclusive: `radius=38.0` means a 76 mm diameter, not a 38 mm diameter.
Standalone layers can be smaller.

## Combine a background and an offset motif

Construct a layer at its intended position, pass it to `face.add()`, and call
`face.generate()` after adding the layers. Enable dial decorations separately.

```python
from turtles import ClousDeParisLayer, WatchFace

face = WatchFace(radius=38.0)
face.add_inner()
face.add_outer()
face.add_center_hole()
face.add_flinque(radius=30.0, num_waves=16, wave_amplitude=0.3)

motif = ClousDeParisLayer.at_clock(
    hour=3, minute=0, distance=18.0,
    spacing=0.8, radius=8.0, resolution=2,
)
face.add(motif)
face.add_hole(hour=6, minute=0, radius=22.0, hole_radius=1.0)
face.generate()
face.to_svg("composed-dial.svg")

assert not motif.get_lines()
motif.generate()
assert motif.get_lines()
```

The last three lines check the copy behavior: adding a layer reconstructs an
independent internal copy of its configuration and center. Generating the watch
face does not generate the original `motif`. You do not need to generate a
standalone object before adding it, and generating it afterward does not update
the copy already inside the face.

For the hole, `radius` is the distance from the dial center to the hole center;
`hole_radius` is the hole's size. These circles are SVG decorations, not drilled
features in STEP or STL.

## Choose a positioning convention

Use keyword arguments for positioned factories: argument order differs between
layer classes. Direct layers provide these placement choices:

- `with_center()` places the center at explicit x/y coordinates.
- `at_polar()` places the center at an angle and distance from the origin.
- `at_clock()` places the center along the hour hand at a specified time.

This example creates three independent standalone drawings:

```python
import math

from turtles import ClousDeParisLayer

layers = {
    "cartesian-motif": ClousDeParisLayer.with_center(
        center_x=-16.0, center_y=0.0,
        spacing=1.0, radius=6.0, resolution=2,
    ),
    "polar-motif": ClousDeParisLayer.at_polar(
        angle=-math.pi / 2, distance=16.0, grid_angle=0.0,
        spacing=1.0, radius=6.0, resolution=2,
    ),
    "clock-motif": ClousDeParisLayer.at_clock(
        hour=3, minute=30, distance=16.0,
        spacing=1.0, radius=6.0, resolution=2,
    ),
}

for name, layer in layers.items():
    layer.generate()
    assert layer.get_lines()
    layer.to_svg(f"{name}.svg")
```

Positive x points right and positive y points down in SVG. Polar angles use
radians: zero points right, and `-math.pi / 2` points up. Clock placement uses
12:00 at the top and 3:00 at the right. At 3:30 the center is halfway between
the 3 and 4 hour positions; the minute value does not select the minute-hand
direction.

Placement moves the center without turning the motif to face outward. For Clous
de Paris, `at_polar(angle=...)` chooses the center direction while `grid_angle`
chooses grid rotation. In its ordinary constructor and `at_clock()`, `angle`
instead means grid rotation. Check the
[pattern reference](../reference/patterns.md) for the corresponding factory
arguments of other families.

If you need to check a position before composing it, inspect `center_x` and
`center_y` on the standalone layer. A fitted standalone SVG can center the motif
in its canvas even when its stored coordinates are offset.

## Add a positioned texture without keeping a layer object

Use the public wrapper's convenience methods when you do not need the original
standalone object. Pass a size explicitly for a small offset motif; omitted size
arguments commonly default to the full watch-face radius.

```python
from turtles import WatchFace

face = WatchFace(radius=38.0)
face.add_inner()
face.add_clous_de_paris(
    spacing=1.0, radius=9.0,
    hour=9, minute=0, distance=17.0, resolution=2,
)
face.add_cube(
    spacing=0.8, radius=9.0,
    hour=3, minute=0, distance=17.0,
    cuts_per_group=4, gap_per_group=4,
)
face.generate()
face.to_svg("paired-textures.svg")
```

Use `face.add(layer)` for existing supported objects, or a specific method such
as `face.add_clous_de_paris(...)` to construct one inside the face. The public
Python wrapper has no generic `add_pattern()` or `add_layer()` method. Its
`add()` does not accept `GuillochePattern`, `RoseEngineLathe`, or
`RoseEngineLatheRun` objects.

## Position a spirograph in a combined pattern

For a centered spirograph, construct `HorizontalSpirograph`, `VerticalSpirograph`,
or `SphericalSpirograph` and pass it to `WatchFace.add()`. The standalone Python
spirograph classes do not expose the direct layers' placement factories.

For an offset spirograph, use the separate `GuillochePattern` API's positioned
adder:

```python
from turtles import GuillochePattern

pattern = GuillochePattern(radius=38.0)
pattern.add_layer_at_clock(
    spiro_type="horizontal",
    outer_radius=8.0, radius_ratio=0.7, point_distance=0.8,
    rotations=7, resolution=120,
    hour=3, minute=0, distance=16.0,
)
pattern.generate()
pattern.to_svg("offset-spirograph.svg")
```

Use `add_layer_at_polar()` instead if you need polar placement. Do not transfer
that method name to the public `WatchFace` wrapper; the two Python composition
APIs expose different methods. See the
[composition reference](../reference/composition.md) for supported families and
signatures.

## Check overlaps and regenerate before export

After adding more layers, call the composition's `generate()` again before
exporting. Exports do not generate geometry automatically. Calls such as `add()`
and `generate()` return `None`; keep them as separate statements.

If a motif is cut off at the dial edge, reduce its size or center distance.
Combined SVG masks patterns at the dial circle; combined STEP and STL trim the
sampled paths there. Placement does not resize a motif to fit. For a locally
circular motif, keep its center distance plus local radius within the dial
radius if you want the whole circle visible. For flinqué, diamant, limaçon, and
draperie, check the generated extent rather than assuming that a size parameter
is an outer clipping radius.

If overlapping motifs appear in an unexpected order, check
[the fixed family painting order](../reference/composition.md#clipping-and-draw-order).
Insertion order is preserved within a family, but not across all families. Adding
a background last does not necessarily paint it last. An offset motif also does
not erase background paths beneath it.

Use positioned layers for texture motifs, not as substitutes for unfinished dial
elements: `WatchFace.add_subdial()` and `WatchFace.add_window()` currently raise
`NotImplementedError`. For boundary and unit details, read
[coordinates and boundaries](../explanation/coordinates.md). To write CAD or mesh
files, continue with [the export guide](export.md).
