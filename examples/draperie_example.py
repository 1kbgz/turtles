"""
Draperie (Drapery) Guilloché Pattern Example

Generates a draperie pattern matching the reference image examples/Draperie.jpg.
The draperie pattern creates flowing, fabric-like folds through overlapping
undulating circular curves - a classic guilloché motif in fine watchmaking.

Usage:
    python examples/draperie_example.py
"""

import os

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLatheRun


def generate_draperie():
    # Create draperie config
    # wave_frequency=6.0: 6 undulations per circle
    # amplitude=2.0: modulation depth
    # The preset automatically sets depth_frequency = wave_frequency * 2.0
    config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)

    # Fine V-bit for delicate lines
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)

    # 20 passes with complete curves (no gaps) creates the overlapping wavy circle mesh
    run = RoseEngineLatheRun(config, bit, num_passes=20, segments_per_pass=1)
    run.generate()

    # Export SVG
    output_path = os.path.join(os.path.dirname(__file__), "draperie_output.svg")
    run.to_svg(output_path)
    print(f"Draperie pattern saved to {output_path}")


if __name__ == "__main__":
    generate_draperie()
