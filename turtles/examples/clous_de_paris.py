"""
Clous de Paris (Hobnail) Guilloché Pattern Example

Demonstrates the clous de Paris guilloché pattern — a grid of small pyramidal
"hobnails" created by two orthogonal sets of parallel V-groove cuts.  The
pattern is typically rotated 45° for the classic diagonal appearance found
on luxury watch dials.

Three implementations are provided:
1. WatchFace.add_clous_de_paris() - High-level watch face API (mathematical)
2. ClousDeParisLayer - Standalone mathematical layer
3. RoseEngineLatheRun.clous_de_paris() - Rose engine lathe simulation

The mathematical and rose engine implementations produce identical output.
"""

import math
import os

from turtles import (
    ClousDeParisLayer,
    RoseEngineLatheRun,
    WatchFace,
)


def watchface_clous_de_paris():
    """Create a watch face with a clous de Paris pattern using WatchFace.add_clous_de_paris()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_clous_de_paris()  # all defaults
    wf.generate()

    output_path = os.path.join("examples", "svg", "clous_de_paris.svg")
    wf.to_svg(output_path)
    print(f"Watch face clous de Paris → {output_path}")


def watchface_clous_de_paris_fine():
    """Create a watch face with a finer, denser clous de Paris pattern."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_clous_de_paris(spacing=0.5)
    wf.generate()

    output_path = os.path.join("examples", "svg", "clous_de_paris_fine.svg")
    wf.to_svg(output_path)
    print(f"Watch face clous de Paris (fine) → {output_path}")


def watchface_clous_de_paris_square():
    """Create a watch face with a 0° grid (horizontal/vertical) clous de Paris."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_clous_de_paris(angle=0.0)  # straight grid
    wf.generate()

    output_path = os.path.join("examples", "svg", "clous_de_paris_square.svg")
    wf.to_svg(output_path)
    print(f"Watch face clous de Paris (square) → {output_path}")


def clous_de_paris_direct():
    """
    Create clous de Paris pattern using ClousDeParisLayer (direct mathematical calculation).

    This is the standalone layer — same output as WatchFace.add_clous_de_paris()
    but without the watch face frame.
    """
    layer = ClousDeParisLayer()  # uses defaults (spacing=1.0, radius=22.0, angle=π/4)
    layer.generate()

    layer.to_svg("examples/svg/clous_de_paris_direct.svg")
    print("Clous de Paris direct (ClousDeParisLayer) → examples/svg/clous_de_paris_direct.svg")


def clous_de_paris_rose_engine():
    """
    Create clous de Paris pattern using RoseEngineLatheRun.clous_de_paris()
    (rose engine simulation).

    This produces identical output to the mathematical ClousDeParisLayer.
    The rose engine configures a straight-line reciprocating mode with two
    orthogonal groove sets.
    """
    run = RoseEngineLatheRun.clous_de_paris()  # uses defaults
    run.generate()

    run.to_svg("examples/svg/clous_de_paris_rose_engine.svg")
    print("Clous de Paris rose engine (RoseEngineLatheRun) → examples/svg/clous_de_paris_rose_engine.svg")


def clous_de_paris_variations():
    """
    Show various clous de Paris configurations side-by-side.

    Generates patterns with different spacings and angles for comparison.
    """
    configs = [
        {"spacing": 0.5, "angle": math.pi / 4.0, "name": "fine_diagonal"},
        {"spacing": 1.0, "angle": math.pi / 4.0, "name": "medium_diagonal"},
        {"spacing": 2.0, "angle": math.pi / 4.0, "name": "coarse_diagonal"},
        {"spacing": 1.0, "angle": 0.0, "name": "medium_square"},
        {"spacing": 1.0, "angle": math.pi / 6.0, "name": "medium_30deg"},
    ]

    for cfg in configs:
        layer = ClousDeParisLayer(
            spacing=cfg["spacing"],
            radius=22.0,
            angle=cfg["angle"],
        )
        layer.generate()

        output_path = f"examples/svg/clous_de_paris_{cfg['name']}.svg"
        layer.to_svg(output_path)
        print(f"Clous de Paris ({cfg['name']}) → {output_path}")


def main():
    print("Generating Clous de Paris (Hobnail) Guilloché Patterns\n")
    print("=" * 60)

    # Method 1: Watch face with mathematical clous de Paris
    print("\nMethod 1: WatchFace.add_clous_de_paris()")
    print("-" * 40)
    watchface_clous_de_paris()

    # Method 2: Watch face with fine clous de Paris
    print("\nMethod 2: WatchFace.add_clous_de_paris(fine)")
    print("-" * 40)
    watchface_clous_de_paris_fine()

    # Method 3: Watch face with square grid
    print("\nMethod 3: WatchFace.add_clous_de_paris(square)")
    print("-" * 40)
    watchface_clous_de_paris_square()

    # Method 4: Standalone ClousDeParisLayer
    print("\nMethod 4: ClousDeParisLayer (standalone)")
    print("-" * 40)
    clous_de_paris_direct()

    # Method 5: Rose engine simulation
    print("\nMethod 5: RoseEngineLatheRun.clous_de_paris()")
    print("-" * 40)
    clous_de_paris_rose_engine()

    # Method 6: Variations
    print("\nMethod 6: ClousDeParisLayer variations")
    print("-" * 40)
    clous_de_paris_variations()

    print("\n" + "=" * 60)
    print("Done! Check examples/svg/ for output files.")
    print("\nNote: ClousDeParisLayer and RoseEngineLatheRun.clous_de_paris() produce identical output.")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    main()
