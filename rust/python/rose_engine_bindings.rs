use pyo3::prelude::*;
use turtles::{
    RoseEngineLathe as BaseRoseEngineLathe,
    RoseEngineLatheRun as BaseRoseEngineLatheRun,
    RoseEngineConfig as BaseRoseEngineConfig,
    CuttingBit as BaseCuttingBit,
    RosettePattern as BaseRosettePattern,
    ExportConfig as BaseExportConfig,
};

/// Python wrapper for RosettePattern
#[pyclass]
#[derive(Clone)]
pub struct RosettePattern {
    pub(crate) inner: BaseRosettePattern,
}

#[pymethods]
impl RosettePattern {
    /// Create a circular pattern (no modulation)
    #[staticmethod]
    fn circular() -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Circular,
        }
    }

    /// Create an elliptical pattern
    #[staticmethod]
    #[pyo3(signature = (eccentricity, rotation=0.0))]
    fn elliptical(eccentricity: f64, rotation: f64) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Elliptical { eccentricity, rotation },
        }
    }

    /// Create a sinusoidal wave pattern
    #[staticmethod]
    fn sinusoidal(frequency: f64) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Sinusoidal { frequency },
        }
    }

    /// Create a multi-lobe rosette pattern
    #[staticmethod]
    fn multi_lobe(lobes: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::MultiLobe { lobes },
        }
    }

    /// Create an epicycloid/rose curve pattern
    #[staticmethod]
    fn epicycloid(petals: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Epicycloid { petals },
        }
    }

    /// Create a Huit-Eight (Figure-Eight) pattern
    #[staticmethod]
    fn huit_eight(lobes: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::HuitEight { lobes },
        }
    }

    /// Create a Grain-de-Riz (Rice Grain) pattern
    #[staticmethod]
    fn grain_de_riz(grain_size: f64, rows: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::GrainDeRiz { grain_size, rows },
        }
    }

    /// Create a Draperie (Drapery) pattern
    #[staticmethod]
    #[pyo3(signature = (frequency, wave_exponent=1))]
    fn draperie(frequency: f64, wave_exponent: u32) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Draperie { frequency, wave_exponent },
        }
    }

    /// Create a Paon (Peacock) pattern
    #[staticmethod]
    fn paon(frequency: f64) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Paon { frequency },
        }
    }

    /// Create a Diamant (Diamond) pattern
    #[staticmethod]
    fn diamant(divisions: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Diamant { divisions },
        }
    }

    fn __repr__(&self) -> String {
        match &self.inner {
            BaseRosettePattern::Circular => "RosettePattern.circular()".to_string(),
            BaseRosettePattern::Elliptical { eccentricity, rotation } => {
                format!("RosettePattern.elliptical(eccentricity={}, rotation={})", eccentricity, rotation)
            }
            BaseRosettePattern::Sinusoidal { frequency } => {
                format!("RosettePattern.sinusoidal(frequency={})", frequency)
            }
            BaseRosettePattern::MultiLobe { lobes } => {
                format!("RosettePattern.multi_lobe(lobes={})", lobes)
            }
            BaseRosettePattern::Epicycloid { petals } => {
                format!("RosettePattern.epicycloid(petals={})", petals)
            }
            BaseRosettePattern::HuitEight { lobes } => {
                format!("RosettePattern.huit_eight(lobes={})", lobes)
            }
            BaseRosettePattern::GrainDeRiz { grain_size, rows } => {
                format!("RosettePattern.grain_de_riz(grain_size={}, rows={})", grain_size, rows)
            }
            BaseRosettePattern::Draperie { frequency, wave_exponent } => {
                format!("RosettePattern.draperie(frequency={}, wave_exponent={})", frequency, wave_exponent)
            }
            BaseRosettePattern::Paon { frequency } => {
                format!("RosettePattern.paon(frequency={})", frequency)
            }
            BaseRosettePattern::Diamant { divisions } => {
                format!("RosettePattern.diamant(divisions={})", divisions)
            }
            BaseRosettePattern::Custom { samples, .. } => {
                format!("RosettePattern.custom(samples={})", samples)
            }
        }
    }
}

/// Python wrapper for CuttingBit
#[pyclass]
#[derive(Clone)]
pub struct CuttingBit {
    pub(crate) inner: BaseCuttingBit,
}

