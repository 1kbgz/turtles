use pyo3::prelude::*;
use turtles::{
    ClousDeParisConfig as BaseClousDeParisConfig,
    ClousDeParisLayer as BaseClousDeParisLayer,
};

/// Python wrapper for ClousDeParisLayer - creates hobnail grid guilloché patterns
/// using two perpendicular sets of parallel lines clipped to a circle
#[pyclass]
pub struct ClousDeParisLayer {
    pub inner: BaseClousDeParisLayer,
}

#[pymethods]
impl ClousDeParisLayer {
    /// Create a new clous de Paris layer centered at origin
    ///
    /// # Arguments
    /// * `spacing` - Distance between parallel grooves in mm (controls hobnail size)
    /// * `radius` - Radius of the circular clipping region in mm
    /// * `angle` - Rotation angle of the grid in radians (default π/4 = 45°)
    /// * `resolution` - Number of sample points per line
    #[new]
    #[pyo3(signature = (spacing=1.0, radius=22.0, angle=std::f64::consts::FRAC_PI_4, resolution=200))]
    pub fn new(
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseClousDeParisConfig {
            spacing,
            radius,
            angle,
            resolution,
        };
        BaseClousDeParisLayer::new(config)
            .map(|inner| ClousDeParisLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a clous de Paris layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (center_x, center_y, spacing=1.0, radius=22.0, angle=std::f64::consts::FRAC_PI_4, resolution=200))]
    fn with_center(
        center_x: f64,
        center_y: f64,
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseClousDeParisConfig {
            spacing,
            radius,
            angle,
            resolution,
        };
        BaseClousDeParisLayer::new_with_center(config, center_x, center_y)
            .map(|inner| ClousDeParisLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a clous de Paris layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (angle, distance, spacing=1.0, radius=22.0, grid_angle=std::f64::consts::FRAC_PI_4, resolution=200))]
    fn at_polar(
        angle: f64,
        distance: f64,
        spacing: f64,
        radius: f64,
        grid_angle: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseClousDeParisConfig {
            spacing,
            radius,
            angle: grid_angle,
            resolution,
        };
        BaseClousDeParisLayer::new_at_polar(config, angle, distance)
            .map(|inner| ClousDeParisLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a clous de Paris layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the layer center
    #[staticmethod]
    #[pyo3(signature = (hour, minute, distance, spacing=1.0, radius=22.0, angle=std::f64::consts::FRAC_PI_4, resolution=200))]
    fn at_clock(
        hour: u32,
        minute: u32,
        distance: f64,
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseClousDeParisConfig {
            spacing,
            radius,
            angle,
            resolution,
        };
        BaseClousDeParisLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| ClousDeParisLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the clous de Paris pattern
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Export the pattern to SVG format
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Get all generated lines as list of list of (x, y) tuples
    fn get_lines(&self) -> Vec<Vec<(f64, f64)>> {
        self.inner
            .lines()
            .iter()
            .map(|line| line.iter().map(|p| (p.x, p.y)).collect())
            .collect()
    }

    /// Get the spacing between grooves
    #[getter]
    fn spacing(&self) -> f64 {
        self.inner.config.spacing
    }

    /// Get the radius
    #[getter]
    fn radius(&self) -> f64 {
        self.inner.config.radius
    }

    /// Get the grid rotation angle
    #[getter]
    fn angle(&self) -> f64 {
        self.inner.config.angle
    }

    /// Get the resolution
    #[getter]
    fn resolution(&self) -> usize {
        self.inner.config.resolution
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

    fn __repr__(&self) -> String {
        format!(
            "ClousDeParisLayer(spacing={}, radius={}, center=({}, {}))",
            self.inner.config.spacing,
            self.inner.config.radius,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
