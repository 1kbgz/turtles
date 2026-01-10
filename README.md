# turtles

[![Build Status](https://github.com/1kbgz/turtles/actions/workflows/build.yaml/badge.svg?branch=main&event=push)](https://github.com/1kbgz/turtles/actions/workflows/build.yaml)
[![codecov](https://codecov.io/gh/1kbgz/turtles/branch/main/graph/badge.svg)](https://codecov.io/gh/1kbgz/turtles)
[![License](https://img.shields.io/github/license/1kbgz/turtles)](https://github.com/1kbgz/turtles)
[![PyPI](https://img.shields.io/pypi/v/turtles.svg)](https://pypi.python.org/pypi/turtles)

## Overview

A high-performance Python library for generating spirograph and guilloche patterns for watch face manufacturing. Built with Rust for speed and precision, this library generates intricate geometric patterns suitable for CNC machining, laser engraving, and 3D printing.

## Features

### Pattern Types

- **Horizontal Spirograph** - Traditional hypotrochoid/epitrochoid patterns
- **Vertical Spirograph** - Patterns with vertical wave modulation
- **Spherical Spirograph** - 3D patterns projected onto spherical surfaces (for domed watch faces)
- **Guilloche Pattern** - Combine multiple spirograph layers for complex effects

### Export Formats

- **SVG** - 2D vector output for laser engraving and visualization
- **STEP (.stp)** - 3D CAD format for CNC milling toolpath generation
- **STL** - 3D mesh format for CNC machining and 3D printing

### Key Capabilities

- Configurable outer radius constrained to **26mm-44mm** (watch face standard)
- Adjustable pattern parameters (inner radius ratio, point distance, rotations)
- Multi-layer pattern composition for guilloche effects
- High-level API with preset watch face styles
- Validation and error handling for manufacturing constraints

## Installation

```bash
pip install turtles
```

## Quick Start

### Basic Spirograph Pattern

```python
from turtles import HorizontalSpirograph

# Create a horizontal spirograph pattern
spiro = HorizontalSpirograph(
    outer_radius=40.0,  # mm, must be 26-44mm
    radius_ratio=0.75,
    point_distance=0.6,
    rotations=50,
    resolution=360
)

# Generate and export
spiro.generate()
spiro.to_svg("pattern.svg")
spiro.to_step("pattern.stp", depth=0.1)  # 0.1mm groove depth
spiro.to_stl("pattern.stl", depth=0.1, base_thickness=2.0)
```

### Complex Guilloche Pattern

```python
from turtles import GuillochePattern, HorizontalSpirograph, VerticalSpirograph

# Combine multiple patterns
pattern = GuillochePattern(radius=38.0)
pattern.add_layer(HorizontalSpirograph(38.0, 0.75, 0.6, 50, 360))
pattern.add_layer(VerticalSpirograph(38.0, 0.6, 0.5, 30, 360, wave_amplitude=2.0))

pattern.generate()
pattern.export_all("watch_face")  # Creates .svg, .stp, .stl
```

### Watch Face Presets

```python
from turtles import WatchFaceGenerator, WatchFacePreset

# Use a preset style
gen = WatchFaceGenerator(radius=40.0, preset=WatchFacePreset.Luxury)
gen.generate()
gen.export_all("luxury_watch_face")
```

## Available Presets

- **Classic** - Traditional fine-line guilloche patterns
- **Modern** - Contemporary designs with vertical wave modulation
- **Vintage** - Complex overlapping patterns
- **Sport** - Bold, geometric patterns
- **Luxury** - Spherical dome effects with intricate details

## API Reference

### HorizontalSpirograph

Traditional spirograph pattern generator.

**Parameters:**

- `outer_radius` (float): Outer circle radius in mm (26-44mm)
- `radius_ratio` (float): Inner circle radius ratio (0-1)
- `point_distance` (float): Drawing point distance
- `rotations` (int): Number of rotations
- `resolution` (int): Points per revolution

**Methods:**

- `generate()`: Generate the pattern points
- `to_svg(filename)`: Export as SVG
- `to_step(filename, depth=0.1)`: Export as STEP with groove depth
- `to_stl(filename, depth=0.1, base_thickness=2.0)`: Export as STL

### VerticalSpirograph

Spirograph with vertical wave modulation.

**Additional Parameters:**

- `wave_amplitude` (float): Vertical wave amplitude
- `wave_frequency` (float): Vertical wave frequency

### SphericalSpirograph

3D spirograph projected onto a spherical surface.

**Additional Parameters:**

- `dome_height` (float): Height of the dome in mm

### GuillochePattern

Combines multiple spirograph layers.

**Methods:**

- `add_layer(spirograph)`: Add a spirograph layer
- `generate()`: Generate all layers
- `export_all(base_name, depth=0.1)`: Export all formats

### WatchFaceGenerator

High-level API with presets.

**Parameters:**

- `radius` (float): Watch face radius (26-44mm)
- `preset` (WatchFacePreset): Preset style

## Development

```bash
# Install development dependencies
make requirements

# Build the library
make build

# Run tests
make test

# Run linters
make lint
```

## License

Apache-2.0

## Notes

> [!NOTE]
> This library was generated using [copier](https://copier.readthedocs.io/en/stable/) from the [Base Python Project Template repository](https://github.com/python-project-templates/base).
