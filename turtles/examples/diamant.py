#!/usr/bin/env python3
"""
Diamant (Diamond) Guilloché Pattern Example

This example demonstrates a multi-pass rose engine creating a classic diamant
guilloché pattern by making multiple overlapping cuts at different rotations.
This creates geometric diamond grids characteristic of traditional
watchmaking guilloché patterns.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with diamant rosette
    config = RoseEngineConfig(base_radius=20.0, amplitude=1.5)
    config.set_rosette(RosettePattern.diamant(divisions=12))
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create multi-pass run with 12 rotational passes
    run = RoseEngineLatheRun(config, bit, num_passes=12)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/diamant.svg")
    print("Generated examples/svg/diamant.svg")


if __name__ == "__main__":
    main()
