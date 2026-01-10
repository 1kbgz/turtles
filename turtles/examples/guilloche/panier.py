"""
Guilloché Pattern: Panier (Basket Weave)

This example recreates the panier guilloché pattern
featuring a basket weave or interlaced design.

Pattern characteristics:
- Interlaced lines creating woven appearance
- Resembles basket weaving pattern
- Complex and intricate design
- Creates rich textured surface
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for panier (basket weave) pattern
# Uses epicycloid rosette for complex interlacing
config = RoseEngineConfig(
    rosette=RosettePattern.epicycloid(petals=6),
    amplitude=1.8,
    base_radius=20.0,
    resolution=1800
)

# Alternative: Use multi-lobe with specific parameters
# config = RoseEngineConfig(
#     rosette=RosettePattern.multi_lobe(lobes=6),
#     amplitude=1.5,
#     base_radius=20.0,
#     resolution=1800
# ).with_phase(0.5)  # Phase shift creates weave effect

# Round bit creates smooth interlaced appearance
bit = CuttingBit.round(radius=0.35)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("panier.svg")

print("Panier (Basket Weave) pattern generated successfully!")
print("Output: panier.svg")
print("\nThis pattern features:")
print("- Interlaced lines creating woven texture")
print("- Complex and intricate design")
print("- Traditional pattern for decorative surfaces")
