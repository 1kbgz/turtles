#!/usr/bin/env python3
"""
Draperie (Drapery) Guilloché Pattern Example

This example demonstrates a multi-pass rose engine creating a classic draperie
guilloché pattern by making multiple overlapping cuts at different rotations.
This creates flowing wave-like patterns characteristic of traditional
watchmaking guilloché drapery designs.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with draperie rosette
    config = RoseEngineConfig(base_radius=20.0, amplitude=2.0)
    config.set_rosette(RosettePattern.draperie(frequency=6.0, depth_frequency=3.0))
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create multi-pass run with 18 rotational passes
    run = RoseEngineLatheRun(config, bit, num_passes=18)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/draperie.svg")
    print("Generated examples/svg/draperie.svg")


if __name__ == "__main__":
    main()
