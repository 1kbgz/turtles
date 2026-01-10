"""
Generate Draperie (Drapery) Guilloché Pattern

This example creates a drapery guilloché pattern that mimics flowing fabric with
wave-like folds, creating an elegant flowing appearance.
"""

from turtles import CuttingBit, RoseEngineConfig, RoseEngineLathe

# Create a draperie pattern configuration
# wave_frequency controls how many wave folds appear
config = RoseEngineConfig.draperie(base_radius=20.0, wave_frequency=6.0, amplitude=2.0)

# Create a V-shaped cutting bit (30 degree angle, 0.4mm width)
bit = CuttingBit.v_shaped(angle=30.0, width=0.4)

# Create and generate the lathe pattern
lathe = RoseEngineLathe(config, bit)
lathe.generate()

# Export to SVG
output_file = "examples/svg/draperie.svg"
lathe.to_svg(output_file)
print(f"Generated {output_file}")
