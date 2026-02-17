"""
Flinqué (Engine-Turned) Guilloché Pattern Example

Demonstrates the flinqué guilloché pattern — a classic engine-turned motif
with petal-like lobes and fine wavy engraving, commonly found on watch dials,
cigarette cases, and decorative metalwork.

Two approaches are provided:
1. FlinqueLayer via GuillochePattern - Standalone pattern (add layer, generate, export)
2. WatchFace.add_flinque() - High-level watch face API
"""

import os

from turtles import FlinqueLayer, WatchFace
from turtles.turtles import GuillochePattern


def flinque_direct():
    """
    Create a flinqué pattern using FlinqueLayer added to a GuillochePattern.

    FlinqueLayer defines the pattern geometry; GuillochePattern is the container
    that generates and exports it.
    """
    layer = FlinqueLayer(
        radius=38.0,
        num_petals=12,
        num_waves=60,
        wave_amplitude=0.8,
        wave_frequency=20.0,
        inner_radius_ratio=0.05,
    )

    pattern = GuillochePattern(radius=38.0)
    pattern.add_flinque_layer(layer)
    pattern.generate()
    pattern.to_svg("examples/svg/flinque_direct.svg")
    print("Flinqué direct (FlinqueLayer) → examples/svg/flinque_direct.svg")


def flinque_dense():
    """
    Create a denser flinqué pattern with more petals and waves.
    """
    layer = FlinqueLayer(
        radius=38.0,
        num_petals=18,
        num_waves=120,
        wave_amplitude=0.5,
        wave_frequency=30.0,
        inner_radius_ratio=0.03,
    )

    pattern = GuillochePattern(radius=38.0)
    pattern.add_flinque_layer(layer)
    pattern.generate()
    pattern.to_svg("examples/svg/flinque_dense.svg")
    print("Flinqué dense (18 petals)     → examples/svg/flinque_dense.svg")


def flinque_with_center():
    """
    Create a flinqué pattern offset from the origin using FlinqueLayer.with_center().
    """
    layer = FlinqueLayer.with_center(
        radius=15.0,
        center_x=10.0,
        center_y=10.0,
        num_petals=8,
        num_waves=40,
        wave_amplitude=1.0,
        wave_frequency=15.0,
        inner_radius_ratio=0.06,
    )

    pattern = GuillochePattern(radius=38.0)
    pattern.add_flinque_layer(layer)
    pattern.generate()
    pattern.to_svg("examples/svg/flinque_offset.svg")
    print("Flinqué offset (with_center)  → examples/svg/flinque_offset.svg")


def watchface_flinque():
    """
    Create a watch face with a flinqué pattern at 12 o'clock.
    """
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    wf.add_flinque(
        radius=10.0,
        hour=12,
        minute=0,
        distance=18.0,
        num_petals=12,
        num_waves=60,
    )
    wf.generate()

    wf.to_svg("examples/svg/flinque_watchface.svg")
    print("Watch face flinqué (12 o'clock) → examples/svg/flinque_watchface.svg")


def watchface_flinque_multi():
    """
    Create a watch face with flinqué patterns at the four cardinal positions.
    """
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()

    for hour in (12, 3, 6, 9):
        wf.add_flinque(
            radius=8.0,
            hour=hour,
            minute=0,
            distance=18.0,
            num_petals=8,
            num_waves=40,
            wave_amplitude=0.6,
        )
    wf.generate()

    wf.to_svg("examples/svg/flinque_watchface_multi.svg")
    print("Watch face flinqué (4 positions) → examples/svg/flinque_watchface_multi.svg")


def main():
    print("Generating Flinqué (Engine-Turned) Guilloché Patterns\n")
    print("=" * 55)

    # Method 1: Standalone FlinqueLayer via GuillochePattern
    print("\nMethod 1: FlinqueLayer (standalone)")
    print("-" * 35)
    flinque_direct()

    # Method 2: Dense variant
    print("\nMethod 2: FlinqueLayer (dense, 18 petals)")
    print("-" * 35)
    flinque_dense()

    # Method 3: Offset placement
    print("\nMethod 3: FlinqueLayer.with_center()")
    print("-" * 35)
    flinque_with_center()

    # Method 4: Watch face with single flinqué
    print("\nMethod 4: WatchFace.add_flinque()")
    print("-" * 35)
    watchface_flinque()

    # Method 5: Watch face with four flinqué patterns
    print("\nMethod 5: WatchFace.add_flinque() × 4")
    print("-" * 35)
    watchface_flinque_multi()

    print("\n" + "=" * 55)
    print("Done! Check examples/svg/ for output files.")


if __name__ == "__main__":
    os.makedirs(os.path.join("examples", "svg"), exist_ok=True)
    main()