#[pymethods]
impl CuttingBit {
    /// Create a V-shaped cutting bit
    #[staticmethod]
    fn v_shaped(angle: f64, width: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::v_shaped(angle, width),
        }
    }

    /// Create a flat cutting bit
    #[staticmethod]
    fn flat(width: f64, depth: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::flat(width, depth),
        }
    }

    /// Create a round/ball cutting bit
    #[staticmethod]
    fn round(diameter: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::round(diameter),
        }
    }

    /// Create an elliptical cutting bit
    #[staticmethod]
    fn elliptical(width: f64, aspect_ratio: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::elliptical(width, aspect_ratio),
        }
    }

    #[getter]
    fn width(&self) -> f64 {
        self.inner.width
    }

    #[getter]
    fn depth(&self) -> f64 {
        self.inner.depth
    }

    fn __repr__(&self) -> String {
        format!(
            "CuttingBit(width={}, depth={})",
            self.inner.width,
            self.inner.depth
        )
    }
}

/// Python wrapper for RoseEngineConfig
#[pyclass]
pub struct RoseEngineConfig {
    pub(crate) inner: BaseRoseEngineConfig,
}

#[pymethods]
impl RoseEngineConfig {
    /// Create a new rose engine configuration
    #[new]
    #[pyo3(signature = (base_radius, amplitude))]
    fn new(base_radius: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::new(base_radius, amplitude),
        }
    }

    /// Set the rosette pattern
    fn set_rosette(&mut self, pattern: RosettePattern) {
        self.inner.rosette = pattern.inner;
    }

    /// Set the resolution (number of points)
    fn set_resolution(&mut self, resolution: usize) {
        self.inner.resolution = resolution;
    }

    /// Add a secondary rosette for compound motion
    fn with_secondary_rosette(&mut self, rosette: RosettePattern, amplitude: f64) {
        self.inner.with_secondary_rosette(rosette.inner, amplitude);
    }

    /// Enable depth modulation
    fn with_depth_modulation(&mut self, amplitude: f64, frequency: f64) {
        self.inner.with_depth_modulation(amplitude, frequency);
    }

    /// Classic multi-lobe pattern preset
    #[staticmethod]
    fn classic_multi_lobe(base_radius: f64, lobes: usize, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::classic_multi_lobe(base_radius, lobes, amplitude),
        }
    }

    /// Sunburst pattern preset
    #[staticmethod]
    fn sunburst(base_radius: f64, rays: usize, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::sunburst(base_radius, rays, amplitude),
        }
    }

    /// Wave pattern preset
    #[staticmethod]
    fn wave(base_radius: f64, frequency: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::wave(base_radius, frequency, amplitude),
        }
    }

    /// Rose curve pattern preset
    #[staticmethod]
    fn rose_curve(base_radius: f64, petals: usize, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::rose_curve(base_radius, petals, amplitude),
        }
    }

    /// Compound pattern preset
    #[staticmethod]
    fn compound(
        base_radius: f64,
        primary_lobes: usize,
        primary_amplitude: f64,
        secondary_frequency: f64,
        secondary_amplitude: f64,
    ) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::compound(
                base_radius,
                primary_lobes,
                primary_amplitude,
                secondary_frequency,
                secondary_amplitude,
            ),
        }
    }

    /// Huit-Eight (Figure-Eight) pattern preset
    #[staticmethod]
    fn huit_eight(base_radius: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::huit_eight(base_radius, amplitude),
        }
    }

    /// Grain-de-Riz (Rice Grain) pattern preset
    #[staticmethod]
    fn grain_de_riz(base_radius: f64, grain_size: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::grain_de_riz(base_radius, grain_size, amplitude),
        }
    }

    /// Draperie (Drapery) pattern preset
    #[staticmethod]
    fn draperie(base_radius: f64, wave_frequency: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::draperie(base_radius, wave_frequency, amplitude),
        }
    }

    /// Diamant (Diamond) pattern preset
    #[staticmethod]
    fn diamant(base_radius: f64, divisions: usize, amplitude: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::diamant(base_radius, divisions, amplitude),
        }
    }

    #[getter]
    fn base_radius(&self) -> f64 {
        self.inner.base_radius
    }

    #[getter]
    fn amplitude(&self) -> f64 {
        self.inner.amplitude
    }

    #[getter]
    fn resolution(&self) -> usize {
        self.inner.resolution
    }

    fn __repr__(&self) -> String {
        format!(
            "RoseEngineConfig(base_radius={}, amplitude={}, resolution={})",
            self.inner.base_radius,
            self.inner.amplitude,
            self.inner.resolution
        )
    }
}

/// Python wrapper for RoseEngineLathe
#[pyclass]
pub struct RoseEngineLathe {
    pub(crate) inner: BaseRoseEngineLathe,
}

