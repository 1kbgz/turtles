#!/usr/bin/env python3
"""
Huit-Eight (Figure-Eight) Guilloché Pattern Example

This example creates a huit-eight guilloché pattern using high-frequency
sinusoidal modulation with many overlapping segmented passes, creating the
characteristic dense wave texture seen in traditional watchmaking.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with high-frequency sinusoidal wave
    # Huit-eight patterns have dense, regular wave texture
    config = RoseEngineConfig(base_radius=20.0, amplitude=0.4)
    config.set_rosette(RosettePattern.sinusoidal(frequency=24.0))
    config.set_resolution(2000)  # High resolution for smooth curves
    
    # Create a very fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.02)
    
    # Create multi-pass run with segmentation for classical guilloché appearance
    # More passes = denser texture; segments create gaps for mesh-like appearance
    run = RoseEngineLatheRun(config, bit, num_passes=60, segments_per_pass=36)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/huit_eight.svg")
    print("Generated examples/svg/huit_eight.svg")
    print(f"  Pattern: Segmented with {60 * 36} individual arc segments")


if __name__ == "__main__":
    main()
