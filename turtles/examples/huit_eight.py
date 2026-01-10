#!/usr/bin/env python3
"""
Huit-Eight (Figure-Eight) Rose Engine Pattern Example

This example demonstrates a rose engine lathe configured with a figure-eight
rosette pattern. Note: This creates a single-pass modulated circular pattern.
Classic guilloché huit-eight patterns require multiple overlapping tool passes.
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with huit-eight rosette
    config = RoseEngineConfig.huit_eight(base_radius=20.0, amplitude=3.0)
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create and generate the lathe pattern
    lathe = RoseEngineLathe(config, bit)
    lathe.generate()
    
    # Export to SVG
    lathe.to_svg("examples/svg/huit_eight.svg")
    print("Generated examples/svg/huit_eight.svg")


if __name__ == "__main__":
    main()
