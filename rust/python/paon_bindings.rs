use pyo3::prelude::*;
use turtles::{
    PaonConfig as BasePaonConfig,
    PaonLayer as BasePaonLayer,
};

/// Python wrapper for PaonLayer - creates peacock-feather guilloché patterns
/// using a fan of lines emanating from 6 o'clock with zigzag oscillation
#[pyclass]
pub struct PaonLayer {
    pub inner: BasePaonLayer,
}

#[pymethods]
impl PaonLayer {
    /// Create a new paon layer centered at origin
    ///
    /// # Arguments
    /// * `num_lines` - Number of fan lines (more = denser pattern)
    /// * `radius` - Radius of the circular dial in mm
    /// * `amplitude` - Perpendicular oscillation amplitude in mm
    /// * `wave_frequency` - Number of zigzag cycles per line
    /// * `phase_rate` - Phase change rate across fan (controls arch band count)
    /// * `resolution` - Number of sample points per line
    /// * `n_harmonics` - 0=sine, 1+=triangle-wave (sharper cusps)
    /// * `fan_angle` - Total angular spread in radians (~2.618 = 150°)
    #[new]
    #[pyo3(signature = (num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0, phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0, vanishing_point=0.3))]
    pub fn new(
        num_lines: usize,
        radius: f64,
        amplitude: f64,
        wave_frequency: f64,
        phase_rate: f64,
        resolution: usize,
        n_harmonics: usize,
        fan_angle: f64,
        vanishing_point: f64,
    ) -> PyResult<Self> {
        let config = BasePaonConfig {
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point,
        };
        BasePaonLayer::new(config)
            .map(|inner| PaonLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a paon layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (center_x, center_y, num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0, phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0, vanishing_point=0.3))]
    fn with_center(
        center_x: f64,
        center_y: f64,
        num_lines: usize,
        radius: f64,
        amplitude: f64,
        wave_frequency: f64,
        phase_rate: f64,
        resolution: usize,
        n_harmonics: usize,
        fan_angle: f64,
        vanishing_point: f64,
    ) -> PyResult<Self> {
        let config = BasePaonConfig {
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point,
        };
        BasePaonLayer::new_with_center(config, center_x, center_y)
            .map(|inner| PaonLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a paon layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (angle, distance, num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0, phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0, vanishing_point=0.3))]
    fn at_polar(
        angle: f64,
        distance: f64,
        num_lines: usize,
        radius: f64,
        amplitude: f64,
        wave_frequency: f64,
        phase_rate: f64,
        resolution: usize,
        n_harmonics: usize,
        fan_angle: f64,
        vanishing_point: f64,
    ) -> PyResult<Self> {
        let config = BasePaonConfig {
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point,
        };
        BasePaonLayer::new_at_polar(config, angle, distance)
            .map(|inner| PaonLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a paon layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the layer center
    #[staticmethod]
    #[pyo3(signature = (hour, minute, distance, num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0, phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0, vanishing_point=0.3))]
    fn at_clock(
        hour: u32,
        minute: u32,
        distance: f64,
        num_lines: usize,
        radius: f64,
        amplitude: f64,
        wave_frequency: f64,
        phase_rate: f64,
        resolution: usize,
        n_harmonics: usize,
        fan_angle: f64,
        vanishing_point: f64,
    ) -> PyResult<Self> {
        let config = BasePaonConfig {
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point,
        };
        BasePaonLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| PaonLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the paon pattern
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

    /// Get the number of lines in the pattern
    #[getter]
    fn num_lines(&self) -> usize {
        self.inner.config.num_lines
    }

    /// Get the radius
    #[getter]
    fn radius(&self) -> f64 {
        self.inner.config.radius
    }

    /// Get the amplitude
    #[getter]
    fn amplitude(&self) -> f64 {
        self.inner.config.amplitude
    }

    /// Get the wave frequency
    #[getter]
    fn wave_frequency(&self) -> f64 {
        self.inner.config.wave_frequency
    }

    /// Get the phase rate
    #[getter]
    fn phase_rate(&self) -> f64 {
        self.inner.config.phase_rate
    }

    /// Get the number of Fourier harmonics
    #[getter]
    fn n_harmonics(&self) -> usize {
        self.inner.config.n_harmonics
    }

    /// Get the fan angle
    #[getter]
    fn fan_angle(&self) -> f64 {
        self.inner.config.fan_angle
    }

    /// Get the vanishing point distance
    #[getter]
    fn vanishing_point(&self) -> f64 {
        self.inner.config.vanishing_point
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
            "PaonLayer(num_lines={}, radius={}, center=({}, {}))",
            self.inner.config.num_lines,
            self.inner.config.radius,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
