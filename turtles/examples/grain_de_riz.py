#!/usr/bin/env python3
"""
Grain-de-Riz (Rice Grain) Guilloché Pattern Example

This example creates a grain-de-riz guilloché pattern using very high-frequency
modulation with many overlapping segmented passes, creating the characteristic
fine "rice grain" texture seen in luxury watchmaking.
"""

from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern


def main():
    # Create configuration with very high-frequency wave for rice grain texture
    # Grain-de-riz has very fine, dense texture with small "grains"
    config = RoseEngineConfig(base_radius=20.0, amplitude=0.3)
    config.set_rosette(RosettePattern.sinusoidal(frequency=50.0))
    config.set_resolution(3000)  # Very high resolution for fine detail
    
    # Create a very fine V-shaped cutting bit
    bit = CuttingBit.v_shaped(angle=30.0, width=0.015)
    
    # Create multi-pass run with segmentation for fine grain mesh texture
    run = RoseEngineLatheRun(config, bit, num_passes=80, segments_per_pass=48)
    run.generate()
    
    # Export to SVG
    run.to_svg("examples/svg/grain_de_riz.svg")
    print("Generated examples/svg/grain_de_riz.svg")
    print(f"  Pattern: Segmented with {80 * 48} individual arc segments")


if __name__ == "__main__":
    main()
