#!/usr/bin/env python3
"""
Diamant (Diamond) Guilloché Pattern Example

This example creates a diamant guilloché pattern. The diamant pattern is formed
by creating equally-sized circles that are tangent to the center, rotated around
the center. The overlapping circles create diamond/mesh intersections.

Four implementations are provided:
1. DiamantLayer - Direct geometric calculation using circles (recommended)
2. LimaconLayer - Direct geometric calculation using limaçon curves
3. RoseEngineLatheRun.diamant() - Rose engine simulation (circles, matches DiamantLayer 1-1)
4. RoseEngineLatheRun.limacon() - Rose engine simulation (limaçon, matches LimaconLayer 1-1)
"""

import os

from turtles import (
    DiamantLayer,
    LimaconLayer,
    RoseEngineLatheRun,
)


def diamant_direct():
    """
    Create diamant pattern using DiamantLayer (direct geometric calculation).

    This is the recommended approach - it directly calculates circles that are
    tangent to the center and rotates them around to create the pattern.
    """
    layer = DiamantLayer(num_circles=72, circle_radius=20.0, resolution=360)
    layer.generate()

    layer.to_svg("examples/svg/diamant_direct.svg")
    print("Diamant direct (DiamantLayer) → examples/svg/diamant_direct.svg")


def limacon_direct():
    """
    Create limaçon pattern using LimaconLayer (direct geometric calculation).

    This produces identical output to the rose engine with sinusoidal frequency=1.
    The limaçon equation is: r = base_radius + amplitude * sin(θ + phase)

    When amplitude = base_radius, the curves pass through the origin,
    creating shapes that are tangent to the center.
    """
    layer = LimaconLayer(num_curves=72, base_radius=20.0, amplitude=20.0, resolution=360)
    layer.generate()

    layer.to_svg("examples/svg/limacon_direct.svg")
    print("Limaçon direct (LimaconLayer) → examples/svg/limacon_direct.svg")


def diamant_rose_engine():
    """
    Create diamant pattern using RoseEngineLatheRun.diamant() (rose engine simulation).

    This produces identical output to the mathematical DiamantLayer.
    The rose engine models a round eccentric cam whose eccentricity equals the
    circle radius.  Each pass traces a circle tangent to the centre.
    """
    run = RoseEngineLatheRun.diamant(num_circles=72, circle_radius=20.0, resolution=360)
    run.generate()

    run.to_svg("examples/svg/diamant_rose_engine.svg")
    print("Diamant rose engine (RoseEngineLatheRun) → examples/svg/diamant_rose_engine.svg")


def limacon_rose_engine():
    """
    Create limaçon pattern using RoseEngineLatheRun.limacon() (rose engine simulation).

    This produces identical output to the mathematical LimaconLayer.
    The rose engine uses a sinusoidal cam (frequency 1) whose polar equation
    r = base_radius + amplitude · sin(θ + phase) naturally traces limaçon curves.
    """
    run = RoseEngineLatheRun.limacon(num_curves=72, base_radius=20.0, amplitude=20.0, resolution=360)
    run.generate()

    run.to_svg("examples/svg/limacon_rose_engine.svg")
    print("Limaçon rose engine (RoseEngineLatheRun) → examples/svg/limacon_rose_engine.svg")


def main():
    print("Generating Diamant (Diamond) Guilloché Patterns\n")
    print("=" * 55)

    # Method 1: Direct geometric calculation with circles (recommended)
    print("\nMethod 1: DiamantLayer (circles)")
    print("-" * 35)
    diamant_direct()

    # Method 2: Direct geometric calculation with limaçon curves
    print("\nMethod 2: LimaconLayer (limaçon curves)")
    print("-" * 35)
    limacon_direct()

    # Method 3: Rose engine simulation (circles)
    print("\nMethod 3: RoseEngineLatheRun.diamant()")
    print("-" * 35)
    diamant_rose_engine()

    # Method 4: Rose engine simulation (limaçon)
    print("\nMethod 4: RoseEngineLatheRun.limacon()")
    print("-" * 35)
    limacon_rose_engine()

    print("\n" + "=" * 55)
    print("Done! Check examples/svg/ for output files.")
    print("\nNote: DiamantLayer and RoseEngineLatheRun.diamant() produce identical output (circles).")
    print("Note: LimaconLayer and RoseEngineLatheRun.limacon() produce identical output (limaçon).")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    main()
