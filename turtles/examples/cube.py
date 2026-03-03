"""
Cube (Tumbling Blocks) Guilloché Pattern Example

Demonstrates the cube guilloché pattern — an optical illusion of three-dimensional
cubes tiled across a surface.  The pattern is created from three sets of parallel
lines offset by 60°.  When superimposed, the three directions create the
classic tumbling-blocks / Escher-cube tessellation (rhombille tiling).

Three implementations are provided:
1. WatchFace.add_cube() - High-level watch face API (mathematical)
2. CubeLayer - Standalone mathematical layer
3. RoseEngineLatheRun.cube() - Rose engine lathe simulation

The cube pattern uses three sets of parallel straight lines at 60° offsets,
clipped to a circle.  The mathematical and rose engine implementations
produce identical output.
"""

import math
import os

from turtles import (
    CubeLayer,
    RoseEngineLatheRun,
    WatchFace,
)


def watchface_cube():
    """Create a watch face with a cube pattern using WatchFace.add_cube()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_cube()  # all defaults
    wf.generate()

    output_path = os.path.join("examples", "svg", "cube.svg")
    wf.to_svg(output_path)
    print(f"Watch face cube → {output_path}")


def watchface_cube_fine():
    """Create a watch face with a finer, denser cube pattern."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_cube(spacing=0.27)
    wf.generate()

    output_path = os.path.join("examples", "svg", "cube_fine.svg")
    wf.to_svg(output_path)
    print(f"Watch face cube (fine) → {output_path}")


def watchface_cube_rotated():
    """Create a watch face with a 30°-rotated cube pattern."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_cube(angle=math.pi / 6.0)  # 30° rotation
    wf.generate()

    output_path = os.path.join("examples", "svg", "cube_rotated.svg")
    wf.to_svg(output_path)
    print(f"Watch face cube (rotated 30°) → {output_path}")


def watchface_cube_large():
    """Create a watch face with large, bold cubes."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_cube(spacing=1.0)
    wf.generate()

    output_path = os.path.join("examples", "svg", "cube_large.svg")
    wf.to_svg(output_path)
    print(f"Watch face cube (large) → {output_path}")


def cube_direct():
    """
    Create cube pattern using CubeLayer (direct mathematical calculation).

    This is the standalone layer — same output as WatchFace.add_cube()
    but without the watch face frame.
    """
    layer = CubeLayer()  # uses defaults (spacing=0.5, radius=22.0, angle=0)
    layer.generate()

    layer.to_svg("examples/svg/cube_direct.svg")
    print("Cube direct (CubeLayer) → examples/svg/cube_direct.svg")


def cube_rose_engine():
    """
    Create cube pattern using RoseEngineLatheRun.cube()
    (rose engine simulation).

    This produces identical output to the mathematical CubeLayer.
    The rose engine configures a straight-line reciprocating mode with three
    sets of straight-line groove cuts at 60° offsets.
    """
    run = RoseEngineLatheRun.cube()  # uses defaults
    run.generate()

    run.to_svg("examples/svg/cube_rose_engine.svg")
    print("Cube rose engine (RoseEngineLatheRun) → examples/svg/cube_rose_engine.svg")


def cube_variations():
    """
    Show various cube configurations side-by-side.

    Generates patterns with different spacings and angles.
    """
    configs = [
        {"spacing": 0.27, "angle": 0.0, "name": "fine"},
        {"spacing": 0.25, "angle": 0.0, "name": "medium"},
        {"spacing": 0.25, "angle": 0.0, "name": "coarse"},
        {"spacing": 0.25, "angle": math.pi / 6.0, "name": "rotated_30deg"},
    ]

    for cfg in configs:
        layer = CubeLayer(
            spacing=cfg["spacing"],
            radius=22.0,
            angle=cfg["angle"],
        )
        layer.generate()

        output_path = f"examples/svg/cube_{cfg['name']}.svg"
        layer.to_svg(output_path)
        print(f"Cube ({cfg['name']}) → {output_path}")


def main():
    print("Generating Cube (Tumbling Blocks) Guilloché Patterns\n")
    print("=" * 60)

    # Method 1: Watch face with mathematical cube
    print("\nMethod 1: WatchFace.add_cube()")
    print("-" * 40)
    watchface_cube()

    # Method 2: Watch face with fine cube
    print("\nMethod 2: WatchFace.add_cube(fine)")
    print("-" * 40)
    watchface_cube_fine()

    # Method 3: Watch face with rotated grid
    print("\nMethod 3: WatchFace.add_cube(rotated)")
    print("-" * 40)
    watchface_cube_rotated()

    # Method 4: Watch face with large cubes
    print("\nMethod 4: WatchFace.add_cube(large)")
    print("-" * 40)
    watchface_cube_large()

    # Method 5: Standalone CubeLayer
    print("\nMethod 5: CubeLayer (standalone)")
    print("-" * 40)
    cube_direct()

    # Method 6: Rose engine simulation
    print("\nMethod 6: RoseEngineLatheRun.cube()")
    print("-" * 40)
    cube_rose_engine()

    # Method 7: Variations
    print("\nMethod 7: CubeLayer variations")
    print("-" * 40)
    cube_variations()

    print("\n" + "=" * 60)
    print("All cube patterns generated!")


if __name__ == "__main__":
    main()
