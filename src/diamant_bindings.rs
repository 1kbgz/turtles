use pyo3::prelude::*;
use turtles::{
    DiamantConfig as BaseDiamantConfig,
    DiamantLayer as BaseDiamantLayer,
};

/// Python wrapper for DiamantLayer - creates diamond guilloché patterns
/// using circles tangent to the center, rotated around the center
#[pyclass]
pub struct DiamantLayer {
    pub inner: BaseDiamantLayer,
}

#[pymethods]
impl DiamantLayer {
    /// Create a new diamant layer centered at origin
    ///
    /// # Arguments
    /// * `num_circles` - Number of circles to draw (more = denser mesh)
    /// * `circle_radius` - Radius of each individual circle
    /// * `resolution` - Number of points per circle (default: 360)
    #[new]
    #[pyo3(signature = (num_circles, circle_radius, resolution=360))]
    fn new(num_circles: usize, circle_radius: f64, resolution: usize) -> PyResult<Self> {
        let config = BaseDiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };
        BaseDiamantLayer::new(config)
            .map(|inner| DiamantLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a diamant layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (num_circles, circle_radius, center_x, center_y, resolution=360))]
    fn with_center(
        num_circles: usize,
        circle_radius: f64,
        center_x: f64,
        center_y: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseDiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };
        BaseDiamantLayer::new_with_center(config, center_x, center_y)
            .map(|inner| DiamantLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a diamant layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (num_circles, circle_radius, angle, distance, resolution=360))]
    fn at_polar(
        num_circles: usize,
        circle_radius: f64,
        angle: f64,
        distance: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseDiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };
        BaseDiamantLayer::new_at_polar(config, angle, distance)
            .map(|inner| DiamantLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a diamant layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `num_circles` - Number of circles to draw
    /// * `circle_radius` - Radius of each individual circle
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the subdial center
    /// * `resolution` - Number of points per circle (default: 360)
    #[staticmethod]
    #[pyo3(signature = (num_circles, circle_radius, hour, minute, distance, resolution=360))]
    fn at_clock(
        num_circles: usize,
        circle_radius: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        resolution: usize,
    ) -> PyResult<Self> {
        let config = BaseDiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };
        BaseDiamantLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| DiamantLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the diamant pattern
    fn generate(&mut self) {
        self.inner.generate();
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

    /// Export the pattern to SVG format
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Get the number of circles in the pattern
    #[getter]
    fn num_circles(&self) -> usize {
        self.inner.config.num_circles
    }

    /// Get the circle radius
    #[getter]
    fn circle_radius(&self) -> f64 {
        self.inner.config.circle_radius
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
            "DiamantLayer(num_circles={}, circle_radius={}, center=({}, {}))",
            self.inner.config.num_circles,
            self.inner.config.circle_radius,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
