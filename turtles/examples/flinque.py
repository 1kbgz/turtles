"""
Generate Flinqué (Sunburst/Engine-Turned) Pattern

This example creates a flinqué pattern using the existing FlinqueLayer class,
demonstrating the radial sunburst engine-turned pattern with chevron peaks.
"""

from turtles.turtles import FlinqueLayer, GuillochePattern

# Create a guilloche pattern container (must be between 26-44mm for watch face)
pattern = GuillochePattern(radius=38.0)

# Add a flinqué layer with sunburst pattern
# num_petals controls the number of radial chevron peaks
# num_waves controls the density of concentric rings
flinque = FlinqueLayer(
    radius=38.0,
    num_petals=12,  # 12 chevron peaks radiating from center
    num_waves=60,   # Dense concentric rings for fine line work
    wave_amplitude=0.8,  # Chevron amplitude
    wave_frequency=20.0,  # Fine ripple texture
    inner_radius_ratio=0.05,  # Start very close to center
)

pattern.add_flinque_layer(flinque)
pattern.generate()

# Export to SVG
output_file = "examples/svg/flinque.svg"
pattern.to_svg(output_file)
print(f"Generated {output_file}")
