"""
Generate Huit-Eight (Figure-Eight) Guilloché Pattern

This example creates a classic figure-eight guilloché pattern with interlocking loops,
similar to those used in fine watchmaking.
"""

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

# Create a huit-eight pattern configuration
# This creates interlocking figure-eight loops
config = RoseEngineConfig.huit_eight(base_radius=20.0, amplitude=2.0)

# Create a V-shaped cutting bit (30 degree angle, 0.5mm width)
bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

# Create and generate the lathe pattern
lathe = RoseEngineLathe(config, bit)
lathe.generate()

# Export to SVG
output_file = "examples/svg/huit_eight.svg"
lathe.to_svg(output_file)
print(f"Generated {output_file}")
