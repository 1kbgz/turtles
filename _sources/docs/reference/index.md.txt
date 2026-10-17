# Reference

The Python reference describes the public `turtles` imports, including the
Python `WatchFace` wrapper and the classes exposed by the native extension.
Signatures include Python defaults, which can differ from Rust defaults.

## Python interfaces

- [Pattern layers and spirographs](patterns.md) lists the eight direct layers,
  three spirographs, positioning factories, units, properties, and constraints.
- [Composition](composition.md) describes `WatchFace`, `GuillochePattern`,
  copying, clipping, and draw order.
- [Rose engine](rose-engine.md) covers rosettes, cutting bits, configurations,
  single passes, and multi-pass runs.

## Output and diagnostics

- [Exports](exports.md) lists supported methods, file contents, and limitations.
- [Errors](errors.md) describes exceptions, validation timing, and checks that
  are not performed.
- [Pattern gallery](gallery.md) identifies the visual motifs and their example
  programs.

## Rust interfaces

[Rust API](rust.md) maps the public library modules, configuration types, geometry
accessors, and error types, with a complete crate example. Rustdoc provides the
full declaration-level reference.

```{toctree}
---
hidden:
---
patterns
composition
rose-engine
exports
errors
gallery
rust
```
