use pyo3::prelude::*;
use turtles::{
    LimaconConfig as BaseLimaconConfig,
    LimaconLayer as BaseLimaconLayer,
};

/// Python wrapper for LimaconLayer - creates limaçon guilloché patterns
/// using the polar equation r = base_radius + amplitude * sin(θ + phase)
/// This produces identical output to a rose engine with sinusoidal frequency=1
#[pyclass]
pub struct LimaconLayer {
    pub inner: BaseLimaconLayer,
}

#[pymethods]
impl LimaconLayer {
    /// Create a new limaçon layer centered at origin
    ///
    /// # Arguments
    /// * `num_curves` - Number of limaçon curves to draw (more = denser mesh)
    /// * `base_radius` - Base radius (distance from center when sin=0)
    /// * `amplitude` - Amplitude of sinusoidal modulation
    /// * `resolution` - Number of points per curve (default: 360)
    #[new]
    #[pyo3(signature = (num_curves, base_radius, amplitude, resolution=360))]
    fn new(num_curves: usize, base_radius: f64, amplitude: f64, resolution: usize) -> PyResult<Self> {
        let config = BaseLimaconConfig {
            num_curves,
            base_radius,
            amplitude,
            resolution,
        };
        BaseLimaconLayer::new(config)
            .map(|inner| LimaconLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a limaçon layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (num_curves, base_radius, amplitude, center_x, center_y, resolution=360))]
    fn with_center(
        num_curves: usize,
        base_radius: f64,
        amplitude: f64,
        center_x: f64,
        center_y: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseLimaconConfig {
            num_curves,
            base_radius,
            amplitude,
            resolution,
        };
        BaseLimaconLayer::new_with_center(config, center_x, center_y)
            .map(|inner| LimaconLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a limaçon layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (num_curves, base_radius, amplitude, angle, distance, resolution=360))]
    fn at_polar(
        num_curves: usize,
        base_radius: f64,
        amplitude: f64,
        angle: f64,
        distance: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseLimaconConfig {
            num_curves,
            base_radius,
            amplitude,
            resolution,
        };
        BaseLimaconLayer::new_at_polar(config, angle, distance)
            .map(|inner| LimaconLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a limaçon layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `num_curves` - Number of limaçon curves to draw
    /// * `base_radius` - Base radius
    /// * `amplitude` - Amplitude of sinusoidal modulation
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the subdial center
    /// * `resolution` - Number of points per curve (default: 360)
    #[staticmethod]
    #[pyo3(signature = (num_curves, base_radius, amplitude, hour, minute, distance, resolution=360))]
    fn at_clock(
        num_curves: usize,
        base_radius: f64,
        amplitude: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseLimaconConfig {
            num_curves,
            base_radius,
            amplitude,
            resolution,
        };
        BaseLimaconLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| LimaconLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the limaçon pattern
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Export the pattern to SVG format
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Get the number of curves in the pattern
    #[getter]
    fn num_curves(&self) -> usize {
        self.inner.config.num_curves
    }

    /// Get the base radius
    #[getter]
    fn base_radius(&self) -> f64 {
        self.inner.config.base_radius
    }

    /// Get the amplitude
    #[getter]
    fn amplitude(&self) -> f64 {
        self.inner.config.amplitude
    }

    /// Get the center x coordinate
    #[getter]
    fn center_x(&self) -> f64 {
        self.inner.center_x
    }

    /// Get the center y coordinate
    #[getter]
    fn center_y(&self) -> f64 {
        self.inner.center_y
    }

    /// Get the generated pattern lines as a list of point lists
    /// Each line is a list of (x, y) tuples
    fn get_lines(&self) -> Vec<Vec<(f64, f64)>> {
        self.inner
            .lines()
            .iter()
            .map(|line| line.iter().map(|p| (p.x, p.y)).collect())
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "LimaconLayer(num_curves={}, base_radius={}, amplitude={}, center=({}, {}))",
            self.inner.config.num_curves,
            self.inner.config.base_radius,
            self.inner.config.amplitude,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
