# Geometry and exports

Why does a convincing dial preview not imply a machinable solid? Turtles primarily generates ordered paths. Exporters translate those paths into drawing commands, CAD curves, or triangles, but each translation retains only part of the design model. The format's ability to describe a solid does not mean the exporter constructs one.

## The common object is a path

Direct layers store sampled coordinates in ordered lists. The combined composition gathers every supported family into a collection of 2D paths. The exporters work from those samples, so smooth mathematical curves generally become straight segments between consecutive points. They are not automatically fitted to exact circles, splines, or CNC arcs.

This representation preserves the trace of a motif without defining the material around it. Intersections between paths do not become joined CAD topology, and overlapping strokes do not cause a Boolean cut. Likewise, a closed loop is not automatically a filled region or a boundary of stock.

Spherical spirographs expose an important difference between standalone and combined output. The standalone object retains both its planar trace and dome-projected 3D points. Its standalone STEP and STL exporters use the 3D points. When added to `GuillochePattern`, it contributes its 2D trace; the combined STEP places that trace at z = 0, and the combined STL extrudes it down from z = 0. A combined export therefore does not preserve the dome.

## SVG is a styled view of the paths

SVG paths use move and line commands with a stroke width, color, and other drawing attributes. The stroke width gives a line visible thickness without constructing the sidewalls or cross-section of an engraved groove. A dark line can suggest depth, but it does not establish how much material was removed.

The watch-face drawing adds circles for the dial, optional bezel, and configured holes. The combined guilloché SVG also adds its own dial, bezel, and central hole decorations. These are SVG elements. `WatchFace` delegates STEP and STL to its underlying guilloché composition, so those decorations are not exported as a plate, bezel solid, or drilled holes. Drawing a filled circle over a pattern also does not delete the paths beneath it.

Canvas size is another drawing choice. Many standalone exporters fit the generated bounds with a margin, while watch-face and combined guilloché SVG use a larger dial-centered canvas. Different amounts of whitespace can make identical dimensions appear different when previews are scaled to the same display width. The [coordinate model](coordinates.md) explains the relationship between millimeters and the SVG view box.

Path closure is separate from stroke styling. A spirograph SVG deliberately leaves the sampled trace open, because forcing its last point back to its first could add a long chord across an incomplete pattern. Other families can return to their start by sampling a complete period. A nearly closed appearance is not evidence that the stored endpoints coincide.

## A clipping mask is not trimmed geometry

The [two boundary levels](coordinates.md#local-and-global-boundaries-solve-different-problems) distinguish a layer's own region from the global dial circle. At the global level, combined SVG hides pattern content outside a circular clip path. The source path coordinates can still extend beyond the circle; the viewer simply does not show that ink.

Combined STEP and STL instead use geometrically clipped paths. The clipper intersects each sampled segment with the dial circle, inserts boundary points, and splits a path when it leaves and later reenters. It can also retain the portion of a crossing segment whose two endpoints are both outside. Keeping the pieces separate avoids adding a chord over the discarded interval.

This trimming limits the path coordinates exported by those combined exporters. It does not remove the hidden coordinates from the original layer, enforce every family's local boundary, or account for cutter width. A centerline ending on the dial edge can still put part of a finite-width tool beyond that edge. Standalone spirograph and single-lathe exports do not acquire the composition's global dial clipping merely by using the same file format.

## STEP contains polyline curves

The implemented STEP writer emits an ISO 10303-21 file using the AP214 schema. It creates referenced Cartesian points, a `POLYLINE` for each path with at least two points, and a `GEOMETRIC_CURVE_SET` connected to the product's shape representation. Its length unit is millimeters.

This is CAD curve geometry. It contains no swept grooves, faces, shell, or boundary-representation solid. The writer retains the supplied vertices rather than converting an apparent circle into an analytic STEP circle. The representation's numerical uncertainty declaration does not establish the approximation error between a sampled path and its original formula.

For combined guilloché and watch-face output, all curves lie at z = 0 after global clipping. Standalone planar spirographs likewise become curves at z = 0; standalone spherical spirographs retain their generated z values. The working STEP adapters ignore the export configuration, so changing depth, base thickness, or tool radius does not give these curves a physical section or change their height.

STEP support is not uniform across the library. The single `RoseEngineLathe` STEP method currently returns an unimplemented-export error. `RoseEngineLatheRun` supplies SVG output and line inspection, not its own STEP or STL exporter. Individual direct decorative layers have standalone SVG output; their paths reach combined STEP or STL through composition. A STEP extension alone is therefore insufficient to describe either the contents or the available export route.

## STL contains simple vertical ribbons

The planar spirograph, single-lathe, and combined STL implementations make two triangles for each path segment. For a segment from $p_1$ to $p_2$, they use the two endpoints at z = 0 and the same x/y coordinates at z = −depth. This produces a vertical rectangular strip with **zero width across the path**. The spherical spirograph uses each point's own z value and a second copy lowered by the configured depth.

These strips have no finite groove width, top or bottom caps, surrounding dial plate, or union at intersections. Together they do not define a closed, watertight manufactured solid. The exporters write a fixed normal of `(0, 0, 1)` rather than computing the actual normal of each vertical triangle. STL vertices are also converted to single-precision coordinates during writing.

The current STL code uses `depth` but does not use `base_thickness` or `tool_radius` to build stock or compensate a cutter. In the single-lathe exporter, the triangles follow the centerline, not the generated left and right cut edges. A detailed cutting-bit configuration therefore does not turn this STL into a swept V-groove or ball-cut surface.

Closure also differs by exporter. Standalone spirograph STL connects the final point back to the first using modulo indexing, including when the trace is incomplete. Its SVG and STEP do not add that connection. Combined STL treats each clipped piece as an open run; single-lathe STL also follows consecutive samples without adding an extra closing segment. An unexpected connecting wall can consequently be specific to the standalone STL rather than present in the underlying preview.

STL itself has no embedded length-unit declaration. Turtles writes coordinate values intended as millimeters, but a receiving program's interpretation remains separate from the STEP unit declaration and SVG physical dimensions.

## Engine previews and machining plans answer different questions

The rose-engine model adds information useful for understanding a cut: a centerline, width-offset edges, bit descriptions, and optional depth modulation. The rendered single-lathe output includes the centerline and both approximate edges. Depth and shading arrays are computed when modulation is enabled, but its SVG writer does not use those arrays to vary the stroke, and its STL writer uses the constant export depth. The tool-path arc collection is cleared during generation and is not populated with fitted machining arcs.

The general multi-pass run's SVG displays segmented centerlines rather than all the single-lathe edge traces. Its drawing gaps come from retaining roughly 70 percent of each sampled cycle when segmentation is enabled. Dedicated family branches bypass that general segmentation path. Such preview differences can change the visible density without representing different stock-removal operations.

A machining plan must additionally resolve stock shape, cutter engagement, finite-width clearances, overlapping cuts, workholding, entry and exit moves, safe heights, feed and spindle choices, and the target machine's coordinates and commands. The current exporters do not perform those operations or generate a machine program. Geometric clipping and successful file serialization do not certify collision avoidance or manufacturability.

The useful interpretation is therefore specific: SVG shows styled paths and decorations; the working STEP exporters exchange sampled curves; STL records the implemented triangle strips. Those outputs can inform later CAD or CAM work, but manufacturing decisions require a model beyond the decorative preview.

The [export reference](../reference/exports.md) describes the available interfaces and options. [Pattern models](pattern-models.md) explains the source of the paths, and the [rose-engine reference](../reference/rose-engine.md) distinguishes simulation outputs from direct layer geometry.
