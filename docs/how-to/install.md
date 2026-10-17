# How to install Turtles

Install the **`pyturtles` distribution** with Python 3.11 or newer. Import it as
**`turtles`**. The Python package contains a compiled Rust extension; installing a
compatible wheel does not require a Rust toolchain, but building from source does.

## Install the Python package

Create and activate a virtual environment. These commands use a POSIX shell; on
Windows, use the activation command appropriate to your shell from the virtual
environment's Scripts directory.

```sh
python3.11 -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install pyturtles
```

You can substitute another Python version supported by
[the package metadata](../../pyproject.toml). Keep using the environment's
`python -m pip` so installation and execution use the same interpreter.

If a compatible wheel is unavailable, pip can select a source distribution and
compile it. To require a wheel instead of compiling locally, replace the final
installation command with:

```sh
python -m pip install --only-binary=:all: pyturtles
```

If that command reports no matching distribution, use the source-installation
instructions below or choose an interpreter and platform with an available wheel.
This guide does not assume that every supported Python/platform combination has
a published wheel.

## Verify installation

Run this code with the same interpreter used for installation. It imports the
extension and generates a small pattern without creating output files.

```python
import turtles
from turtles import ClousDeParisLayer

layer = ClousDeParisLayer(spacing=1.0, radius=6.0, resolution=2)
layer.generate()
assert layer.get_lines()
print(turtles.__version__)
```

Use [the first-pattern tutorial](../tutorials/first-pattern.md) if you want to
create a drawing next, or go directly to
[positioning and composition](position-and-compose.md).

## Install from a checkout

Install a stable Rust toolchain, including `rustc` and `cargo`, through
[rustup](https://rustup.rs/). Source builds also need the platform's native linker
and build tools. On macOS, install the Xcode Command Line Tools if the compiler
reports a missing linker or SDK.

With Git, Python 3.11 or newer, and Rust available:

```sh
git clone https://github.com/1kbgz/turtles.git
cd turtles
python3.11 -m venv .venv
source .venv/bin/activate
python -m pip install --upgrade pip
python -m pip install -e .
```

The build configuration in [the Python project file](../../pyproject.toml) uses
Hatchling with the `hatch-rs` hook to compile the PyO3 extension. Installing from
source therefore requires Cargo even if you only intend to call the Python API.

An editable installation picks up edits to the Python wrapper. After changing
Rust code or bindings, repeat the editable installation to rebuild the extension,
then restart any Python process that already imported it.

For tests and documentation work, install the development and documentation
extras from the repository root:

```sh
python -m pip install -e '.[develop,docs]'
```

Continue with [the development guide](develop.md) for tests and build commands.

## Use the Rust library locally

Use [the nested Rust manifest](../../rust/Cargo.toml) for the Rust library crate
named `turtles`. [The repository-root manifest](../../Cargo.toml) instead builds
the Python extension package named `turtles_py`.

To build the library from the repository root:

```sh
cargo build --manifest-path rust/Cargo.toml
```

To consume it from another Rust project, add a path dependency to that project's
dependency table. For sibling checkouts named `my-design` and `turtles`, the entry
is:

```toml
[dependencies]
turtles = { path = "../turtles/rust" }
```

Adjust the path relative to the consuming project's manifest. Point it at the
nested Rust crate, not the repository root. See the [Rust reference](../reference/rust.md)
for public types and the differences from the Python API.

## Resolve installation failures

If `import turtles` fails, check which interpreter owns the installation:

```sh
python -m pip show pyturtles
```

If the distribution is missing, activate the intended environment and reinstall
it there. Do not replace the distribution name with the import name when running
pip.

If installation fails while compiling, check that Cargo and the Rust compiler
are on the active shell's search path:

```sh
cargo --version
rustc --version
```

If importing the package fails specifically at the compiled extension after a
source change, rebuild with the editable-install command above and restart the
interpreter. A successful checkout or an installed Python wrapper alone does not
mean that the native extension has been built.
