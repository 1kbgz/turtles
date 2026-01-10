"""
Guilloché Pattern: Clou de Paris (Hobnail)

This example recreates the clou de paris guilloché pattern
featuring a hobnail or pyramid-like textured surface.

Pattern characteristics:
- Small pyramid or hobnail shapes
- Creates a three-dimensional textured surface
- Used extensively on watch bezels
- Classic and timeless design
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for clou de paris pattern
# Uses 8-lobe rosette with moderate amplitude
config = RoseEngineConfig.clou_de_paris(base_radius=20.0)

# Can also create it manually:
# config = RoseEngineConfig(
#     rosette=RosettePattern.multi_lobe(lobes=8),
#     amplitude=0.6,
#     base_radius=20.0,
#     resolution=1200
# )

# V-shaped bit creates sharp pyramid points
bit = CuttingBit.v_shaped(angle=60.0, width=0.5)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("clou_de_paris.svg")

print("Clou de Paris pattern generated successfully!")
print("Output: clou_de_paris.svg")
print("\nThis pattern features:")
print("- Hobnail/pyramid textured surface")
print("- Creates excellent grip on watch bezels")
print("- One of the most iconic guilloché patterns")
