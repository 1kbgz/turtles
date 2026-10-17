# A rose-engine pattern

In this tutorial, we will draw a six-lobe rose-engine pass, repeat it at several
phase offsets, and export a complete draperie preset.

Use the environment from [your first pattern](first-pattern.md). All files are
written to the current directory.

## Generate one pass

Create a script named `rose_pattern.py`:

```python
from turtles import (
    CuttingBit,
    RoseEngineConfig,
    RoseEngineLathe,
    RoseEngineLatheRun,
    RosettePattern,
)

config = RoseEngineConfig(base_radius=15.0, amplitude=2.0)
config.set_rosette(RosettePattern.multi_lobe(lobes=6))
config.set_resolution(360)
bit = CuttingBit.v_shaped(angle=60.0, width=0.2)

lathe = RoseEngineLathe(config, bit)
lathe.generate()
lathe.to_svg("rose-pass.svg")
print("Created rose-pass.svg")
```

Run `python rose_pattern.py` and open the SVG. Notice six lobes and the close
parallel traces beside the centerline. Those traces are the single-pass
model's width-offset edges.

## Repeat the pass

Append:

```python
run = RoseEngineLatheRun(config, bit, num_passes=12, segments_per_pass=24)
run.generate()
run.to_svg("rose-run.svg")
assert run.num_passes == 12
assert run.get_lines()
print("Created a 12-pass run")
```

Run the script again and open `rose-run.svg`. You should see repeated,
phase-shifted strokes with gaps, rather than the three continuous traces of
the single-pass SVG. The pass count is a property, so the check uses
`run.num_passes`, without parentheses.

## Generate a named preset

Append:

```python
draperie = RoseEngineLatheRun.draperie(
    num_rings=24,
    base_radius=12.0,
    radius_step=0.3,
    wave_frequency=12.0,
    resolution=360,
)
draperie.generate()
draperie.to_svg("rose-draperie.svg")
assert draperie.get_lines()
print("Created rose-draperie.svg")
```

Run the script and open the third SVG. Concentric wavy rings form folded bands.
This factory returns a separate run; it does not change `config` or the earlier
run.

You have created single-pass, general multi-pass, and preset output. Read
[pattern models](../explanation/pattern-models.md) for the relationship between
presets and direct layers. The [rose-engine reference](../reference/rose-engine.md)
lists the available rosettes and factories. Export options differ by class; a
multi-pass run exposes SVG and sampled lines, not STL or STEP methods.
