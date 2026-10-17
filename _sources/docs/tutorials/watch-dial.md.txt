# A patterned watch dial

In this tutorial, we will combine a hobnail background and an offset figure-eight
motif, then add a dial circle, bezel, and center-hole decoration.

Complete [your first pattern](first-pattern.md) first. Use the same activated
environment and a writable working directory.

## Draw the dial frame

Create a script named `watch_dial.py`:

```python
from turtles import WatchFace

face = WatchFace(radius=38.0)
face.add_inner()
face.add_outer()
face.add_center_hole()
face.to_svg("dial-frame.svg")
print("Created dial-frame.svg")
```

Run it with `python watch_dial.py` and open the SVG. You should see a light dial,
an outer ring, and a dark center circle. The API takes a **radius**: this dial's
diameter is 76 mm. The current compositor accepts radii from 26 through 44 mm.

## Add the background

Append this block:

```python
face.add_clous_de_paris(spacing=1.5, resolution=20)
face.generate()
face.to_svg("dial-background.svg")
print("Created dial-background.svg")
```

Run the script again and open `dial-background.svg`. The grid fills the dial,
and its visible strokes stop at the dial circle. Notice that the frame remains
in place.

## Place a second motif at three o'clock

Append:

```python
face.add_huiteight(
    num_curves=12,
    scale=7.0,
    hour=3,
    minute=0,
    distance=18.0,
    resolution=120,
)
face.generate()
face.to_svg("dial.svg")
print("Created dial.svg")
```

Run the script and open `dial.svg`. A small woven figure-eight motif appears to
the right of the dial center, with its center 18 mm from the origin. The
background remains visible through it; adding a texture does not cut a blank
region out of another texture.

The second `generate()` includes the new motif and regenerates the background.
Keep all three files to compare the frame, background, and completed composition.

You have composed a dial from two pattern families. For other placement methods,
see [how to position and compose patterns](../how-to/position-and-compose.md).
The [composition reference](../reference/composition.md) describes fixed family
draw order and the limits of hole decorations.
