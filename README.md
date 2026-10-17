<a href="https://github.com/1kbgz/turtles">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://github.com/1kbgz/turtles/raw/main/docs/img/logo-dark.png?raw=true">
    <img alt="turtles logo, top view of a green turtle with a spirograph/guilloche shell" src="https://github.com/1kbgz/turtles/raw/main/docs/img/logo-light.png?raw=true" width="400">
  </picture>
</a>

<br/>

[![Build Status](https://github.com/1kbgz/turtles/actions/workflows/build.yaml/badge.svg?branch=main&event=push)](https://github.com/1kbgz/turtles/actions/workflows/build.yaml)
[![codecov](https://codecov.io/gh/1kbgz/turtles/branch/main/graph/badge.svg)](https://codecov.io/gh/1kbgz/turtles)
[![License](https://img.shields.io/github/license/1kbgz/turtles)](https://github.com/1kbgz/turtles)
[![PyPI](https://img.shields.io/pypi/v/pyturtles.svg)](https://pypi.python.org/pypi/pyturtles)

## Overview

A Python and Rust library for generating spirograph and guilloché patterns.

## Documentation

Start with [your first pattern](https://github.com/1kbgz/turtles/blob/main/docs/tutorials/first-pattern.md), or browse the
[documentation](https://github.com/1kbgz/turtles/blob/main/docs/home.md):

- [Tutorials](https://github.com/1kbgz/turtles/blob/main/docs/tutorials/index.md): generate a pattern, compose a dial, and run a rose engine.
- [How-to guides](https://github.com/1kbgz/turtles/blob/main/docs/how-to/index.md): install, position layers, export, and develop.
- [Reference](https://github.com/1kbgz/turtles/blob/main/docs/reference/index.md): Python APIs, Rust entry points, units, defaults, and errors.
- [Explanation](https://github.com/1kbgz/turtles/blob/main/docs/explanation/index.md): pattern models, coordinates, clipping, and export geometry.

## Examples

### Flinqué

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/flinque.png" width=500>

### Diamant

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/diamant.png" width=500>

### Limaçon

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/limacon.png" width=500>

### Draperie

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/draperie_circular.png" width=500>
<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/draperie.png" width=500>
<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/draperie_sharp.png" width=500>
<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/draperie_petal.png" width=500>

### Paon

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/paon.png" width=500>

### Huit-Eight (Figure-Eight)

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/huiteight.png" width=500>

### Clous de Paris

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/clous_de_paris.png" width=500>

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/clous_de_paris_square.png" width=500>

### Cube

<img src="https://raw.githubusercontent.com/1kbgz/turtles/refs/heads/main/docs/img/cube.png" width=500>

### Export Formats

- **SVG** - 2D vector previews and pattern paths.
- **STEP (.stp)** - Sampled CAD curves, not solid geometry or a CNC program.
- **STL** - Triangle ribbons, not watertight plates or swept grooves.

Supported methods differ by class. See the [export reference](https://github.com/1kbgz/turtles/blob/main/docs/reference/exports.md)
before choosing an export route.

## Installation

### Rust

The library crate is `turtles`, defined in the nested Rust package. See
[installation](https://github.com/1kbgz/turtles/blob/main/docs/how-to/install.md) for a local Cargo dependency and the
[Rust reference](https://github.com/1kbgz/turtles/blob/main/docs/reference/rust.md) for a complete example.

### Python

Requires Python 3.11 or later.

```bash
pip install pyturtles
```

Import the package as `turtles`. Source builds require Rust and a native linker;
see [installation](https://github.com/1kbgz/turtles/blob/main/docs/how-to/install.md).

### JavaScript

JavaScript bindings are not currently included.

## License

Apache-2.0

## Notes

> [!NOTE]
> This library was generated using [copier](https://copier.readthedocs.io/en/stable/) from the [Base Python Project Template repository](https://github.com/python-project-templates/base).
