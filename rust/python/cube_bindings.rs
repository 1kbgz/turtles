use pyo3::prelude::*;
use turtles::{
    CubeConfig as BaseCubeConfig,
    CubeLayer as BaseCubeLayer,
};

/// Python wrapper for CubeLayer - creates tumbling-blocks guilloché patterns
/// using parallel zigzag lines grouped in sets with phase-shifted gaps
#[pyclass]
pub struct CubeLayer {
    pub inner: BaseCubeLayer,
}

#[pymethods]
impl CubeLayer {
    /// Create a new cube layer centered at origin
    ///
    /// # Arguments
    /// * `spacing` - Distance between adjacent zigzag lines in mm
    /// * `radius` - Radius of the circular clipping region in mm
    /// * `angle` - Base rotation angle of the pattern in radians
    /// * `resolution` - Number of sample points per line
    /// * `cuts_per_group` - Number of zigzag lines per cutting group
    #[new]
    #[pyo3(signature = (spacing=0.5, radius=22.0, angle=0.0, resolution=200, cuts_per_group=8, gap_per_group=8, amplitude=0.0, leg_angle=30.0))]
    pub fn new(
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
        cuts_per_group: usize,
        gap_per_group: usize,
        amplitude: f64,
        leg_angle: f64,
    ) -> PyResult<Self> {
        let config = BaseCubeConfig {
            spacing,
            radius,
            angle,
            resolution,
            cuts_per_group,
            gap_per_group,
            amplitude,
            leg_angle,
        };
        BaseCubeLayer::new(config)
            .map(|inner| CubeLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a cube layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (center_x, center_y, spacing=0.5, radius=22.0, angle=0.0, resolution=200, cuts_per_group=8, gap_per_group=8, amplitude=0.0, leg_angle=30.0))]
    fn with_center(
        center_x: f64,
        center_y: f64,
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
        cuts_per_group: usize,
        gap_per_group: usize,
        amplitude: f64,
        leg_angle: f64,
    ) -> PyResult<Self> {
        let config = BaseCubeConfig {
            spacing,
            radius,
            angle,
            resolution,
            cuts_per_group,
            gap_per_group,
            amplitude,
            leg_angle,
        };
        BaseCubeLayer::new_with_center(config, center_x, center_y)
            .map(|inner| CubeLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a cube layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (angle, distance, spacing=0.5, radius=22.0, grid_angle=0.0, resolution=200, cuts_per_group=8, gap_per_group=8, amplitude=0.0, leg_angle=30.0))]
    fn at_polar(
        angle: f64,
        distance: f64,
        spacing: f64,
        radius: f64,
        grid_angle: f64,
        resolution: usize,
        cuts_per_group: usize,
        gap_per_group: usize,
        amplitude: f64,
        leg_angle: f64,
    ) -> PyResult<Self> {
        let config = BaseCubeConfig {
            spacing,
            radius,
            angle: grid_angle,
            resolution,
            cuts_per_group,
            gap_per_group,
            amplitude,
            leg_angle,
        };
        BaseCubeLayer::new_at_polar(config, angle, distance)
            .map(|inner| CubeLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a cube layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the layer center
    #[staticmethod]
    #[pyo3(signature = (hour, minute, distance, spacing=0.5, radius=22.0, angle=0.0, resolution=200, cuts_per_group=8, gap_per_group=8, amplitude=0.0, leg_angle=30.0))]
    fn at_clock(
        hour: u32,
        minute: u32,
        distance: f64,
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
        cuts_per_group: usize,
        gap_per_group: usize,
        amplitude: f64,
        leg_angle: f64,
    ) -> PyResult<Self> {
        let config = BaseCubeConfig {
            spacing,
            radius,
            angle,
            resolution,
            cuts_per_group,
            gap_per_group,
            amplitude,
            leg_angle,
        };
        BaseCubeLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| CubeLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the cube pattern
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

    /// Get the spacing between parallel lines
    #[getter]
    fn spacing(&self) -> f64 {
        self.inner.config.spacing
    }

    /// Get the radius
    #[getter]
    fn radius(&self) -> f64 {
        self.inner.config.radius
    }

    /// Get the base rotation angle
    #[getter]
    fn angle(&self) -> f64 {
        self.inner.config.angle
    }

    /// Get the resolution
    #[getter]
    fn resolution(&self) -> usize {
        self.inner.config.resolution
    }

    /// Get the number of zigzag lines per cutting group
    #[getter]
    fn cuts_per_group(&self) -> usize {
        self.inner.config.cuts_per_group
    }

    /// Get the number of line-spacings of empty gap between groups
    #[getter]
    fn gap_per_group(&self) -> usize {
        self.inner.config.gap_per_group
    }

    /// Get the zigzag amplitude
    #[getter]
    fn amplitude(&self) -> f64 {
        self.inner.config.amplitude
    }

    /// Get the leg angle in degrees
    #[getter]
    fn leg_angle(&self) -> f64 {
        self.inner.config.leg_angle
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
            "CubeLayer(spacing={}, radius={}, angle={}, center=({}, {}))",
            self.inner.config.spacing,
            self.inner.config.radius,
            self.inner.config.angle,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
