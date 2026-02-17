"""
Paon (Peacock) Guilloché Pattern Example

Demonstrates the paon guilloché pattern — a fan of lines emanating from
6 o'clock, each zigzagging perpendicular to its travel direction.  Phase
offsets between neighbouring lines create peacock-feather arch bands.

Three implementations are provided:
1. WatchFace.add_paon() - High-level watch face API (mathematical)
2. PaonLayer - Standalone mathematical layer
3. RoseEngineLatheRun.paon() - Rose engine lathe simulation

The mathematical and rose engine implementations produce identical output.
"""

import os

from turtles import (
    PaonLayer,
    RoseEngineLatheRun,
    WatchFace,
)


def watchface_paon():
    """Create a watch face with a paon pattern using WatchFace.add_paon()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_paon()  # all defaults
    wf.generate()

    output_path = os.path.join("examples", "svg", "paon.svg")
    wf.to_svg(output_path)
    print(f"Watch face paon → {output_path}")


def watchface_paon_dense():
    """Create a watch face with a denser, more detailed paon pattern."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_paon()
    wf.generate()

    output_path = os.path.join("examples", "svg", "paon_dense.svg")
    wf.to_svg(output_path)
    print(f"Watch face paon (dense) → {output_path}")


def paon_direct():
    """
    Create paon pattern using PaonLayer (direct mathematical calculation).

    This is the standalone layer — same output as WatchFace.add_paon()
    but without the watch face frame.
    """
    layer = PaonLayer()  # uses defaults (100 lines, fan from 6 o'clock, n_harmonics=3)
    layer.generate()

    layer.to_svg("examples/svg/paon_direct.svg")
    print("Paon direct (PaonLayer) → examples/svg/paon_direct.svg")


def paon_rose_engine():
    """
    Create paon pattern using RoseEngineLatheRun.paon() (rose engine simulation).

    This produces identical output to the mathematical PaonLayer.
    The rose engine configures a linear pass mode with sinusoidal rosette
    modulation and phase variation matching the direct method.
    """
    run = RoseEngineLatheRun.paon()  # uses defaults (100 lines, fan from 6 o'clock, n_harmonics=3)
    run.generate()

    run.to_svg("examples/svg/paon_rose_engine.svg")
    print("Paon rose engine (RoseEngineLatheRun) → examples/svg/paon_rose_engine.svg")


def main():
    print("Generating Paon (Peacock) Guilloché Patterns\n")
    print("=" * 55)

    # Method 1: Watch face with mathematical paon
    print("\nMethod 1: WatchFace.add_paon()")
    print("-" * 35)
    watchface_paon()

    # Method 2: Watch face with dense paon
    print("\nMethod 2: WatchFace.add_paon(dense)")
    print("-" * 35)
    watchface_paon_dense()

    # Method 3: Standalone PaonLayer
    print("\nMethod 3: PaonLayer (standalone)")
    print("-" * 35)
    paon_direct()

    # Method 4: Rose engine simulation
    print("\nMethod 4: RoseEngineLatheRun.paon()")
    print("-" * 35)
    paon_rose_engine()

    print("\n" + "=" * 55)
    print("Done! Check examples/svg/ for output files.")
    print("\nNote: PaonLayer and RoseEngineLatheRun.paon() produce identical output.")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    main()
