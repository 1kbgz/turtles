#!/usr/bin/env python3
"""
Diamant (Diamond) Guilloché Pattern Example

This example creates a diamant guilloché pattern. The diamant pattern is formed
by creating equally-sized circles that are tangent to the center, rotated around
the center. The overlapping circles create diamond/mesh intersections.

Two implementations are provided:
1. DiamantLayer - Direct geometric calculation (recommended)
2. RoseEngineLatheRun - Using the rose engine simulation
"""

from turtles import (
    CuttingBit,
    DiamantLayer,
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


def diamant_rose_engine():
    """
    Create diamant pattern using RoseEngineLatheRun (rose engine simulation).

    This approach simulates a rose engine lathe with sinusoidal rosette pattern.
    With frequency=1, the sinusoidal pattern creates a circle whose center is
    offset from the origin. Rotating the phase rotates this offset, creating
    circles tangent to the center at different angles.
    """
    # Use sinusoidal rosette with frequency=1
    # This creates: radius = base_radius + amplitude * sin(angle)
    # Which is equivalent to a circle of radius=amplitude centered at
    # distance=base_radius from origin, but traced in polar coordinates.
    # When amplitude = base_radius, the circle passes through the origin
    # (is tangent to the center)
    config = RoseEngineConfig(base_radius=20.0, amplitude=20.0)
    config.set_rosette(RosettePattern.sinusoidal(frequency=1.0))
    config.set_resolution(360)

    # Create a cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.02)

    # Create multi-pass run - each pass will be rotated around the center
    # by rotating the phase of the sinusoidal pattern
    run = RoseEngineLatheRun(config, bit, num_passes=72)
    run.generate()

    # Export to SVG
    run.to_svg("examples/svg/diamant_rose_engine.svg")
    print("Generated examples/svg/diamant_rose_engine.svg")
    print("  Method: RoseEngineLatheRun (rose engine simulation)")
    print("  Passes: 72")


def main():
    print("Generating Diamant (Diamond) Guilloché Patterns\n")
    print("=" * 50)

    # Method 1: Direct geometric calculation (recommended)
    print("\nMethod 1: DiamantLayer")
    print("-" * 30)
    diamant_direct()

    # Method 2: Rose engine simulation
    print("\nMethod 2: RoseEngineLatheRun")
    print("-" * 30)
    diamant_rose_engine()

    print("\n" + "=" * 50)
    print("Done! Check examples/svg/ for output files.")


if __name__ == "__main__":
    main()
