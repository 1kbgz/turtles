#!/usr/bin/env python3
"""
Diamant (Diamond) Guilloché Pattern Example

This example creates a diamant guilloché pattern using multi-lobe modulation
with many overlapping passes, creating intersecting lines that form
diamond/checkerboard shapes.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with multi-lobe pattern for diamond effect
    # Diamant patterns have geometric, angular appearance with sharp points
    config = RoseEngineConfig(base_radius=20.0, amplitude=0.5)
    config.set_rosette(RosettePattern.multi_lobe(lobes=36))
    config.set_resolution(2000)  # High resolution for sharp angles
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.025)
    
    # Create multi-pass run - number of passes should create diamond grid
    # Using same number as lobes creates nice diamond intersections
    run = RoseEngineLatheRun(config, bit, num_passes=36)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/diamant.svg")
    print("Generated examples/svg/diamant.svg")


if __name__ == "__main__":
    main()
