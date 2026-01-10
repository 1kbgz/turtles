# Guilloché Pattern Examples

This directory contains examples of classic guilloché patterns created using the Rose Engine Lathe module.

## What is Guilloché?

Guilloché is a decorative engraving technique in which a very precise, intricate, and repetitive pattern is mechanically engraved into an underlying material via engine turning, which uses a machine called a rose engine lathe. The patterns are used in watchmaking, jewelry, currency, and other fine crafts.

## Patterns Included

### Core Patterns

1. **Flinqué** (`flinque.py`) - Classic radial sunburst pattern with chevron waves
   - Reference: `examples/Guilloche-Flinque.jpg`
   - Traditional use: Watch dial backgrounds, subdials

2. **Huit/Eight** (`huit_eight.py`) - Figure-8 or infinity-shaped curves
   - Reference: `examples/Guilloche-huit-eight.jpg`
   - Traditional use: Decorative watch bezels, jewelry

3. **Grain de Riz** (`grain_de_riz.py`) - Rice grain pattern with small oval indentations
   - Reference: `examples/Guilloche-grain-de-riz.jpg`
   - Traditional use: Watch dials, decorative metalwork

4. **Draperie** (`draperie.py`) - Drapery/curtain pattern with flowing waves
   - Reference: `examples/Guilloche-draperie.jpg`
   - Traditional use: Watch cases, decorative panels

5. **Diamant** (`diamant.py`) - Diamond/lozenge-shaped pattern
   - Reference: `examples/Guilloche-diamant-diamond.jpg`
   - Traditional use: Luxury watch dials, high-end jewelry

### Bonus Patterns

6. **Clou de Paris** (`clou_de_paris.py`) - Hobnail pattern with pyramid-like texture
   - Reference: `examples/Guilloche-Clou-de-Paris.jpg`
   - Traditional use: Watch bezels, lighter cases

7. **Soleil/Sunray** (`soleil_sunray.py`) - Radial sunburst pattern
   - Reference: `examples/Guilloche-soleil-sunray.jpg`
   - Traditional use: Watch dials (very popular)

8. **Panier** (`panier.py`) - Basket weave pattern
   - Reference: `examples/Guilloche-panier.jpg`
   - Traditional use: Decorative surfaces

## How to Run

First, ensure you have the turtles package installed:

```bash
# From the repository root
make install
```

Then run any example:

```bash
python turtles/examples/guilloche/flinque.py
python turtles/examples/guilloche/diamant.py
```

Each example will generate SVG output files in the current directory.

## Understanding the Rose Engine

The rose engine lathe creates these patterns through three main components:

1. **Rosette Pattern** - A cam or template that modulates the radial position
   - `MultiLobe`: Creates n-pointed star patterns
   - `Sinusoidal`: Creates wave patterns
   - `Elliptical`: Creates oval modulations
   - `Epicycloid`: Creates mathematical rose curves

2. **Cutting Bit** - The tool that cuts into the material
   - `V-shaped`: Creates sharp, crisp cuts
   - `Flat`: Creates uniform width cuts
   - `Round`: Creates smooth, rounded cuts

3. **Configuration** - Parameters that control the pattern
   - `amplitude`: How much the pattern varies radially
   - `base_radius`: The average distance from center
   - `resolution`: How many points to generate
   - `phase`: Rotational offset of the pattern

## Pattern Parameters Guide

### Creating Your Own Patterns

You can create custom patterns by adjusting these parameters:

```python
from turtles import RoseEngineLathe, RoseEngineConfig, RosettePattern, CuttingBit

# Create a custom rosette
config = RoseEngineConfig(
    rosette=RosettePattern.multi_lobe(lobes=8),  # 8-pointed pattern
    amplitude=1.5,                                # Moderate variation
    base_radius=20.0,                             # 20mm from center
    resolution=1000                               # Smooth curve
)

# Choose a cutting bit
bit = CuttingBit.v_shaped(angle=30.0, width=0.5)

# Generate the pattern
lathe = RoseEngineLathe(config, bit)
lathe.generate()
lathe.to_svg("my_pattern.svg")
```

### Tips for Different Effects

- **Sharp, crisp patterns**: Use V-shaped bits with narrow angles (30°)
- **Soft, flowing patterns**: Use round bits or V-shaped with wide angles (90°)
- **Dense patterns**: Increase the number of lobes and reduce amplitude
- **Bold patterns**: Increase amplitude and use fewer lobes
- **Fine detail**: Increase resolution (2000+)
- **Quick preview**: Decrease resolution (500)

## Reference Images

All reference images are located in the `examples/` directory of the repository. These show traditional guilloché patterns found on luxury watches, fine jewelry, and historical currency.

## Further Reading

- [Guilloché on Wikipedia](https://en.wikipedia.org/wiki/Guilloch%C3%A9)
- Rose Engine Turning - Traditional technique and machinery
- Engine Turning - Decorative machining process
