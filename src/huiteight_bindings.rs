use pyo3::prelude::*;
use turtles::{
    HuitEightConfig as BaseHuitEightConfig,
    HuitEightLayer as BaseHuitEightLayer,
};

/// Python wrapper for HuitEightLayer - creates figure-eight guilloché patterns
/// using lemniscates of Bernoulli that pass through the centre, rotated
/// around the centre
#[pyclass]
pub struct HuitEightLayer {
    pub inner: BaseHuitEightLayer,
}

#[pymethods]
impl HuitEightLayer {
    /// Create a new huit-eight layer centred at origin
    ///
    /// # Arguments
    /// * `num_curves` - Number of figure-eight curves (more = denser mesh)
    /// * `scale` - Half-width of each lemniscate
    /// * `resolution` - Number of points per curve (default: 360)
    /// * `num_clusters` - Number of clusters to group curves into (0 = uniform)
    /// * `cluster_spread` - Angular spread within each cluster in radians (0.0 = auto)
    #[new]
    #[pyo3(signature = (num_curves, scale, resolution=360, num_clusters=0, cluster_spread=0.0))]
    fn new(num_curves: usize, scale: f64, resolution: usize, num_clusters: usize, cluster_spread: f64) -> PyResult<Self> {
        let config = BaseHuitEightConfig {
            num_curves,
            scale,
            resolution,
            num_clusters,
            cluster_spread,
        };
        BaseHuitEightLayer::new(config)
            .map(|inner| HuitEightLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a huit-eight layer with a custom centre point
    #[staticmethod]
    #[pyo3(signature = (num_curves, scale, center_x, center_y, resolution=360, num_clusters=0, cluster_spread=0.0))]
    fn with_center(
        num_curves: usize,
        scale: f64,
        center_x: f64,
        center_y: f64,
        resolution: usize,
        num_clusters: usize,
        cluster_spread: f64,
    ) -> PyResult<Self> {
        let config = BaseHuitEightConfig {
            num_curves,
            scale,
            resolution,
            num_clusters,
            cluster_spread,
        };
        BaseHuitEightLayer::new_with_center(config, center_x, center_y)
            .map(|inner| HuitEightLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a huit-eight layer positioned at a given angle and distance from origin
    #[staticmethod]
    #[pyo3(signature = (num_curves, scale, angle, distance, resolution=360, num_clusters=0, cluster_spread=0.0))]
    fn at_polar(
        num_curves: usize,
        scale: f64,
        angle: f64,
        distance: f64,
        resolution: usize,
        num_clusters: usize,
        cluster_spread: f64,
    ) -> PyResult<Self> {
        let config = BaseHuitEightConfig {
            num_curves,
            scale,
            resolution,
            num_clusters,
            cluster_spread,
        };
        BaseHuitEightLayer::new_at_polar(config, angle, distance)
            .map(|inner| HuitEightLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Create a huit-eight layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `num_curves` - Number of figure-eight curves
    /// * `scale` - Half-width of each lemniscate
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face to the subdial centre
    /// * `resolution` - Number of points per curve (default: 360)
    #[staticmethod]
    #[pyo3(signature = (num_curves, scale, hour, minute, distance, resolution=360, num_clusters=0, cluster_spread=0.0))]
    fn at_clock(
        num_curves: usize,
        scale: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        resolution: usize,
        num_clusters: usize,
        cluster_spread: f64,
    ) -> PyResult<Self> {
        let config = BaseHuitEightConfig {
            num_curves,
            scale,
            resolution,
            num_clusters,
            cluster_spread,
        };
        BaseHuitEightLayer::new_at_clock(config, hour, minute, distance)
            .map(|inner| HuitEightLayer { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate the huit-eight pattern
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

    /// Get the number of curves in the pattern
    #[getter]
    fn num_curves(&self) -> usize {
        self.inner.config.num_curves
    }

    /// Get the scale (half-width) of each lemniscate
    #[getter]
    fn scale(&self) -> f64 {
        self.inner.config.scale
    }

    /// Get the centre x coordinate
    #[getter]
    fn center_x(&self) -> f64 {
        self.inner.center_x
    }

    /// Get the centre y coordinate
    #[getter]
    fn center_y(&self) -> f64 {
        self.inner.center_y
    }

    /// Get the number of clusters (0 = uniform distribution)
    #[getter]
    fn num_clusters(&self) -> usize {
        self.inner.config.num_clusters
    }

    /// Get the cluster spread in radians (0.0 = auto)
    #[getter]
    fn cluster_spread(&self) -> f64 {
        self.inner.config.cluster_spread
    }

    fn __repr__(&self) -> String {
        format!(
            "HuitEightLayer(num_curves={}, scale={}, center=({}, {}))",
            self.inner.config.num_curves,
            self.inner.config.scale,
            self.inner.center_x,
            self.inner.center_y
        )
    }
}
