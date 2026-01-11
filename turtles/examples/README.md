# Rose Engine Python Examples

This directory contains example scripts demonstrating the Python bindings for the rose engine lathe functionality.

## Examples

### huit_eight.py
Demonstrates a multi-pass rose engine creating a classic figure-eight (huit-eight) guilloché pattern with 60 rotational passes, each segmented into 36 arcs (2160 total paths).

### grain_de_riz.py
Demonstrates a multi-pass rose engine creating a classic rice grain (grain-de-riz) guilloché pattern with 80 rotational passes, each segmented into 48 arcs (3840 total paths) for very fine detail.

### draperie.py
Demonstrates a multi-pass rose engine creating a classic drapery (draperie) guilloché pattern with 36 rotational passes, each segmented into 24 arcs (864 total paths) with flowing wave modulation.

### diamant.py
Demonstrates a multi-pass rose engine creating a classic diamond (diamant) guilloché pattern with 36 rotational passes, each segmented into 24 arcs (864 total paths) forming geometric diamond grids.

### flinque.py
Demonstrates the FlinqueLayer class which creates radial sunburst/engine-turned patterns with concentric wavy rings.

## Multi-Pass Segmented Guilloché Patterns

The examples use the `RoseEngineLatheRun` class, which simulates traditional watchmaking techniques by making **multiple overlapping tool passes** at different rotations. Each pass is further **segmented into multiple arcs with gaps**, creating the characteristic mesh-like appearance of classical guilloché patterns.

### Why Segmentation?

Classical guilloché patterns (as seen in luxury watch dials) show a complex mesh of intersecting lines rather than complete overlapping circles. Segmentation achieves this by:

1. **Breaking each circular pass into multiple arcs** (typically 24-48 segments)
2. **Creating gaps between arc segments** (70% drawing, 30% gap)
3. **Allowing the underlying pattern structure to show through**

This creates the textured, "breathable" appearance characteristic of true guilloché work.

### Example Usage

```python
from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern

config = RoseEngineConfig(base_radius=20.0, amplitude=0.4)
config.set_rosette(RosettePattern.sinusoidal(frequency=24.0))

bit = CuttingBit.v_shaped(angle=30.0, width=0.02)

# Create multi-pass run with segmentation
# 60 passes × 36 segments = 2160 individual arc paths
run = RoseEngineLatheRun(config, bit, num_passes=60, segments_per_pass=36)
run.generate()
run.to_svg("pattern.svg")
```

## Running the Examples

From the project root directory:

```bash
# Install the package first
python -m pip install -e .

# Run an example
python turtles/examples/huit_eight.py

# Or run all examples
for script in turtles/examples/*.py; do
    python "$script"
done
```

## Output

Examples generate SVG files in the `examples/svg/` directory showing complex guilloché patterns created by thousands of individual arc segments arranged in multi-pass formations.

