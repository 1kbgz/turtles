#!/usr/bin/env python3
"""
Flinqué (Sunburst) Guilloché Pattern Example

This example demonstrates the FlinqueLayer class which creates radial
sunburst/engine-turned patterns with multiple concentric wavy rings.
This is a true multi-pass guilloché effect unlike the single-pass rose engine examples.
"""

import sys
sys.path.insert(0, '.')
from turtles.turtles import GuillochePattern, FlinqueLayer


def main():
    # Create a guilloché pattern (uses watch face radius constraints)
    pattern = GuillochePattern(radius=35.0)
    
    # Create the flinqué layer with configuration parameters
    # The pattern is automatically generated on creation
    layer = FlinqueLayer(
        radius=20.0,
        num_petals=12,      # Number of radial segments
        num_waves=60,       # Number of concentric rings
        wave_amplitude=0.8,
        wave_frequency=20.0,
        inner_radius_ratio=0.05
    )
    
    # Add the layer to the pattern
    pattern.add_flinque_layer(layer)
    pattern.generate()
    
    # Export to SVG
    pattern.to_svg("examples/svg/flinque.svg")
    print("Generated examples/svg/flinque.svg")


if __name__ == "__main__":
    main()
