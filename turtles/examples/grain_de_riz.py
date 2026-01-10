#!/usr/bin/env python3
"""
Grain-de-Riz (Rice Grain) Rose Engine Pattern Example

This example demonstrates a rose engine lathe configured with a grain-de-riz
rosette pattern. Note: This creates a single-pass modulated circular pattern.
Classic guilloché grain-de-riz patterns require small individual grain shapes
arranged in a grid.
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with grain-de-riz rosette
    config = RoseEngineConfig.grain_de_riz(base_radius=20.0, grain_size=1.0, amplitude=1.5)
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create and generate the lathe pattern
    lathe = RoseEngineLathe(config, bit)
    lathe.generate()
    
    # Export to SVG
    lathe.to_svg("examples/svg/grain_de_riz.svg")
    print("Generated examples/svg/grain_de_riz.svg")


if __name__ == "__main__":
    main()
