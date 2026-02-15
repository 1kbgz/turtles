"""
Draperie (Drapery) Guilloché Pattern Example

Demonstrates the DraperieLayer - a clean, high-level API for generating
classical draperie guilloché patterns (flowing fabric-like concentric
wavy rings). No manual amplitude calculation needed.
"""

import os

from turtles import WatchFace


def watchface_draperie():
    """Create a watch face with a smooth draperie pattern using WatchFace.add_draperie()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie(phase_oscillations=3, circular_phase=0.5)  # all defaults (circular_phase=True, rounded)
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

    wf.add_draperie_sharp()  # circular_phase=False, phase_exponent=1, sharp V-shaped folds
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


def watchface_draperie_sharp_soft_wave():
    """Create a watch face with sharp folds AND soft wave crests."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie_sharp(wave_exponent=3)  # sharp folds + soft wave peaks
    wf.generate()

    output_path = os.path.join("examples", "svg", "draperie_sharp_soft_wave.svg")
    wf.to_svg(output_path)
    print(f"Watch face draperie (sharp+soft) → {output_path}")


def watchface_draperie_petal():
    """Create a watch face with a smooth draperie pattern using WatchFace.add_draperie()."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_draperie(phase_oscillations=3, circular_phase=0.5)  # all defaults (circular_phase=True, rounded)
    wf.generate()

    output_path = os.path.join("examples", "svg", "draperie.svg")
    wf.to_svg(output_path)
    print(f"Watch face draperie (smooth) → {output_path}")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    watchface_draperie()
    watchface_draperie_sharp()
    watchface_draperie_soft_wave()
    watchface_draperie_sharp_soft_wave()
    watchface_draperie_petal()
