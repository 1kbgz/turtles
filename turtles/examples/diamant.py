"""
Generate Diamant (Diamond) Guilloché Pattern

This example creates a diamond guilloché pattern with crisply intersecting lines
forming diamond shapes, creating a jewel-like reflective appearance.
"""

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

# Create a diamant pattern configuration
# divisions controls the number of diamond facets
config = RoseEngineConfig.diamant(base_radius=20.0, divisions=12, amplitude=1.5)

# Create a V-shaped cutting bit (30 degree angle, 0.4mm width)
bit = CuttingBit.v_shaped(angle=30.0, width=0.4)

# Create and generate the lathe pattern
lathe = RoseEngineLathe(config, bit)
lathe.generate()

# Export to SVG
output_file = "examples/svg/diamant.svg"
lathe.to_svg(output_file)
print(f"Generated {output_file}")
