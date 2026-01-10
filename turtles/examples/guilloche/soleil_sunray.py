"""
Guilloché Pattern: Soleil/Sunray

This example recreates the soleil (sunray) guilloché pattern,
one of the most popular patterns in luxury watchmaking.

Pattern characteristics:
- Radial lines emanating from center like sun rays
- Creates dramatic light play
- Very popular on modern watch dials
- Clean and dynamic appearance
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for sunray pattern
# Uses multi-lobe rosette with higher lobe count
config = RoseEngineConfig.sunray(
    num_rays=48,        # Many rays for fine sunburst effect
    base_radius=20.0    # 20mm radius
)

# Can also create it manually:
# config = RoseEngineConfig(
#     rosette=RosettePattern.multi_lobe(lobes=48),
#     amplitude=1.5,
#     base_radius=20.0,
#     resolution=2000
# )

# V-shaped bit with sharp angle for defined rays
bit = CuttingBit.v_shaped(angle=30.0, width=0.4)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("soleil_sunray.svg")

print("Soleil/Sunray pattern generated successfully!")
print("Output: soleil_sunray.svg")
print("\nThis pattern features:")
print("- Radial lines emanating from center")
print("- Creates stunning light play and depth")
print("- Most popular guilloché pattern in modern watchmaking")
