use pyo3::prelude::*;
use turtles::{
    GuillochePattern as BaseGuillochePattern,
    FlinqueConfig as BaseFlinqueConfig,
    FlinqueLayer as BaseFlinqueLayer,
    HorizontalSpirograph as BaseHorizontalSpirograph,
    VerticalSpirograph as BaseVerticalSpirograph,
    SphericalSpirograph as BaseSphericalSpirograph,
    ExportConfig as BaseExportConfig,
};

use crate::spirograph_bindings::{HorizontalSpirograph, VerticalSpirograph, SphericalSpirograph};

/// Python wrapper for FlinqueLayer - a radial sunburst engine-turned pattern
#[pyclass]
pub struct FlinqueLayer {
    pub inner: BaseFlinqueLayer,
}

#[pymethods]
impl FlinqueLayer {
    #[new]
    #[pyo3(signature = (radius, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn new(
        radius: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<Self> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        BaseFlinqueLayer::new(radius, config)
            .map(|inner| FlinqueLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a flinqué layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (radius, center_x, center_y, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn with_center(
        radius: f64,
        center_x: f64,
        center_y: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<Self> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        BaseFlinqueLayer::new_with_center(radius, config, center_x, center_y)
            .map(|inner| FlinqueLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a flinqué layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (radius, angle, distance, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn at_polar(
        radius: f64,
        angle: f64,
        distance: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<Self> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        BaseFlinqueLayer::new_at_polar(radius, config, angle, distance)
            .map(|inner| FlinqueLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a flinqué layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `radius` - Radius of the flinqué pattern
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the subdial center
    #[staticmethod]
    #[pyo3(signature = (radius, hour, minute, distance, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn at_clock(
        radius: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<Self> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        BaseFlinqueLayer::new_at_clock(radius, config, hour, minute, distance)
            .map(|inner| FlinqueLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[getter]
    fn radius(&self) -> f64 {
        self.inner.radius
    }

    #[getter]
    fn center_x(&self) -> f64 {
        self.inner.center_x
    }

    #[getter]
    fn center_y(&self) -> f64 {
        self.inner.center_y
    }

    fn __repr__(&self) -> String {
        format!(
            "FlinqueLayer(radius={}, center=({}, {}), petals={})",
            self.inner.radius,
            self.inner.center_x,
            self.inner.center_y,
            self.inner.config.num_petals
        )
    }
}

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

    /// Add a flinqué (engine-turned) layer to the pattern
    fn add_flinque_layer(&mut self, flinque: &FlinqueLayer) -> PyResult<()> {
        let new_layer = BaseFlinqueLayer::new_with_center(
            flinque.inner.radius,
            flinque.inner.config.clone(),
            flinque.inner.center_x,
            flinque.inner.center_y,
        ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        self.inner.add_flinque_layer(new_layer);
        Ok(())
    }

    /// Add a flinqué layer positioned at a given angle and distance from origin
    #[pyo3(signature = (radius, angle, distance, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn add_flinque_at_polar(
        &mut self,
        radius: f64,
        angle: f64,
        distance: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<()> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        self.inner.add_flinque_at_polar(radius, config, angle, distance)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Add a flinqué layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `radius` - Radius of the flinqué pattern (subdial size)
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the subdial center
    #[pyo3(signature = (radius, hour, minute, distance, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn add_flinque_at_clock(
        &mut self,
        radius: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<()> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        self.inner.add_flinque_at_clock(radius, config, hour, minute, distance)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
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
