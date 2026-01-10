#!/usr/bin/env python3
"""
Diamant (Diamond) Rose Engine Pattern Example

This example demonstrates a rose engine lathe configured with a diamant
rosette pattern. Note: This creates a single-pass modulated circular pattern.
Classic guilloché diamant patterns require intersecting diagonal tool passes
forming a diamond grid.
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with diamant rosette
    config = RoseEngineConfig.diamant(base_radius=20.0, divisions=12, amplitude=1.5)
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create and generate the lathe pattern
    lathe = RoseEngineLathe(config, bit)
    lathe.generate()
    
    # Export to SVG
    lathe.to_svg("examples/svg/diamant.svg")
    print("Generated examples/svg/diamant.svg")


if __name__ == "__main__":
    main()
