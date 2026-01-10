# Rose Engine Python Examples

This directory contains example scripts demonstrating the Python bindings for the rose engine lathe functionality.

## Examples

### huit_eight.py
Demonstrates a multi-pass rose engine creating a classic figure-eight (huit-eight) guilloché pattern by making 16 rotational passes with overlapping cuts.

### grain_de_riz.py
Demonstrates a multi-pass rose engine creating a classic rice grain (grain-de-riz) guilloché pattern by making 24 rotational passes with fine detail.

### draperie.py
Demonstrates a multi-pass rose engine creating a classic drapery (draperie) guilloché pattern by making 18 rotational passes with flowing wave modulation.

### diamant.py
Demonstrates a multi-pass rose engine creating a classic diamond (diamant) guilloché pattern by making 12 rotational passes forming geometric diamond grids.

### flinque.py
Demonstrates the FlinqueLayer class which creates radial sunburst/engine-turned patterns with concentric wavy rings.

## Multi-Pass Guilloché Patterns

The examples use the `RoseEngineLatheRun` class, which simulates traditional watchmaking techniques by making **multiple overlapping tool passes** at different rotations. This creates the complex intersecting geometry characteristic of classical guilloché patterns.

Each pass is a complete circular sweep of the rose engine lathe, and the combination of all passes creates intricate patterns similar to those found in luxury watch dials.

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

Examples generate SVG files in the `examples/svg/` directory showing complex guilloché patterns created by multiple overlapping rose engine passes.
