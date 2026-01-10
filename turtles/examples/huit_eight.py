#!/usr/bin/env python3
"""
Huit-Eight (Figure-Eight) Guilloché Pattern Example

This example demonstrates a multi-pass rose engine creating a classic huit-eight
guilloché pattern by making multiple overlapping cuts at different rotations.
This creates the complex intersecting geometry characteristic of traditional
watchmaking guilloché patterns.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with huit-eight rosette
    config = RoseEngineConfig(base_radius=20.0, amplitude=3.0)
    config.set_rosette(RosettePattern.huit_eight(lobes=8))
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create multi-pass run with 16 rotational passes
    run = RoseEngineLatheRun(config, bit, num_passes=16)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/huit_eight.svg")
    print("Generated examples/svg/huit_eight.svg")


if __name__ == "__main__":
    main()
