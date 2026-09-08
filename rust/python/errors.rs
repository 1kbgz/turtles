//! Central mapping from [`SpirographError`] to Python exception types.
//!
//! The bindings used to pick an exception type per call site, so the same
//! error variant surfaced as `ValueError` in one method and `OSError` in
//! another. Worst of all, methods that both validate parameters *and* write
//! files wrapped everything in `PyIOError`, so a rejected parameter arrived in
//! Python as an `OSError`. Routing every conversion through here keeps the
//! variant-to-exception mapping in one place.

use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::PyErr;
use turtles::SpirographError;

/// Convert a [`SpirographError`] into the appropriate Python exception.
///
/// * [`SpirographError::InvalidRadius`] and [`SpirographError::InvalidParameter`]
///   are caller mistakes about *values*, so they map to `ValueError`.
/// * [`SpirographError::ExportError`] covers filesystem and serialization
///   failures, so it maps to `OSError` (`PyIOError` is an alias of `OSError`
///   in Python 3).
pub fn to_py_err(err: SpirographError) -> PyErr {
    match err {
        SpirographError::InvalidRadius(_) | SpirographError::InvalidParameter(_) => {
            PyValueError::new_err(err.to_string())
        }
        SpirographError::ExportError(_) => PyIOError::new_err(err.to_string()),
    }
}
