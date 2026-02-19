"""Huit-Eight (Figure-Eight / Lemniscate) guilloché example.

Generates SVGs using three approaches:
  1. Mathematical HuitEightLayer
  2. Rose engine mechanism (RoseEngineLatheRun.huiteight)
  3. WatchFace convenience method

Each approach is shown with uniform distribution and with clustered
distribution (curves grouped into visible petals, matching real
engine-turned dials).
"""

import os

from turtles import HuitEightLayer, RoseEngineLatheRun, WatchFace

# ---------------------------------------------------------------------------
# Uniform (original) examples
# ---------------------------------------------------------------------------


def huiteight_mathematical():
    """Generate a huit-eight pattern using the direct mathematical model."""
    layer = HuitEightLayer(num_curves=72, scale=20.0, resolution=360)
    layer.generate()
    layer.to_svg("examples/svg/huiteight_math.svg")


def huiteight_rose_engine():
    """Generate a huit-eight pattern via the rose engine mechanism."""
    run = RoseEngineLatheRun.huiteight(
        num_curves=72,
        scale=20.0,
        resolution=360,
    )
    run.generate()
    run.to_svg("examples/svg/huiteight_rose_engine.svg")


def huiteight_watchface():
    """Generate a huit-eight watch face."""
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()
    wf.add_huiteight(num_curves=36, scale=38.0, resolution=360)
    wf.generate()
    wf.to_svg("examples/svg/huiteight_watchface.svg")


# ---------------------------------------------------------------------------
# Clustered examples – curves grouped into visible petals
# ---------------------------------------------------------------------------


def huiteight_clustered_mathematical():
    """Generate a clustered huit-eight pattern using the mathematical model.

    8 clusters of ~9 curves each = 72 total, matching the petal grouping
    seen on real engine-turned watch dials.
    """
    layer = HuitEightLayer(
        num_curves=72,
        scale=20.0,
        resolution=360,
        num_clusters=8,
        cluster_spread=0.3,
    )
    layer.generate()
    layer.to_svg("examples/svg/huiteight_clustered_math.svg")


def huiteight_clustered_rose_engine():
    """Generate a clustered huit-eight pattern via the rose engine."""
    run = RoseEngineLatheRun.huiteight(
        num_curves=72,
        scale=20.0,
        resolution=360,
        num_clusters=8,
        cluster_spread=0.3,
    )
    run.generate()
    run.to_svg("examples/svg/huiteight_clustered_rose_engine.svg")


def huiteight_clustered_watchface():
    """Generate a clustered huit-eight watch face.

    Uses 8 clusters of 9 curves with auto-spread for a natural
    engine-turned appearance.
    """
    wf = WatchFace(radius=38.0)
    wf.add_inner()
    wf.add_outer()
    wf.add_center_hole()
    wf.add_huiteight(
        num_curves=72,
        scale=38.0,
        resolution=360,
        num_clusters=8,
        cluster_spread=0.3,
    )
    wf.generate()
    wf.to_svg("examples/svg/huiteight_clustered_watchface.svg")


def huiteight_tight_clusters():
    """Tighter clustering: 12 clusters with a narrow spread."""
    layer = HuitEightLayer(
        num_curves=72,
        scale=20.0,
        resolution=360,
        num_clusters=12,
        cluster_spread=0.15,
    )
    layer.generate()
    layer.to_svg("examples/svg/huiteight_tight_clusters.svg")


def huiteight_wide_clusters():
    """Wide clustering: 6 clusters with a wider spread – bold lobes."""
    layer = HuitEightLayer(
        num_curves=72,
        scale=20.0,
        resolution=360,
        num_clusters=6,
        cluster_spread=0.5,
    )
    layer.generate()
    layer.to_svg("examples/svg/huiteight_wide_clusters.svg")


def huiteight_auto_spread():
    """Use auto spread (cluster_spread=0) – spread is half the sector width."""
    layer = HuitEightLayer(
        num_curves=72,
        scale=20.0,
        resolution=360,
        num_clusters=8,
    )
    layer.generate()
    layer.to_svg("examples/svg/huiteight_auto_spread.svg")


def main():
    os.makedirs("examples/svg", exist_ok=True)

    # Uniform
    huiteight_mathematical()
    huiteight_rose_engine()
    huiteight_watchface()

    # Clustered
    huiteight_clustered_mathematical()
    huiteight_clustered_rose_engine()
    huiteight_clustered_watchface()

    # Variations
    huiteight_tight_clusters()
    huiteight_wide_clusters()
    huiteight_auto_spread()


if __name__ == "__main__":
    main()
