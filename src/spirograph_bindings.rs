use pyo3::prelude::*;
use turtles::{
    HorizontalSpirograph as BaseHorizontalSpirograph,
    VerticalSpirograph as BaseVerticalSpirograph,
    SphericalSpirograph as BaseSphericalSpirograph,
    ExportConfig as BaseExportConfig,
};

/// Python wrapper for HorizontalSpirograph
#[pyclass]
pub struct HorizontalSpirograph {
    pub(crate) inner: BaseHorizontalSpirograph,
}

#[pymethods]
impl HorizontalSpirograph {
    #[new]
    #[pyo3(signature = (outer_radius, radius_ratio, point_distance, rotations, resolution))]
    fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
    ) -> PyResult<Self> {
        BaseHorizontalSpirograph::new(outer_radius, radius_ratio, point_distance, rotations, resolution)
            .map(|inner| HorizontalSpirograph { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
    
    /// Generate the spirograph pattern points
    fn generate(&mut self) -> PyResult<()> {
        self.inner.generate();
        Ok(())
    }
    
    /// Export pattern as SVG
    #[pyo3(signature = (filename))]
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    /// Export pattern as STEP file
    #[pyo3(signature = (filename, depth=0.1))]
    fn to_step(&self, filename: &str, depth: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness: 2.0,
            tool_radius: 0.0,
        };
        self.inner.to_step(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    /// Export pattern as STL file
    #[pyo3(signature = (filename, depth=0.1, base_thickness=2.0))]
    fn to_stl(&self, filename: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner.to_stl(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "HorizontalSpirograph(outer_radius={}, radius_ratio={}, point_distance={}, rotations={}, resolution={})",
            self.inner.outer_radius,
            self.inner.radius_ratio,
            self.inner.point_distance,
            self.inner.rotations,
            self.inner.resolution
        ))
    }
}

/// Python wrapper for VerticalSpirograph
#[pyclass]
pub struct VerticalSpirograph {
    pub(crate) inner: BaseVerticalSpirograph,
}

#[pymethods]
impl VerticalSpirograph {
    #[new]
    #[pyo3(signature = (outer_radius, radius_ratio, point_distance, rotations, resolution, wave_amplitude=1.0, wave_frequency=5.0))]
    fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
    ) -> PyResult<Self> {
        BaseVerticalSpirograph::new(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            wave_amplitude,
            wave_frequency,
        )
        .map(|inner| VerticalSpirograph { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
    
    fn generate(&mut self) -> PyResult<()> {
        self.inner.generate();
        Ok(())
    }
    
    #[pyo3(signature = (filename))]
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    #[pyo3(signature = (filename, depth=0.1))]
    fn to_step(&self, filename: &str, depth: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness: 2.0,
            tool_radius: 0.0,
        };
        self.inner.to_step(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    #[pyo3(signature = (filename, depth=0.1, base_thickness=2.0))]
    fn to_stl(&self, filename: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner.to_stl(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "VerticalSpirograph(outer_radius={}, radius_ratio={}, point_distance={}, rotations={}, resolution={}, wave_amplitude={}, wave_frequency={})",
            self.inner.outer_radius,
            self.inner.radius_ratio,
            self.inner.point_distance,
            self.inner.rotations,
            self.inner.resolution,
            self.inner.wave_amplitude,
            self.inner.wave_frequency
        ))
    }
}

/// Python wrapper for SphericalSpirograph
#[pyclass]
pub struct SphericalSpirograph {
    pub(crate) inner: BaseSphericalSpirograph,
}

#[pymethods]
impl SphericalSpirograph {
    #[new]
    #[pyo3(signature = (outer_radius, radius_ratio, point_distance, rotations, resolution, dome_height=5.0))]
    fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        dome_height: f64,
    ) -> PyResult<Self> {
        BaseSphericalSpirograph::new(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            dome_height,
        )
        .map(|inner| SphericalSpirograph { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
    
    fn generate(&mut self) -> PyResult<()> {
        self.inner.generate();
        Ok(())
    }
    
    #[pyo3(signature = (filename))]
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    #[pyo3(signature = (filename, depth=0.1))]
    fn to_step(&self, filename: &str, depth: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness: 2.0,
            tool_radius: 0.0,
        };
        self.inner.to_step(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    #[pyo3(signature = (filename, depth=0.1, base_thickness=2.0))]
    fn to_stl(&self, filename: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner.to_stl(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }
    
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "SphericalSpirograph(outer_radius={}, radius_ratio={}, point_distance={}, rotations={}, resolution={}, dome_height={})",
            self.inner.outer_radius,
            self.inner.radius_ratio,
            self.inner.point_distance,
            self.inner.rotations,
            self.inner.resolution,
            self.inner.dome_height
        ))
    }
}
