#!/usr/bin/env python3
"""
Draperie (Drapery) Rose Engine Pattern Example

This example demonstrates a rose engine lathe configured with a draperie
rosette pattern. Note: This creates a single-pass modulated circular pattern.
Classic guilloché draperie patterns require multiple overlapping wavy tool passes.
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with draperie rosette
    config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.1)
    
    # Create and generate the lathe pattern
    lathe = RoseEngineLathe(config, bit)
    lathe.generate()
    
    # Export to SVG
    lathe.to_svg("examples/svg/draperie.svg")
    print("Generated examples/svg/draperie.svg")


if __name__ == "__main__":
    main()