#[pymethods]
impl RoseEngineLathe {
    /// Create a new rose engine lathe
    #[new]
    fn new(config: PyRef<RoseEngineConfig>, bit: PyRef<CuttingBit>) -> PyResult<Self> {
        BaseRoseEngineLathe::new(config.inner.clone(), bit.inner.clone())
            .map(|inner| RoseEngineLathe { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine lathe with custom center position
    #[staticmethod]
    fn with_center(
        config: PyRef<RoseEngineConfig>,
        bit: PyRef<CuttingBit>,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLathe::new_with_center(
            config.inner.clone(),
            bit.inner.clone(),
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLathe { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the rose engine pattern
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Export pattern as SVG
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.to_svg(filename)
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

    fn __repr__(&self) -> String {
        format!(
            "RoseEngineLathe(center=({}, {}), base_radius={})",
            self.inner.center_x,
            self.inner.center_y,
            self.inner.config.base_radius
        )
    }
}

/// Python wrapper for RoseEngineLatheRun - multi-pass guilloché pattern generator
#[pyclass]
pub struct RoseEngineLatheRun {
    pub(crate) inner: BaseRoseEngineLatheRun,
}

#[pymethods]
impl RoseEngineLatheRun {
    /// Create a new multi-pass rose engine lathe run
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration
    /// * `bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes (typically 8-24)
    /// * `segments_per_pass` - Number of arc segments per pass (default 24, creates gaps)
    ///
    /// # Example
    /// ```python
    /// from turtles import RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern
    ///
    /// config = RoseEngineConfig(base_radius=20.0, amplitude=0.5)
    /// config.set_rosette(RosettePattern.multi_lobe(12))
    /// bit = CuttingBit.v_shaped(angle=30.0, width=0.5)
    ///
    /// run = RoseEngineLatheRun(config, bit, num_passes=12, segments_per_pass=24)
    /// run.generate()
    /// run.to_svg("pattern.svg")
    /// ```
    #[new]
    #[pyo3(signature = (config, bit, num_passes, segments_per_pass=24, radius_step=0.0, phase_shift=0.0, phase_oscillations=1.0, circular_phase=0.0, phase_exponent=1))]
    fn new(
        config: PyRef<RoseEngineConfig>,
        bit: PyRef<CuttingBit>,
        num_passes: usize,
        segments_per_pass: usize,
        radius_step: f64,
        phase_shift: f64,
        phase_oscillations: f64,
        circular_phase: f64,
        phase_exponent: u32,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_with_segments(
            config.inner.clone(),
            bit.inner.clone(),
            num_passes,
            segments_per_pass,
            0.0,
            0.0,
        )
        .map(|mut inner| {
            inner.radius_step = radius_step;
            inner.phase_shift = phase_shift;
            inner.phase_oscillations = phase_oscillations;
            inner.circular_phase = circular_phase;
            inner.phase_exponent = phase_exponent;
            RoseEngineLatheRun { inner }
        })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a multi-pass rose engine lathe run with custom center position
    #[staticmethod]
    #[pyo3(signature = (config, bit, num_passes, segments_per_pass=24, center_x=0.0, center_y=0.0))]
    fn with_center(
        config: PyRef<RoseEngineConfig>,
        bit: PyRef<CuttingBit>,
        num_passes: usize,
        segments_per_pass: usize,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_with_segments(
            config.inner.clone(),
            bit.inner.clone(),
            num_passes,
            segments_per_pass,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine draperie pattern that produces identical output
    /// to the mathematical DraperieLayer.
    ///
    /// This configures the rose engine lathe run with the correct rosette
    /// pattern, amplitude, phase alignment, and phase shape function.
    #[staticmethod]
    #[pyo3(signature = (num_rings=96, base_radius=22.0, radius_step=0.44, wave_frequency=12.0, phase_shift=None, phase_oscillations=2.5, resolution=1500, phase_exponent=3, wave_exponent=1, circular_phase=2.0, center_x=0.0, center_y=0.0))]
    fn draperie(
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
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        let ps = phase_shift.unwrap_or(std::f64::consts::PI / 12.0);
        BaseRoseEngineLatheRun::new_draperie(
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            ps,
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine paon (peacock) pattern that produces identical
    /// output to the mathematical PaonLayer.
    ///
    /// This configures the rose engine lathe run in linear-pass mode with
    /// fan lines emanating from 6 o'clock and zigzag oscillation.
    #[staticmethod]
    #[pyo3(signature = (num_lines=500, radius=22.0, amplitude=0.035, wave_frequency=10.0, phase_rate=9.0, resolution=800, n_harmonics=3, fan_angle=4.0, vanishing_point=0.3, center_x=0.0, center_y=0.0))]
    fn paon(
        num_lines: usize,
        radius: f64,
        amplitude: f64,
        wave_frequency: f64,
        phase_rate: f64,
        resolution: usize,
        n_harmonics: usize,
        fan_angle: f64,
        vanishing_point: f64,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_paon(
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine diamant (diamond) pattern that produces identical
    /// output to the mathematical DiamantLayer.
    ///
    /// Models a physical rose engine with a round eccentric cam (sinusoidal
    /// rosette, frequency 1) whose eccentricity equals the circle radius.
    /// Each pass traces a circle of radius `circle_radius` tangent to the
    /// centre.  Multiple passes at different angular positions create the
    /// characteristic diamond mesh.
    #[staticmethod]
    #[pyo3(signature = (num_circles=72, circle_radius=20.0, resolution=360, center_x=0.0, center_y=0.0))]
    fn diamant(
        num_circles: usize,
        circle_radius: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_diamant(
            num_circles,
            circle_radius,
            resolution,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine limaçon pattern that produces identical output
    /// to the mathematical LimaconLayer.
    ///
    /// Models a physical rose engine with a round eccentric rosette
    /// (sinusoidal cam, frequency 1).  Each pass traces the polar curve
    /// r = base_radius + amplitude · sin(θ + phase).  Multiple passes at
    /// different phase offsets create the overlapping limaçon mesh.
    #[staticmethod]
    #[pyo3(signature = (num_curves=72, base_radius=20.0, amplitude=20.0, resolution=360, center_x=0.0, center_y=0.0))]
    fn limacon(
        num_curves: usize,
        base_radius: f64,
        amplitude: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_limacon(
            num_curves,
            base_radius,
            amplitude,
            resolution,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine flinqué (engine-turned) pattern that produces
    /// identical output to the mathematical FlinqueLayer.
    ///
    /// Models a physical rose engine with a multi-lobe rosette (num_petals
    /// lobes) plus a secondary sinusoidal rosette for fine ripple.  The lathe
    /// makes concentric-ring passes from the inner to the outer radius.
    #[staticmethod]
    #[pyo3(signature = (radius=10.0, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05, center_x=0.0, center_y=0.0))]
    fn flinque(
        radius: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_flinque(
            radius,
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine huit-eight (figure-eight) pattern that produces
    /// identical output to the mathematical HuitEightLayer.
    ///
    /// Models a physical rose engine with a figure-eight cam (lemniscate-
    /// shaped) mounted on the spindle.  Each pass traces a lemniscate of
    /// Bernoulli, and multiple passes at different angular rotations create
    /// the overlapping figure-eight mesh.
    #[staticmethod]
    #[pyo3(signature = (num_curves=72, scale=20.0, resolution=360, center_x=0.0, center_y=0.0, num_clusters=0, cluster_spread=0.0))]
    fn huiteight(
        num_curves: usize,
        scale: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
        num_clusters: usize,
        cluster_spread: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_huiteight(
            num_curves,
            scale,
            resolution,
            center_x,
            center_y,
            num_clusters,
            cluster_spread,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a rose engine clous de Paris (hobnail) pattern that produces
    /// identical output to the mathematical ClousDeParisLayer.
    ///
    /// Models a physical straight-line engine making two orthogonal sets of
    /// parallel V-groove cuts, creating a grid of pyramidal hobnails.
    #[staticmethod]
    #[pyo3(signature = (spacing=1.0, radius=22.0, angle=std::f64::consts::FRAC_PI_4, resolution=200, center_x=0.0, center_y=0.0))]
    fn clous_de_paris(
        spacing: f64,
        radius: f64,
        angle: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> PyResult<Self> {
        BaseRoseEngineLatheRun::new_clous_de_paris(
            spacing,
            radius,
            angle,
            resolution,
            center_x,
            center_y,
        )
        .map(|inner| RoseEngineLatheRun { inner })
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate all passes of the rose engine pattern
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Export combined pattern as SVG
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner.to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Get the number of passes
    #[getter]
    fn num_passes(&self) -> usize {
        self.inner.num_passes()
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
            "RoseEngineLatheRun(center=({}, {}), passes={})",
            self.inner.center_x,
            self.inner.center_y,
            self.inner.num_passes()
        )
    }
}
