# Your first pattern

In this tutorial, we will generate a diagonal Clous de Paris grid, save it as an
SVG, and inspect the lines that form it. The result is a circular hobnail pattern
without a watch-face frame.

Use Python 3.11 or later in a writable directory. You will need a text editor and
a browser or another SVG viewer. The
[installation guide](../how-to/install.md) covers platform build prerequisites
if a compatible wheel is unavailable.

## Install the package

Create a virtual environment and install the Python distribution:

```sh
python -m venv .venv
source .venv/bin/activate
python -m pip install pyturtles
```

The activation command above is for a POSIX shell. On Windows PowerShell, use
`.venv\Scripts\Activate.ps1` for that step. The distribution is named `pyturtles`,
but the import is `turtles`.

## Create a small grid

Create a script named `first_pattern.py` containing:

```python
from turtles import ClousDeParisLayer

pattern = ClousDeParisLayer(spacing=1.0, radius=10.0, resolution=20)
pattern.generate()
pattern.to_svg("pattern.svg")
print("Created pattern.svg")
```

Run the script:

```sh
python first_pattern.py
```

You should see `Created pattern.svg` and a new file in the current directory.
Open it in your SVG viewer. Notice two perpendicular sets of diagonal lines,
with their ends trimmed to a circle. The circle has a radius of 10 mm.

## Inspect the generated lines

Append this block to the same script and run it again:

```python
lines = pattern.get_lines()
assert lines
assert all(len(line) >= 2 for line in lines)
assert all(len(point) == 2 for line in lines for point in line)
print("Each line contains (x, y) points")
```

The script now also prints `Each line contains (x, y) points`. `get_lines()`
returns a list of lines, each containing coordinate pairs.

## Make the grid finer

Append one more block:

```python
fine = ClousDeParisLayer(spacing=0.5, radius=10.0, resolution=20)
fine.generate()
fine.to_svg("pattern-fine.svg")
assert len(fine.get_lines()) > len(lines)
print("Created a finer grid")
```

Run the script and open `pattern-fine.svg`. The circle is the same size; the
grooves are closer together. Both SVGs remain available for comparison.

You have generated and inspected two standalone patterns. Continue with
[a patterned watch dial](watch-dial.md) to place motifs inside a dial. The
[Clous de Paris reference](../reference/patterns.md#clousdeparislayer) lists its
positioning factories and constraints.
