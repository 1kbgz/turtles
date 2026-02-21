use pyo3::prelude::*;
use turtles::{
    DraperieConfig as BaseDraperieConfig,
    DraperieLayer as BaseDraperieLayer,
};

/// Python wrapper for DraperieLayer - creates flowing drapery guilloché patterns
/// using concentric wavy rings with sinusoidal phase oscillation
#[pyclass]
pub struct DraperieLayer {
    pub inner: BaseDraperieLayer,
}

#[pymethods]
impl DraperieLayer {
    /// Create a new draperie layer centered at origin
    ///
    /// # Arguments
    /// * `num_rings` - Number of concentric rings (more = denser)
    /// * `base_radius` - Centre of the ring band in mm
    /// * `radius_step` - Radial spacing between ring centres (default: 0.35)
    /// * `wave_frequency` - Number of wave undulations per revolution (default: 6.0)
    /// * `phase_shift` - Peak angular oscillation amplitude in radians (default: π/12 ≈ 15°)
    /// * `phase_oscillations` - Number of full sinusoidal phase cycles (default: 2.5)
    /// * `resolution` - Number of points per ring (default: 1500)
    /// * `phase_exponent` - Exponent for the phase envelope when circular_phase=0 (default: 3)
    /// * `wave_exponent` - Exponent for the wave shape (default: 1 = sinusoidal, 3 = softer crests)
    /// * `circular_phase` - Dome-shaped phase exponent; 0 disables (uses sin^e), 2.0 = rounded folds (default: 2.0)
    #[new]
    #[pyo3(signature = (num_rings=96, base_radius=22.0, radius_step=0.44, wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5, resolution=1500, phase_exponent=3, wave_exponent=1, circular_phase=2.0))]
    pub fn new(
        num_rings: usize,
        base_radius: f64,
        radius_step: f64,
        wave_frequency: f64,
        phase_shift: Option<f64>,
        phase_oscillations: f64,
        resolution: usize,
        phase_exponent: u32,
        wave_exponent: u32,
        circular_phase: f64,
    ) -> PyResult<Self> {
        let config = BaseDraperieConfig {
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            amplitude: None,
            phase_shift: phase_shift.unwrap_or(std::f64::consts::PI / 12.0),
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
        };
        BaseDraperieLayer::new(config)
            .map(|inner| DraperieLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a draperie layer with a custom center point
    #[staticmethod]
    #[pyo3(signature = (center_x, center_y, num_rings=96, base_radius=22.0, radius_step=0.44, wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5, resolution=1500, phase_exponent=3, wave_exponent=1, circular_phase=2.0))]
    fn with_center(
        center_x: f64,
        center_y: f64,
        num_rings: usize,
        base_radius: f64,
        radius_step: f64,
        wave_frequency: f64,
        phase_shift: Option<f64>,
        phase_oscillations: f64,
        resolution: usize,
        phase_exponent: u32,
        wave_exponent: u32,
        circular_phase: f64,
    ) -> PyResult<Self> {
        let config = BaseDraperieConfig {
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            amplitude: None,
            phase_shift: phase_shift.unwrap_or(std::f64::consts::PI / 12.0),
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
        };
        BaseDraperieLayer::new_with_center(config, center_x, center_y)
            .map(|inner| DraperieLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a draperie layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (angle, distance, num_rings=96, base_radius=22.0, radius_step=0.44, wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5, resolution=1500, phase_exponent=3, wave_exponent=1, circular_phase=2.0))]
    fn at_polar(
        angle: f64,
        distance: f64,
        num_rings: usize,
        base_radius: f64,
        radius_step: f64,
        wave_frequency: f64,
        phase_shift: Option<f64>,
        phase_oscillations: f64,
        resolution: usize,
        phase_exponent: u32,
        wave_exponent: u32,
        circular_phase: f64,
    ) -> PyResult<Self> {
        let config = BaseDraperieConfig {
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            amplitude: None,
            phase_shift: phase_shift.unwrap_or(std::f64::consts::PI / 12.0),
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
        };
        BaseDraperieLayer::new_at_polar(config, angle, distance)
            .map(|inner| DraperieLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a draperie layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face to the layer center
    #[staticmethod]
    #[pyo3(signature = (hour, minute, distance, num_rings=96, base_radius=22.0, radius_step=0.44, wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5, resolution=1500, phase_exponent=3, wave_exponent=1, circular_phase=2.0))]
    fn at_clock(
        hour: u32,
        minute: u32,
        distance: f64,
        num_rings: usize,
        base_radius: f64,
        radius_step: f64,
        wave_frequency: f64,
        phase_shift: Option<f64>,
        phase_oscillations: f64,
        resolution: usize,
        phase_exponent: u32,
        wave_exponent: u32,
        circular_phase: f64,
    ) -> PyResult<Self> {
        let config = BaseDraperieConfig {
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            amplitude: None,
            phase_shift: phase_shift.unwrap_or(std::f64::consts::PI / 12.0),
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
        };
        BaseDraperieLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| DraperieLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the draperie pattern
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Export the pattern to SVG format
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Get all generated ring lines as list of list of (x, y) tuples
    fn get_lines(&self) -> Vec<Vec<(f64, f64)>> {
        self.inner
            .lines()
            .iter()
            .map(|ring| ring.iter().map(|p| (p.x, p.y)).collect())
            .collect()
    }

    /// Get the number of rings in the pattern
    #[getter]
    fn num_rings(&self) -> usize {
        self.inner.config.num_rings
    }

    /// Get the base radius
    #[getter]
    fn base_radius(&self) -> f64 {
        self.inner.config.base_radius
    }

    /// Get the radius step
    #[getter]
    fn radius_step(&self) -> f64 {
        self.inner.config.radius_step
    }

    /// Get the wave frequency
    #[getter]
    fn wave_frequency(&self) -> f64 {
        self.inner.config.wave_frequency
    }

    /// Get the phase exponent
    #[getter]
    fn phase_exponent(&self) -> u32 {
        self.inner.config.phase_exponent
    }

    /// Get the wave exponent
    #[getter]
    fn wave_exponent(&self) -> u32 {
        self.inner.config.wave_exponent
    }

    /// Get the circular_phase dome exponent
    #[getter]
    fn circular_phase(&self) -> f64 {
        self.inner.config.circular_phase
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
            "DraperieLayer(num_rings={}, base_radius={}, center=({}, {}))",
            self.inner.config.num_rings,
            self.inner.config.base_radius,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
