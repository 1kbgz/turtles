#!/usr/bin/env python3
"""
Draperie (Drapery) Guilloché Pattern Example

This example creates a draperie guilloché pattern using low-frequency
sinusoidal modulation with segmented passes, creating flowing wave-like
lines that resemble draped fabric.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with low-frequency wave for flowing drapery effect
    # Draperie has gentle, flowing waves
    config = RoseEngineConfig(base_radius=20.0, amplitude=1.0)
    config.set_rosette(RosettePattern.sinusoidal(frequency=10.0))
    config.set_resolution(1500)  # Good resolution for smooth curves
    
    # Create a fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.03)
    
    # Create multi-pass run with segmentation for flowing mesh effect
    run = RoseEngineLatheRun(config, bit, num_passes=36, segments_per_pass=24)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/draperie.svg")
    print("Generated examples/svg/draperie.svg")
    print(f"  Pattern: Segmented with {36 * 24} individual arc segments")


if __name__ == "__main__":
    main()
