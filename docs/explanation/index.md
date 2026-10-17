# Explanation

A turtles design can describe a curve, a collection of decorative layers, or a simplified rose-engine run. Those models can produce similar pictures while retaining different information about the paths, their placement, and the cutting tool. Understanding those differences matters when interpreting a preview or exchanging geometry with another program.

These pages explain the models behind the output. They discuss why a pattern looks and behaves as it does, rather than walking through construction or listing API arguments.

## Pattern models

[Pattern models](pattern-models.md) connects the named pattern families to their geometric construction. It explains direct layers versus rose-engine simulation, how phase creates folds and mesh effects, why sampling can change the apparent shape, and how Python and Rust relate to the same geometry.

## Coordinates and boundaries

[Coordinates](coordinates.md) explains how physical dimensions, SVG axes, clock positions, and layer centers fit together. It separates a layer's own boundary from the enclosing dial and explains why moving a pattern does not automatically resize or rotate it.

## Geometry and exports

[Geometry and exports](geometry-and-exports.md) explains what survives each export and what remains a drawing convention. It distinguishes SVG strokes, STEP polylines, and STL triangles from a manufactured solid, and describes the gap between a decorative preview and machining planning.

```{toctree}
---
hidden:
---
pattern-models
coordinates
geometry-and-exports
```
