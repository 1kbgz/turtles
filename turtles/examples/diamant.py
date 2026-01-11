#!/usr/bin/env python3
"""
Diamant (Diamond) Guilloché Pattern Example

This example creates a diamant guilloché pattern. The diamant pattern is formed
by creating equally-sized circles that are tangent to the center, rotated around
the center. The overlapping circles create diamond/mesh intersections.

Three implementations are provided:
1. DiamantLayer - Direct geometric calculation using circles (recommended)
2. LimaconLayer - Direct geometric calculation using limaçon curves
3. RoseEngineLatheRun - Using the rose engine simulation
"""

from turtles import (
    CuttingBit,
    DiamantLayer,
    LimaconLayer,
    RoseEngineConfig,
    RoseEngineLatheRun,
    RosettePattern,
)


def diamant_direct():
    """
    Create diamant pattern using DiamantLayer (direct geometric calculation).

    This is the recommended approach - it directly calculates circles that are
    tangent to the center and rotates them around to create the pattern.
    """
    # Create a diamant layer with 72 circles, each with radius 20mm
    # More circles = denser mesh pattern
    layer = DiamantLayer(num_circles=72, circle_radius=20.0, resolution=360)
    layer.generate()

    # Export to SVG
    layer.to_svg("examples/svg/diamant_direct.svg")
    print("Generated examples/svg/diamant_direct.svg")
    print("  Method: DiamantLayer (direct geometric calculation)")
    print("  Circles: 72, each with radius 20.0mm")


def limacon_direct():
    """
    Create limaçon pattern using LimaconLayer (direct geometric calculation).

    This produces identical output to the rose engine with sinusoidal frequency=1.
    The limaçon equation is: r = base_radius + amplitude * sin(θ + phase)

    When amplitude = base_radius, the curves pass through the origin,
    creating shapes that are tangent to the center.
    """
    # Create a limaçon layer with 72 curves, matching rose engine parameters
    layer = LimaconLayer(num_curves=72, base_radius=20.0, amplitude=20.0, resolution=360)
    layer.generate()

    # Export to SVG
    layer.to_svg("examples/svg/limacon_direct.svg")
    print("Generated examples/svg/limacon_direct.svg")
    print("  Method: LimaconLayer (direct geometric calculation)")
    print("  Curves: 72, base_radius=20.0mm, amplitude=20.0mm")


def diamant_rose_engine():
    """
    Create a diamant-style pattern using RoseEngineLatheRun (rose engine simulation).

    NOTE: The rose engine produces limaçon (snail) shapes in polar coordinates,
    which are different from the true circles produced by DiamantLayer.
    Both create beautiful overlapping patterns, but with different geometry.

    The sinusoidal rosette with frequency=1 creates:
        radius = base_radius + amplitude * sin(angle)

    This traces a limaçon that passes through the origin when amplitude = base_radius,
    creating shapes that are tangent to the center like the direct method.
    """
    # Use sinusoidal rosette with frequency=1
    # When amplitude = base_radius, the shapes pass through the origin
    # matching the tangent-to-center behavior of DiamantLayer
    config = RoseEngineConfig(base_radius=20.0, amplitude=20.0)
    config.set_rosette(RosettePattern.sinusoidal(frequency=1.0))
    config.set_resolution(360)

    # Create a cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.02)

    # Create multi-pass run - each pass will be rotated around the center
    # by rotating the phase of the sinusoidal pattern
    # Use segments_per_pass=1 to draw complete shapes without gaps
    run = RoseEngineLatheRun(config, bit, num_passes=72, segments_per_pass=1)
    run.generate()

    # Export to SVG
    run.to_svg("examples/svg/diamant_rose_engine.svg")
    print("Generated examples/svg/diamant_rose_engine.svg")
    print("  Method: RoseEngineLatheRun (rose engine simulation)")
    print("  Passes: 72")
    print("  Note: Creates limaçon shapes (different from true circles)")


def main():
    print("Generating Diamant (Diamond) Guilloché Patterns\n")
    print("=" * 50)

    # Method 1: Direct geometric calculation with circles (recommended)
    print("\nMethod 1: DiamantLayer (circles)")
    print("-" * 30)
    diamant_direct()

    # Method 2: Direct geometric calculation with limaçon curves
    print("\nMethod 2: LimaconLayer (limaçon curves)")
    print("-" * 30)
    limacon_direct()

    # Method 3: Rose engine simulation
    print("\nMethod 3: RoseEngineLatheRun")
    print("-" * 30)
    diamant_rose_engine()

    print("\n" + "=" * 50)
    print("Done! Check examples/svg/ for output files.")
    print("\nNote: LimaconLayer and RoseEngineLatheRun produce identical output.")
    print("DiamantLayer uses true circles, which is subtly different.")


if __name__ == "__main__":
    main()
