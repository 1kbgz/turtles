# How-to guides

Use these guides to install Turtles, place patterns on a dial, export generated
geometry, or work on the library. Each guide addresses a task and includes the
checks needed before moving on.

- [Install Turtles](install.md): install the Python distribution, build from
  source, or use the Rust library from a local checkout.
- [Position and compose patterns](position-and-compose.md): place standalone
  layers by coordinates, polar angle, or clock position, then combine them on a
  watch face.
- [Export patterns](export.md): write SVG previews, exchange STEP curves, inspect
  STL approximations, and retrieve sampled lines.
- [Develop and check changes](develop.md): set up an editable installation, run
  Python and Rust tests, build documentation, and build a wheel.

The Python examples in the positioning and export guides are independent: each
block imports its dependencies, constructs its objects, and generates geometry
before writing files. Run them in a writable working directory. Output paths are
relative to that directory, and rerunning an example replaces its output files.

If you want a guided first exercise, start with the
[first-pattern tutorial](../tutorials/first-pattern.md). For signatures and
parameter constraints, use the [pattern](../reference/patterns.md),
[composition](../reference/composition.md), and
[rose-engine](../reference/rose-engine.md) references. For coordinate conventions
and export tradeoffs, read [coordinates](../explanation/coordinates.md) and
[geometry and exports](../explanation/geometry-and-exports.md).

```{toctree}
---
hidden:
---
install
position-and-compose
export
develop
```
