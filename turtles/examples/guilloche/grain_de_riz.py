"""
Guilloché Pattern: Grain de Riz (Rice Grain)

This example recreates the grain de riz guilloché pattern
featuring small oval/grain-shaped indentations.

Pattern characteristics:
- Small oval patterns resembling rice grains
- Often arranged in a grid or concentric pattern
- Creates a textured, matte appearance
- Subtle and elegant
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for grain de riz pattern
# Uses elliptical rosette to create the oval "grain" shapes
config = RoseEngineConfig.grain_de_riz(base_radius=20.0)

# Can also create it manually:
# config = RoseEngineConfig(
#     rosette=RosettePattern.elliptical(major_axis=2.0, minor_axis=1.0),
#     amplitude=0.5,
#     base_radius=20.0,
#     resolution=800
# )

# V-shaped bit with moderate angle
bit = CuttingBit.v_shaped(angle=45.0, width=0.4)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("grain_de_riz.svg")

print("Grain de Riz pattern generated successfully!")
print("Output: grain_de_riz.svg")
print("\nThis pattern features:")
print("- Small oval 'rice grain' shapes")
print("- Subtle texture creating matte appearance")
print("- Traditional pattern for watch dial backgrounds")
