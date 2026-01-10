#!/usr/bin/env python3
"""
Grain-de-Riz (Rice Grain) Guilloché Pattern Example

This example demonstrates a multi-pass rose engine creating a classic grain-de-riz
guilloché pattern by making multiple overlapping cuts at different rotations.
This creates the complex intersecting geometry characteristic of traditional
watchmaking guilloché patterns.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with grain-de-riz rosette
    config = RoseEngineConfig(base_radius=20.0, amplitude=1.5)
    config.set_rosette(RosettePattern.grain_de_riz(grain_size=1.0, rows=12))
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create multi-pass run with 24 rotational passes for fine detail
    run = RoseEngineLatheRun(config, bit, num_passes=24)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/grain_de_riz.svg")
    print("Generated examples/svg/grain_de_riz.svg")


if __name__ == "__main__":
    main()
