"""
Generate Grain-de-Riz (Rice Grain) Guilloché Pattern

This example creates a rice grain guilloché pattern with small elongated oval motifs
arranged in concentric rows, commonly seen on watch dials.
"""

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

# Create a grain-de-riz pattern configuration
# grain_size controls the size of each rice grain
config = RoseEngineConfig.grain_de_riz(base_radius=20.0, grain_size=1.0, amplitude=1.5)

# Create a V-shaped cutting bit (30 degree angle, 0.3mm width for fine detail)
bit = CuttingBit.v_shaped(angle=30.0, width=0.3)

# Create and generate the lathe pattern
lathe = RoseEngineLathe(config, bit)
lathe.generate()

# Export to SVG
output_file = "examples/svg/grain_de_riz.svg"
lathe.to_svg(output_file)
print(f"Generated {output_file}")
