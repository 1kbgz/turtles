use pyo3::prelude::*;
use turtles::{
    GuillochePattern as BaseGuillochePattern,
    HorizontalSpirograph as BaseHorizontalSpirograph,
    VerticalSpirograph as BaseVerticalSpirograph,
    SphericalSpirograph as BaseSphericalSpirograph,
    ExportConfig as BaseExportConfig,
};

use crate::spirograph_bindings::{HorizontalSpirograph, VerticalSpirograph, SphericalSpirograph};

/// Python wrapper for GuillochePattern
#[pyclass]
pub struct GuillochePattern {
    inner: BaseGuillochePattern,
}

#[pymethods]
impl GuillochePattern {
    #[new]
    #[pyo3(signature = (radius))]
    fn new(radius: f64) -> PyResult<Self> {
        BaseGuillochePattern::new(radius)
            .map(|inner| GuillochePattern { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    // Add attr access of radius
    #[getter]
    fn radius(&self) -> PyResult<f64> {
        Ok(self.inner.radius)
    }

    /// Add a horizontal spirograph layer
    fn add_layer(&mut self, spiro: &Bound<'_, PyAny>) -> PyResult<()> {
        // Try to extract different spirograph types
        if let Ok(h_spiro) = spiro.extract::<PyRef<HorizontalSpirograph>>() {
            // We need to create a new instance since we can't clone or move the inner value
            let new_spiro = BaseHorizontalSpirograph::new(
                h_spiro.inner.outer_radius,
                h_spiro.inner.radius_ratio,
                h_spiro.inner.point_distance,
                h_spiro.inner.rotations,
                h_spiro.inner.resolution,
            ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_horizontal_layer(new_spiro);
            return Ok(());
        }

        if let Ok(v_spiro) = spiro.extract::<PyRef<VerticalSpirograph>>() {
            let new_spiro = BaseVerticalSpirograph::new(
                v_spiro.inner.outer_radius,
                v_spiro.inner.radius_ratio,
                v_spiro.inner.point_distance,
                v_spiro.inner.rotations,
                v_spiro.inner.resolution,
                v_spiro.inner.wave_amplitude,
                v_spiro.inner.wave_frequency,
            ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_vertical_layer(new_spiro);
            return Ok(());
        }

        if let Ok(s_spiro) = spiro.extract::<PyRef<SphericalSpirograph>>() {
            let new_spiro = BaseSphericalSpirograph::new(
                s_spiro.inner.outer_radius,
                s_spiro.inner.radius_ratio,
                s_spiro.inner.point_distance,
                s_spiro.inner.rotations,
                s_spiro.inner.resolution,
                s_spiro.inner.dome_height,
            ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_spherical_layer(new_spiro);
            return Ok(());
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected HorizontalSpirograph, VerticalSpirograph, or SphericalSpirograph"
        ))
    }

    /// Generate all layers
    fn generate(&mut self) -> PyResult<()> {
        self.inner.generate();
        Ok(())
    }

    /// Export all layers to files
    #[pyo3(signature = (base_name, depth=0.1, base_thickness=2.0))]
    fn export_all(&self, base_name: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner.export_all(base_name, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Export svg only
    #[pyo3(signature = (filename))]
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.export_combined_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Export step only
    #[pyo3(signature = (filename, depth=0.1))]
    fn to_step(&self, filename: &str, depth: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness: 2.0,
            tool_radius: 0.0,
        };
        self.inner.export_combined_step(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Export stl only
    #[pyo3(signature = (filename, depth=0.1, base_thickness=2.0))]
    fn to_stl(&self, filename: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner.export_combined_stl(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "GuillochePattern(radius={}, layers={})",
            self.inner.radius,
            self.inner.layer_count()
        ))
    }
}
