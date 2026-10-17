# How to develop and check changes

Use an editable Python installation for wrapper and binding work, and address the
nested Rust crate explicitly for core tests and API documentation. Run the
commands below from the repository root unless a step says otherwise.

## Set up a development environment

Install Git, Make, [uv](https://docs.astral.sh/uv/getting-started/installation/),
and a stable Rust toolchain through [rustup](https://rustup.rs/). Source builds
need a native linker and platform build tools as described in
[the installation guide](install.md).

```sh
git clone https://github.com/1kbgz/turtles.git
cd turtles
uv venv --python 3.11
source .venv/bin/activate
uv pip install -e '.[develop,docs]'
```

On Windows, activate the virtual environment with the command for your shell.
The editable installation builds the Rust-backed Python extension through the
Hatchling `hatch-rs` hook configured in
[the Python project file](../../pyproject.toml). The `develop` extra supplies test
and build tools; `docs` supplies the documentation build dependencies.

Check that the extension imports and generates geometry:

```python
from turtles import ClousDeParisLayer

layer = ClousDeParisLayer(spacing=1.0, radius=6.0, resolution=2)
layer.generate()
assert layer.get_lines()
```

## Rebuild after changing native code

Python wrapper edits in [the package initializer](../../turtles/__init__.py) are
visible through the editable installation. After changing
[Rust implementation code](../../rust/src/lib.rs) or
[PyO3 bindings](../../rust/python/lib.rs), rebuild the installed extension:

```sh
uv pip install -e '.[develop,docs]'
```

Restart Python sessions that already imported the extension before checking the
change. Passing core Rust tests alone does not verify that the installed Python
extension includes your latest changes.

## Run Python and Rust tests

Run the Python suite against the rebuilt editable installation:

```sh
python -m pytest turtles/tests
```

The test configuration in [the Python project file](../../pyproject.toml) adds
verbose reporting and a JUnit report. For a narrower Python change, pass a test
path or pytest `-k` expression, then run the full suite before submitting it.

Run the core Rust unit tests and doctests without installing Cargo coverage
helpers:

```sh
cargo test --manifest-path rust/Cargo.toml
```

[The root Rust manifest](../../Cargo.toml) builds the `turtles_py` extension;
[the nested manifest](../../rust/Cargo.toml) defines the `turtles` library.
These are separate packages, not a Cargo workspace. A root `cargo test` or
`cargo test --workspace` is not a substitute for explicitly testing the nested
library crate.

If you use `make test` instead, account for its extra tooling: the Rust target
uses `cargo llvm-cov nextest`, so `cargo-llvm-cov` and `cargo-nextest` must already
be installed. The direct commands above are sufficient to run both language
suites without those coverage helpers.

## Check code style

For Python wrapper, example, or test changes:

```sh
make lint-py
```

For core Rust changes:

```sh
cargo fmt --manifest-path rust/Cargo.toml -- --check
cargo clippy --manifest-path rust/Cargo.toml --all-features
```

If you change bindings, check the root extension crate as well:

```sh
cargo fmt --manifest-path Cargo.toml -- --check
cargo check --manifest-path Cargo.toml
```

Rebuild the Python package and rerun the Python suite after a binding change;
Rust compilation does not check the Python wrapper's dispatch or public keyword
arguments.

## Build and check the documentation

Install both extras into the active environment if you skipped documentation
dependencies during setup:

```sh
python -m pip install -e '.[develop,docs]'
```

Build the site and run documentation formatting and spelling checks:

```sh
make docs
make lint-docs
```

The documentation build uses Yardang with warnings treated as errors. To run the
underlying build directly:

```sh
yardang build --warning-is-error
```

The docs dependencies include Yardang, MyST-aware Markdown formatting,
table formatting, and spelling checks. Use the repository's
[Makefile](../../Makefile) and [documentation configuration](../../pyproject.toml)
as the source of truth for the configured pages and lint scope. Open the HTML
output reported by Yardang and check navigation, code blocks, links, and images
on the pages you changed. A successful Python import does not check the rendered
documentation.

Keep how-to examples executable without hidden state: import dependencies,
construct and configure objects, call `generate()`, then export. Use relative
paths under the current working directory and pass filenames as strings rather
than `Path` objects. Do not make one fenced Python block depend on a previous
block; this also allows examples to be checked in isolated working directories.

To build the Rust API documentation separately:

```sh
cargo doc --manifest-path rust/Cargo.toml --no-deps
```

See [the Rust reference](../reference/rust.md) for the library's public entry
points. The Python wrapper exposes a different API; check
[composition](../reference/composition.md), [patterns](../reference/patterns.md),
and [rose-engine](../reference/rose-engine.md) when updating Python examples.

## Build a release library and a Python wheel

Use explicit build targets when you want artifacts without rerunning development
tool installation:

```sh
make build-rs
make build-py
```

`build-rs` delegates to the Rust directory's release build with all features.
`build-py` runs the Python wheel builder without build isolation, using the
Hatchling and `hatch-rs` dependencies already installed in the active environment.
`make build` runs both targets. Building a Rust release library alone does not
install or refresh the Python extension.

Before submitting a change, rerun the relevant tests against the rebuilt code,
check formatting, and build the documentation if public behavior or examples
changed. For export changes, inspect generated geometry as well as file creation:
the [export guide](export.md) lists the checks needed for curves, clipping, and
mesh limitations. Do not change package versions as part of an unrelated code or
documentation edit.
