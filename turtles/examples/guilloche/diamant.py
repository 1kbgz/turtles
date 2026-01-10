"""
Guilloché Pattern: Diamant (Diamond)

This example recreates the diamant guilloché pattern
featuring diamond or lozenge-shaped patterns.

Pattern characteristics:
- Sharp diamond/lozenge shapes
- Creates a faceted, geometric appearance
- Reflective surfaces catch light like gemstones
- Bold and striking design
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for diamant pattern
# Uses 4-lobe rosette to create diamond shapes
config = RoseEngineConfig.diamant(base_radius=20.0)

# Can also create it manually:
# config = RoseEngineConfig(
#     rosette=RosettePattern.multi_lobe(lobes=4),
#     amplitude=1.0,
#     base_radius=20.0,
#     resolution=1000
# )

# V-shaped bit with sharp angle for crisp diamond facets
bit = CuttingBit.v_shaped(angle=30.0, width=0.6)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("diamant.svg")

print("Diamant pattern generated successfully!")
print("Output: diamant.svg")
print("\nThis pattern features:")
print("- Sharp diamond/lozenge shapes")
print("- Faceted appearance like gemstones")
print("- Popular pattern for luxury watch dials")
