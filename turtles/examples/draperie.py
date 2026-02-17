"""
Draperie (Drapery) Guilloché Pattern Example

Demonstrates the draperie guilloché pattern — flowing fabric-like concentric
wavy rings whose phase oscillates sinusoidally.

Three implementations are provided:
1. WatchFace.add_draperie() - High-level watch face API (mathematical)
2. DraperieLayer - Standalone mathematical layer
3. RoseEngineLatheRun.draperie() - Rose engine lathe simulation

The mathematical and rose engine implementations produce identical output.
"""

import os

from turtles import (
    DraperieLayer,
    RoseEngineLatheRun,
    WatchFace,
)


def watchface_draperie():
    """Create a watch face with a smooth draperie pattern using WatchFace.add_draperie()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie()  # all defaults: circular_phase=2.0, rounded dome folds
    wf.generate()

    output_path = os.path.join("examples", "svg", "draperie.svg")
    wf.to_svg(output_path)
    print(f"Watch face draperie (smooth) → {output_path}")


def watchface_draperie_sharp():
    """Create a watch face with a sharp-angled draperie pattern."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie_sharp()  # circular_phase=0, phase_exponent=1, sharp V-shaped folds
    wf.generate()

    output_path = os.path.join("examples", "svg", "draperie_sharp.svg")
    wf.to_svg(output_path)
    print(f"Watch face draperie (sharp)  → {output_path}")


def watchface_draperie_soft_wave():
    """Create a watch face with soft wave crests (wave_exponent=3)."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie(wave_exponent=3)  # softer, rounded wave peaks
    wf.generate()

    output_path = os.path.join("examples", "svg", "draperie_soft_wave.svg")
    wf.to_svg(output_path)
    print(f"Watch face draperie (soft wave) → {output_path}")


def draperie_direct():
    """
    Create draperie pattern using DraperieLayer (direct mathematical calculation).

    This is the standalone layer — same output as WatchFace.add_draperie()
    but without the watch face frame.
    """
    layer = DraperieLayer(
        num_rings=96,
        base_radius=22.0,
        radius_step=0.44,
        wave_frequency=12.0,
        phase_oscillations=2.5,
        resolution=1500,
        circular_phase=2.0,
    )
    layer.generate()

    layer.to_svg("examples/svg/draperie_direct.svg")
    print("Draperie direct (DraperieLayer) → examples/svg/draperie_direct.svg")


def draperie_rose_engine():
    """
    Create draperie pattern using RoseEngineLatheRun.draperie() (rose engine simulation).

    This produces identical output to the mathematical DraperieLayer.
    The rose engine configures concentric rings with sinusoidal rosette modulation
    and the same phase shape function (dome or sin^e) as the direct method.
    """
    run = RoseEngineLatheRun.draperie(
        num_rings=96,
        base_radius=22.0,
        radius_step=0.44,
        wave_frequency=12.0,
        phase_oscillations=2.5,
        resolution=1500,
        circular_phase=2.0,
    )
    run.generate()

    run.to_svg("examples/svg/draperie_rose_engine.svg")
    print("Draperie rose engine (RoseEngineLatheRun) → examples/svg/draperie_rose_engine.svg")


def main():
    print("Generating Draperie (Drapery) Guilloché Patterns\n")
    print("=" * 55)

    # Method 1: Watch face with mathematical draperie
    print("\nMethod 1: WatchFace.add_draperie() (smooth)")
    print("-" * 35)
    watchface_draperie()

    # Method 2: Watch face with sharp draperie
    print("\nMethod 2: WatchFace.add_draperie_sharp()")
    print("-" * 35)
    watchface_draperie_sharp()

    # Method 3: Watch face with soft wave crests
    print("\nMethod 3: WatchFace.add_draperie(wave_exponent=3)")
    print("-" * 35)
    watchface_draperie_soft_wave()

    # Method 4: Standalone DraperieLayer
    print("\nMethod 4: DraperieLayer (standalone)")
    print("-" * 35)
    draperie_direct()

    # Method 5: Rose engine simulation
    print("\nMethod 5: RoseEngineLatheRun.draperie()")
    print("-" * 35)
    draperie_rose_engine()

    print("\n" + "=" * 55)
    print("Done! Check examples/svg/ for output files.")
    print("\nNote: DraperieLayer and RoseEngineLatheRun.draperie() produce identical output.")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    main()
