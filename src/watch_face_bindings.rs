use pyo3::prelude::*;
use turtles::{
    DiamantConfig as BaseDiamantConfig,
    DiamantLayer as BaseDiamantLayer,
    ExportConfig as BaseExportConfig,
    FlinqueConfig as BaseFlinqueConfig,
    FlinqueLayer as BaseFlinqueLayer,
    HorizontalSpirograph as BaseHorizontalSpirograph,
    SphericalSpirograph as BaseSphericalSpirograph,
    VerticalSpirograph as BaseVerticalSpirograph,
    WatchFace as BaseWatchFace,
};

use crate::diamant_bindings::DiamantLayer;
use crate::guilloche_bindings::FlinqueLayer;
use crate::spirograph_bindings::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};

/// Python wrapper for WatchFace
#[pyclass]
pub struct WatchFace {
    inner: BaseWatchFace,
}

#[pymethods]
impl WatchFace {
    #[new]
    #[pyo3(signature = (radius))]
    fn new(radius: f64) -> PyResult<Self> {
        BaseWatchFace::new(radius)
            .map(|inner| WatchFace { inner })
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[getter]
    fn radius(&self) -> f64 {
        self.inner.radius()
    }

    /// Add the inner dial circle with default styling
    fn add_inner(&mut self) {
        self.inner.add_inner();
    }

    /// Add the outer bezel ring with default styling
    fn add_outer(&mut self) {
        self.inner.add_outer();
    }

    /// Add a center pinhole for watch hands
    fn add_center_hole(&mut self) {
        self.inner.add_center_hole();
    }

    /// Add a hole at a clock position
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    /// * `hole_radius` - Radius of the hole
    #[pyo3(signature = (hour, minute, distance, hole_radius))]
    fn add_hole_at_clock(&mut self, hour: u32, minute: u32, distance: f64, hole_radius: f64) {
        self.inner.add_hole_at_clock(hour, minute, distance, hole_radius);
    }

    /// Add a spirograph layer (HorizontalSpirograph, VerticalSpirograph, or SphericalSpirograph)
    fn add_layer(&mut self, spiro: &Bound<'_, PyAny>) -> PyResult<()> {
        if let Ok(h_spiro) = spiro.extract::<PyRef<HorizontalSpirograph>>() {
            let new_spiro = BaseHorizontalSpirograph::new(
                h_spiro.inner.outer_radius,
                h_spiro.inner.radius_ratio,
                h_spiro.inner.point_distance,
                h_spiro.inner.rotations,
                h_spiro.inner.resolution,
            )
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_horizontal_layer(new_spiro);
            return Ok(());
        }

        if let Ok(v_spiro) = spiro.extract::<PyRef<VerticalSpirograph>>() {
            let new_spiro = BaseVerticalSpirograph::new(
                v_spiro.inner.outer_radius,
                v_spiro.inner.radius_ratio,
                v_spiro.inner.point_distance,
                v_spiro.inner.rotations,
                v_spiro.inner.resolution,
                v_spiro.inner.wave_amplitude,
                v_spiro.inner.wave_frequency,
            )
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_vertical_layer(new_spiro);
            return Ok(());
        }

        if let Ok(s_spiro) = spiro.extract::<PyRef<SphericalSpirograph>>() {
            let new_spiro = BaseSphericalSpirograph::new(
                s_spiro.inner.outer_radius,
                s_spiro.inner.radius_ratio,
                s_spiro.inner.point_distance,
                s_spiro.inner.rotations,
                s_spiro.inner.resolution,
                s_spiro.inner.dome_height,
            )
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            self.inner.add_spherical_layer(new_spiro);
            return Ok(());
        }

        Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected HorizontalSpirograph, VerticalSpirograph, or SphericalSpirograph",
        ))
    }

    /// Add a spirograph layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `spiro_type` - Type of spirograph: "horizontal", "vertical", or "spherical"
    /// * `outer_radius` - Outer circle radius
    /// * `radius_ratio` - Inner/outer radius ratio
    /// * `point_distance` - Drawing point distance
    /// * `rotations` - Number of rotations
    /// * `resolution` - Points per revolution
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center
    /// * `wave_amplitude` - Vertical wave amplitude (for vertical spirograph)
    /// * `wave_frequency` - Vertical wave frequency (for vertical spirograph)
    /// * `dome_height` - Height of dome (for spherical spirograph)
    #[pyo3(signature = (spiro_type, outer_radius, radius_ratio, point_distance, rotations, resolution, hour, minute, distance, wave_amplitude=1.0, wave_frequency=5.0, dome_height=5.0))]
    fn add_layer_at_clock(
        &mut self,
        spiro_type: &str,
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        hour: u32,
        minute: u32,
        distance: f64,
        wave_amplitude: f64,
        wave_frequency: f64,
        dome_height: f64,
    ) -> PyResult<()> {
        match spiro_type.to_lowercase().as_str() {
            "horizontal" => {
                let spiro = BaseHorizontalSpirograph::new_at_clock(
                    outer_radius, radius_ratio, point_distance, rotations, resolution, hour, minute, distance
                ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                self.inner.add_horizontal_layer(spiro);
            }
            "vertical" => {
                let spiro = BaseVerticalSpirograph::new_at_clock(
                    outer_radius, radius_ratio, point_distance, rotations, resolution, wave_amplitude, wave_frequency, hour, minute, distance
                ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                self.inner.add_vertical_layer(spiro);
            }
            "spherical" => {
                let spiro = BaseSphericalSpirograph::new_at_clock(
                    outer_radius, radius_ratio, point_distance, rotations, resolution, dome_height, hour, minute, distance
                ).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
                self.inner.add_spherical_layer(spiro);
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "spiro_type must be 'horizontal', 'vertical', or 'spherical'"
                ));
            }
        }
        Ok(())
    }

    /// Add a flinqué (engine-turned) layer
    fn add_flinque_layer(&mut self, flinque: &FlinqueLayer) -> PyResult<()> {
        let new_layer = BaseFlinqueLayer::new_with_center(
            flinque.inner.radius,
            flinque.inner.config.clone(),
            flinque.inner.center_x,
            flinque.inner.center_y,
        )
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        self.inner.add_flinque_layer(new_layer);
        Ok(())
    }

    /// Add a flinqué layer positioned at a clock position
    #[pyo3(signature = (radius, hour, minute, distance, num_petals=12, num_waves=60, wave_amplitude=0.8, wave_frequency=20.0, inner_radius_ratio=0.05))]
    fn add_flinque_at_clock(
        &mut self,
        radius: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
    ) -> PyResult<()> {
        let config = BaseFlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };
        self.inner
            .add_flinque_at_clock(radius, config, hour, minute, distance)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Add a diamant (diamond pattern) layer
    fn add_diamant_layer(&mut self, diamant: &DiamantLayer) -> PyResult<()> {
        let new_layer = BaseDiamantLayer::new_with_center(
            diamant.inner.config.clone(),
            diamant.inner.center_x,
            diamant.inner.center_y,
        )
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        self.inner.add_diamant_layer(new_layer);
        Ok(())
    }

    /// Add a diamant layer positioned at a clock position
    #[pyo3(signature = (num_circles, circle_radius, hour, minute, distance, resolution=360))]
    fn add_diamant_at_clock(
        &mut self,
        num_circles: usize,
        circle_radius: f64,
        hour: u32,
        minute: u32,
        distance: f64,
        resolution: usize,
    ) -> PyResult<()> {
        let config = BaseDiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };
        self.inner
            .add_diamant_at_clock(config, hour, minute, distance)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    /// Generate all layers
    fn generate(&mut self) {
        self.inner.generate();
    }

    /// Get layer count
    fn layer_count(&self) -> usize {
        self.inner.layer_count()
    }

    /// Export to SVG
    #[pyo3(signature = (filename))]
    fn to_svg(&self, filename: &str) -> PyResult<()> {
        self.inner
            .to_svg(filename)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Export to STL
    #[pyo3(signature = (filename, depth=0.1, base_thickness=2.0))]
    fn to_stl(&self, filename: &str, depth: f64, base_thickness: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness,
            tool_radius: 0.0,
        };
        self.inner
            .to_stl(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    /// Export to STEP
    #[pyo3(signature = (filename, depth=0.1))]
    fn to_step(&self, filename: &str, depth: f64) -> PyResult<()> {
        let config = BaseExportConfig {
            depth,
            base_thickness: 2.0,
            tool_radius: 0.0,
        };
        self.inner
            .to_step(filename, &config)
            .map_err(|e| pyo3::exceptions::PyIOError::new_err(e.to_string()))
    }

    fn __repr__(&self) -> String {
        format!(
            "WatchFace(radius={}, layers={})",
            self.inner.radius(),
            self.inner.layer_count()
        )
    }
}
