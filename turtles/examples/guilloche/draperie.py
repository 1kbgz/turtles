"""
Guilloché Pattern: Draperie (Drapery)

This example recreates the draperie guilloché pattern
featuring flowing, curtain-like waves.

Pattern characteristics:
- Smooth, flowing wave patterns
- Resembles fabric drapery or curtains
- Creates sense of movement and depth
- Elegant and sophisticated
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for draperie pattern
# Uses sinusoidal rosette to create smooth waves
config = RoseEngineConfig.draperie(base_radius=20.0)

# Can also create it manually:
# config = RoseEngineConfig(
#     rosette=RosettePattern.sinusoidal(frequency=8.0),
#     amplitude=1.2,
#     base_radius=20.0,
#     resolution=1500
# )

# Round bit creates smooth, flowing waves
bit = CuttingBit.round(radius=0.4)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("draperie.svg")

print("Draperie pattern generated successfully!")
print("Output: draperie.svg")
print("\nThis pattern features:")
print("- Smooth, flowing wave patterns")
print("- Resembles draped fabric or curtains")
print("- Classic pattern for watch cases and decorative panels")
