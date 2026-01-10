"""
Guilloché Pattern: Huit/Eight

This example recreates the "huit" (eight) guilloché pattern
featuring figure-8 or infinity-shaped curves.

Pattern characteristics:
- Figure-8 or infinity symbol-like curves
- Creates flowing, interlaced patterns
- Multiple overlapping loops
- Elegant and organic appearance
"""

from turtles import RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern

# Create configuration for huit/eight pattern
# Uses a low number of lobes to create the figure-8 effect
config = RoseEngineConfig(
    rosette=RosettePattern.multi_lobe(lobes=2),  # 2 lobes creates figure-8
    amplitude=2.5,          # Large amplitude for pronounced curves
    base_radius=20.0,       # 20mm radius
    resolution=1500         # High resolution for smooth curves
)

# Round bit creates smooth, flowing curves
bit = CuttingBit.round(radius=0.3)

# Create the rose engine lathe
lathe = RoseEngineLathe(config, bit)

# Generate the pattern
lathe.generate()

# Export to SVG
lathe.to_svg("huit_eight.svg")

print("Huit/Eight pattern generated successfully!")
print("Output: huit_eight.svg")
print("\nThis pattern features:")
print("- Figure-8 shaped curves")
print("- Smooth, flowing lines from round cutting bit")
print("- Classic design for decorative bezels and panels")
