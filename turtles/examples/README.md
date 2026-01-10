# Rose Engine Python Examples

This directory contains example scripts demonstrating the Python bindings for the rose engine lathe functionality.

## Examples

### huit_eight.py
Demonstrates a rose engine with a figure-eight (huit-eight) rosette pattern. Creates a single modulated circular path.

### grain_de_riz.py
Demonstrates a rose engine with a rice grain (grain-de-riz) rosette pattern. Creates a single modulated circular path.

### draperie.py
Demonstrates a rose engine with a drapery (draperie) rosette pattern. Creates a single modulated circular path with wave-like modulation.

### diamant.py
Demonstrates a rose engine with a diamond (diamant) rosette pattern. Creates a single modulated circular path with diamond-like modulation.

### flinque.py
Demonstrates the FlinqueLayer class which creates true multi-pass guilloché patterns with radial sunburst/engine-turned effects.

## Important Notes

The rose engine examples (huit_eight, grain_de_riz, draperie, diamant) demonstrate **single-pass** rose engine operations. Each generates one circular tool path with amplitude modulation based on the rosette pattern.

**Classical guilloché patterns** as seen in traditional watchmaking require **multiple overlapping tool passes** at different rotations and positions. The single-pass rose engine patterns shown here demonstrate the rose engine mechanism but do not replicate the complex intersecting geometry of traditional guilloché work.

To create patterns matching classical guilloché references, you would need to:
1. Generate multiple rose engine passes at different rotations
2. Composite the results into a single pattern
3. Or use dedicated multi-pass generators (like FlinqueLayer for sunburst patterns)

## Running the Examples

```bash
# Install the package first
cd ../..
python -m pip install -e .

# Run an example
python turtles/examples/huit_eight.py

# Or run all examples
for script in turtles/examples/*.py; do
    python "$script"
done
```

## Output

Examples generate SVG files in the `examples/svg/` directory.
