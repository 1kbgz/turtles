# Coordinates and boundaries

Why can a small, correctly placed layer still extend outside a dial, and why does a clock position differ from a polar angle? Turtles separates the dimensions of a motif, the position of its center, and the boundary of the surrounding composition. All three use the same planar coordinates, but they describe different relationships.

## Physical dimensions and drawing space

The core geometry uses millimeters for dimensional quantities such as radii, center offsets, spacing, amplitudes, and export depth. The SVG exporters pair numeric view-box dimensions with physical width and height in millimeters, so one coordinate unit corresponds to one millimeter in the SVG's intended physical size. A browser's zoom or an embedded image's display width can change its apparent size without changing those dimensions.

`WatchFace` and `GuillochePattern` currently accept a **radius of 26–44 mm**, inclusive. These values are actual radii, not diameters: a radius of 38 mm describes a dial circle 76 mm across. This is the implemented validation rule, regardless of diameter conventions in watch descriptions. It is not a universal restriction on pattern layers. A standalone spirograph or flinqué layer can have a smaller positive radius for a smaller motif.

A radius also has to be interpreted within its own model. Diamant's `circle_radius` is the radius of each constituent circle, whose far edge reaches twice that distance from the layer center. Draperie's `base_radius` is the middle of a band of rings. A limaçon's base radius receives an additional radial displacement. Those quantities do not promise that all generated points fit inside a circle of the same radius.

Ratios remain dimensionless. For a spirograph, `radius_ratio` is rolling radius divided by outer radius, and `point_distance` is pen offset divided by rolling radius. Multiplying either by a dimension in millimeters produces a physical length; treating either as a length changes the intended model.

## SVG axes and positive angles

The dial center is the origin. Positive x points right, and positive y points down in SVG screen coordinates. Negative coordinates are normal: the upper half of a centered dial has negative y values. The view box determines which rectangle of that coordinate plane appears on the page; it does not move the origin to the upper-left corner of the dial.

Polar placement uses the ordinary formulas

$$
x = d\cos\theta, \qquad y = d\sin\theta.
$$

Thus zero radians points right, $\pi/2$ points down, $\pi$ points left, and $-\pi/2$ points up. Increasing a polar angle appears clockwise on screen because y increases downward. Importing the coordinates into a program that displays positive y upward can mirror that apparent orientation unless the receiving coordinate convention is accounted for.

Placement angles and most geometric phase or grid-rotation values are in radians. There are specific exceptions: a V-bit's included angle and cube's `leg_angle` are in degrees. Paon's `fan_angle` is a phase-envelope scale, despite its name. There is no universal angle unit that can safely be inferred from the word “angle.”

## Clock positions follow the hour hand

Clock placement describes the direction of the **hour hand**, including its movement between hours. The helper converts an hour $h$ and minute $m$ to

$$
\theta = -\frac{\pi}{2} + 2\pi\frac{60(h\bmod 12)+m}{720}.
$$

The full turn is 12 hours, or 720 minutes. At distance $d$, 12:00 is $(0,-d)$, 3:00 is $(d,0)$, 6:00 is $(0,d)$, and 9:00 is $(-d,0)$. At 3:30 the center lies halfway between the 3 and 4 hour positions, 15 degrees clockwise from 3:00. It does not point to 6 as the minute hand would. Similarly, 12:30 is only 15 degrees clockwise from the top.

The clock direction and distance determine the layer's center. A distance of zero leaves that center at the origin regardless of the hour. The layer's own radius determines its size around that center; it is not another way to specify the distance from the dial center.

## Translation does not reorient a motif

A layer generally computes its motif around a local center and adds `center_x` and `center_y` to obtain drawing coordinates. Polar and clock constructors calculate those offsets; they do not automatically rotate the motif to face outward or scale it to fit the remaining space.

For example, moving a clous de Paris layer toward 3:00 leaves the grid's configured rotation unchanged. Moving a paon layer leaves its fan orientation unchanged. The resulting composition can therefore place the same visual motif at several locations without turning each copy into a radial arrangement.

The composition retains the independently generated paths. An offset motif can overlap a background, another motif, or the dial edge. The dial radius supplies a global boundary, but it does not resize the layer or create a separate circular mask around every displaced center.

(local-and-global-boundaries-solve-different-problems)=

## Local and global boundaries solve different problems

There are two boundary levels.

**A local boundary belongs to a particular layer.** Clous de Paris computes line endpoints on its own circle. Cube clips its zigzag segments to a local circle, then rotates and translates them. Paon retains samples inside its local circle, so its endpoints approximate the boundary at the selected sampling density rather than using the same analytic trimming as clous de Paris. Moving one of these layers moves its local region too.

Not every family has such a local clipping step. Flinqué uses its radius to arrange the ring band but adds waves beyond the base radii. Limaçon can exceed its base radius, and draperie generates its configured ring band without clipping it to an enclosing dial. A size parameter is not automatically a clipping radius.

**The global boundary belongs to the combined dial.** The watch-face and combined guilloché SVG exporters apply a circular clip path centered at the origin to the pattern group. A locally bounded layer can still cross this global boundary when offset. Conversely, a layer without a local clip can be visibly trimmed by the global dial circle.

Combined STEP and STL use geometric clipping to that global circle, splitting paths at exits and reentries instead of relying on the SVG mask. This operates on the sampled line segments, not on their original analytic curves. It also bounds centerlines rather than a swept cutter's full width. The distinction between a visible mask, a trimmed path, and a physical cut is developed in [geometry and exports](geometry-and-exports.md).

The [composition reference](../reference/composition.md) covers the placement interfaces. [Pattern models](pattern-models.md) explains why individual families have different extents even when their size parameters have similar names.
