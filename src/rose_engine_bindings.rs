use pyo3::prelude::*;
use turtles::{
    RoseEngineLathe as BaseRoseEngineLathe,
    RoseEngineConfig as BaseRoseEngineConfig,
    RosettePattern as BaseRosettePattern,
    CuttingBit as BaseCuttingBit,
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
    /// Create a circular rosette (no modulation)
    #[staticmethod]
    fn circular() -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Circular,
        }
    }
    
    /// Create an elliptical rosette
    #[staticmethod]
    #[pyo3(signature = (major_axis, minor_axis))]
    fn elliptical(major_axis: f64, minor_axis: f64) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Elliptical { major_axis, minor_axis },
        }
    }
    
    /// Create a sinusoidal rosette
    #[staticmethod]
    #[pyo3(signature = (frequency))]
    fn sinusoidal(frequency: f64) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Sinusoidal { frequency },
        }
    }
    
    /// Create a multi-lobe rosette
    #[staticmethod]
    #[pyo3(signature = (lobes))]
    fn multi_lobe(lobes: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::MultiLobe { lobes },
        }
    }
    
    /// Create an epicycloid rosette
    #[staticmethod]
    #[pyo3(signature = (petals))]
    fn epicycloid(petals: usize) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Epicycloid { petals },
        }
    }
    
    /// Create a custom rosette
    #[staticmethod]
    #[pyo3(signature = (name))]
    fn custom(name: String) -> Self {
        RosettePattern {
            inner: BaseRosettePattern::Custom { name },
        }
    }
    
    fn __repr__(&self) -> String {
        format!("RosettePattern({})", self.inner.name())
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
    #[pyo3(signature = (angle, width))]
    fn v_shaped(angle: f64, width: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::v_shaped(angle, width),
        }
    }
    
    /// Create a flat cutting bit
    #[staticmethod]
    #[pyo3(signature = (width, depth))]
    fn flat(width: f64, depth: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::flat(width, depth),
        }
    }
    
    /// Create a round/ball-nose cutting bit
    #[staticmethod]
    #[pyo3(signature = (radius))]
    fn round(radius: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::round(radius),
        }
    }
    
    /// Create an elliptical cutting bit
    #[staticmethod]
    #[pyo3(signature = (width, height))]
    fn elliptical(width: f64, height: f64) -> Self {
        CuttingBit {
            inner: BaseCuttingBit::elliptical(width, height),
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
        format!("CuttingBit(width={}, depth={})", self.inner.width, self.inner.depth)
    }
}

/// Python wrapper for RoseEngineConfig
#[pyclass]
#[derive(Clone)]
pub struct RoseEngineConfig {
    pub(crate) inner: BaseRoseEngineConfig,
}

#[pymethods]
impl RoseEngineConfig {
    /// Create a new rose engine configuration
    #[new]
    #[pyo3(signature = (rosette, amplitude, base_radius, resolution))]
    fn new(rosette: RosettePattern, amplitude: f64, base_radius: f64, resolution: usize) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::new(rosette.inner, amplitude, base_radius, resolution),
        }
    }
    
    /// Create a configuration for a classic flinqué pattern
    #[staticmethod]
    #[pyo3(signature = (num_petals, base_radius))]
    fn flinque(num_petals: usize, base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::flinque(num_petals, base_radius),
        }
    }
    
    /// Create a configuration for a sunray pattern
    #[staticmethod]
    #[pyo3(signature = (num_rays, base_radius))]
    fn sunray(num_rays: usize, base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::sunray(num_rays, base_radius),
        }
    }
    
    /// Create a configuration for a grain de riz (rice grain) pattern
    #[staticmethod]
    #[pyo3(signature = (base_radius))]
    fn grain_de_riz(base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::grain_de_riz(base_radius),
        }
    }
    
    /// Create a configuration for a draperie (drapery) pattern
    #[staticmethod]
    #[pyo3(signature = (base_radius))]
    fn draperie(base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::draperie(base_radius),
        }
    }
    
    /// Create a configuration for a diamond pattern
    #[staticmethod]
    #[pyo3(signature = (base_radius))]
    fn diamant(base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::diamant(base_radius),
        }
    }
    
    /// Create a configuration for a clou de paris (hobnail) pattern
    #[staticmethod]
    #[pyo3(signature = (base_radius))]
    fn clou_de_paris(base_radius: f64) -> Self {
        RoseEngineConfig {
            inner: BaseRoseEngineConfig::clou_de_paris(base_radius),
        }
    }
    
    /// Set the center position
    #[pyo3(signature = (x, y))]
    fn with_center(&self, x: f64, y: f64) -> Self {
        RoseEngineConfig {
            inner: self.inner.clone().with_center(x, y),
        }
    }
    
    /// Set the phase offset
    #[pyo3(signature = (phase))]
    fn with_phase(&self, phase: f64) -> Self {
        RoseEngineConfig {
            inner: self.inner.clone().with_phase(phase),
        }
    }
    
    /// Set the angular range
    #[pyo3(signature = (start, end))]
    fn with_angle_range(&self, start: f64, end: f64) -> Self {
        RoseEngineConfig {
            inner: self.inner.clone().with_angle_range(start, end),
        }
    }
    
    /// Set the depth modulation
    #[pyo3(signature = (modulation))]
    fn with_depth_modulation(&self, modulation: f64) -> Self {
        RoseEngineConfig {
            inner: self.inner.clone().with_depth_modulation(modulation),
        }
    }
    
    #[getter]
    fn amplitude(&self) -> f64 {
        self.inner.amplitude
    }
    
    #[getter]
    fn base_radius(&self) -> f64 {
        self.inner.base_radius
    }
    
    #[getter]
    fn resolution(&self) -> usize {
        self.inner.resolution
    }
    
    fn __repr__(&self) -> String {
        format!(
            "RoseEngineConfig(amplitude={}, base_radius={}, resolution={})",
            self.inner.amplitude, self.inner.base_radius, self.inner.resolution
        )
    }
}

/// Python wrapper for RoseEngineLathe
#[pyclass]
pub struct RoseEngineLathe {
    inner: BaseRoseEngineLathe,
}

#[pymethods]
impl RoseEngineLathe {
    /// Create a new rose engine lathe
    #[new]
    #[pyo3(signature = (config, cutting_bit))]
    fn new(config: RoseEngineConfig, cutting_bit: CuttingBit) -> PyResult<Self> {
        BaseRoseEngineLathe::new(config.inner, cutting_bit.inner)
            .map(|inner| RoseEngineLathe { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
    
    /// Generate the complete rose engine pattern
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
    
    /// Export pattern as STEP file (not yet implemented)
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
    
    /// Export pattern as STL file (not yet implemented)
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
            "RoseEngineLathe(base_radius={}, amplitude={})",
            self.inner.config.base_radius,
            self.inner.config.amplitude
        ))
    }
}
