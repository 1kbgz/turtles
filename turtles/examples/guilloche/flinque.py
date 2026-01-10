"""
Guilloché Pattern: Flinqué

This example recreates the classic flinqué guilloché pattern
seen in luxury watch dials and subdials.

Pattern characteristics:
- Radial sunburst with chevron waves
- Creates a sparkling, reflective appearance
- Multiple concentric rings with wave modulation
- Sharp V-shaped peaks creating "petals"
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for flinqué pattern
# The flinqué pattern uses a multi-lobe rosette to create the radial petals
config = RoseEngineConfig.flinque(
    num_petals=12,      # Classic 12-petal design
    base_radius=20.0    # 20mm radius pattern
)

# Select a V-shaped cutting bit for sharp, crisp petals
bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("flinque.svg")

print("Flinqué pattern generated successfully!")
print("Output: flinque.svg")
print("\nThis pattern features:")
print("- 12 radial petals creating a sunburst effect")
print("- Sharp V-shaped cuts for maximum light reflection")
print("- Classic design used in high-end watch subdials")
