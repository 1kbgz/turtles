use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Draperie (Drapery) guilloché pattern
///
/// The draperie pattern is formed by drawing concentric wavy rings whose phase
/// oscillates sinusoidally from the innermost to the outermost ring, producing
/// the classic draping-fabric / flowing-fold appearance seen in fine watchmaking.
///
/// Each ring is a circle modulated by: `r = base_radius + i*radius_step + amplitude * sin(frequency * θ + φ_i)`
/// where `φ_i = phase_shift * sin(2π * phase_oscillations * i / N)`.
///
/// The amplitude is automatically clamped so adjacent rings never cross.
#[derive(Debug, Clone)]
pub struct DraperieConfig {
    /// Number of concentric rings
    pub num_rings: usize,
    /// Radial spacing between ring centres (mm)
    pub radius_step: f64,
    /// Number of wave undulations per revolution
    pub wave_frequency: f64,
    /// Base radius — centre of the ring band (mm).
    /// The innermost ring is at `base_radius - (num_rings-1)/2 * radius_step`.
    pub base_radius: f64,
    /// Maximum radial amplitude of each ring's wave (mm).
    /// If `None`, it is computed automatically to prevent overlap.
    pub amplitude: Option<f64>,
    /// Peak angular oscillation amplitude (radians).
    /// The phase of each ring swings by ±phase_shift from its neutral position.
    pub phase_shift: f64,
    /// Number of full sinusoidal phase cycles across the ring stack.
    /// Each cycle produces 2 visible direction changes, so `phase_oscillations = 2`
    /// produces 4 visible folds from centre to edge.
    pub phase_oscillations: f64,
    /// Resolution — number of points per ring
    pub resolution: usize,
    /// Exponent applied to the sinusoidal phase envelope.
    /// Only used when `circular_phase` is 0 (dome mode disabled).
    /// Use 3 (default) for smooth folds; use 1 for sharp angular folds.
    pub phase_exponent: u32,
    /// Exponent applied to the per-ring wave shape `sin(f*(θ+φ))`.
    /// Use 1 (default) for standard sinusoidal peaks/troughs;
    /// use 3 for softer, rounded wave crests.
    /// This is orthogonal to `phase_exponent` which controls fold sharpness.
    pub wave_exponent: u32,
    /// Dome-shaped phase envelope exponent.
    /// When > 0, the phase envelope uses `sgn(sin(t)) · [1 − (1 − |sin(t)|)^n]`
    /// where `n = circular_phase`.  This produces round, gradually growing
    /// fold curves.  Use 2.0 (default) for standard rounded folds; higher
    /// values produce even more "squared-off" flat-top domes.
    /// When 0.0, falls back to `sin^e` mode using `phase_exponent`.
    pub circular_phase: f64,
}

impl Default for DraperieConfig {
    fn default() -> Self {
        DraperieConfig {
            num_rings: 96,
            radius_step: 0.44,
            wave_frequency: 12.0,
            base_radius: 22.0,
            amplitude: None,        // auto-computed
            phase_shift: PI / 12.0, // 15°
            phase_oscillations: 2.5,
            resolution: 1500,
            phase_exponent: 3,
            wave_exponent: 1,
            circular_phase: 2.0,
        }
    }
}

impl DraperieConfig {
    /// Create a new draperie configuration with sensible defaults.
    ///
    /// # Arguments
    /// * `num_rings` - Number of concentric rings (more = denser)
    /// * `base_radius` - Centre of the ring band in mm
    pub fn new(num_rings: usize, base_radius: f64) -> Self {
        DraperieConfig {
            num_rings,
            base_radius,
            ..Default::default()
        }
    }

    /// Set the resolution (points per ring)
    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    /// Compute the maximum safe amplitude so that adjacent rings never cross
    /// and the innermost ring does not pass through the centre.
    pub fn safe_amplitude(&self) -> f64 {
        // Constraint 1: adjacent rings must not cross.
        //   Compute the maximum phase difference between adjacent rings
        //   numerically, using whichever phase shape is active.
        let dt_ring = 2.0 * PI * self.phase_oscillations / (self.num_rings as f64);
        let n_sample: usize = 1000;
        let mut max_diff = 0.0_f64;
        for k in 0..n_sample {
            let t = 2.0 * PI * (k as f64) / (n_sample as f64);
            let v1 = self.phase_shape_fn(t);
            let v2 = self.phase_shape_fn(t + dt_ring);
            max_diff = max_diff.max((v2 - v1).abs());
        }
        let max_adj_dphi = self.phase_shift * max_diff;
        let sin_term = (self.wave_frequency * max_adj_dphi / 2.0).sin().abs();
        let max_amp_phase = if sin_term > 1e-12 {
            self.radius_step / (2.0 * sin_term)
        } else {
            f64::INFINITY // no phase change → any amplitude is fine
        };

        // Constraint 2: innermost ring must not reach r = 0.
        let innermost_base =
            self.base_radius - ((self.num_rings as f64 - 1.0) / 2.0) * self.radius_step;
        let max_amp_centre = if innermost_base > 0.0 {
            innermost_base * 0.9
        } else {
            0.0
        };

        let max_amplitude = max_amp_phase.min(max_amp_centre);
        // Use 60 % of the theoretical limit for more defined waves
        0.6 * max_amplitude
    }

    /// Evaluate the phase-shape function at parameter `t`.
    ///
    /// * **dome mode** (`circular_phase > 0`):
    ///   Uses `sgn(sin(t)) · [1 − (1 − |sin(t)|)^n]` where `n = circular_phase`.
    ///   n=2 produces round dome-shaped peaks; higher n gives wider flat-top
    ///   dwell at the peaks.
    ///
    /// * **sin-power mode** (`circular_phase == 0`):
    ///   Uses `|sin(t)|^e · sign(sin(t))` where `e = phase_exponent`.
    fn phase_shape_fn(&self, t: f64) -> f64 {
        if self.circular_phase > 0.0 {
            let s = t.sin();
            let a = s.abs();
            let dome = 1.0 - (1.0 - a).powf(self.circular_phase);
            dome * s.signum()
        } else {
            let s = t.sin();
            s.abs().powi(self.phase_exponent as i32) * s.signum()
        }
    }
}

/// A Draperie pattern layer that creates the flowing-fabric guilloché effect
///
/// This pattern is created by drawing concentric sinusoidal rings whose phase
/// oscillates back and forth across the stack, producing visually flowing
/// wave-folds that radiate from the centre — a hallmark of fine guilloché.
#[derive(Debug, Clone)]
pub struct DraperieLayer {
    pub config: DraperieConfig,
    pub center_x: f64,
    pub center_y: f64,
    rings: Vec<Vec<Point2D>>,
}

impl DraperieLayer {
    /// Create a new draperie layer centred at origin
    pub fn new(config: DraperieConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new draperie layer with a custom centre point
    pub fn new_with_center(
        config: DraperieConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.num_rings == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_rings must be at least 1".to_string(),
            ));
        }

        if config.radius_step <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "radius_step must be positive".to_string(),
            ));
        }

        if config.base_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "base_radius must be positive".to_string(),
            ));
        }

        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }

        Ok(DraperieLayer {
            config,
            center_x,
            center_y,
            rings: Vec::new(),
        })
    }

    /// Create a draperie layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: DraperieConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Create a draperie layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Draperie configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face
    pub fn new_at_clock(
        config: DraperieConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Generate the draperie pattern
    ///
    /// Produces `num_rings` concentric wavy rings with a sinusoidal phase
    /// envelope. The amplitude is automatically clamped to prevent overlap
    /// if not explicitly set.
    pub fn generate(&mut self) {
        self.rings.clear();

        let amplitude = self
            .config
            .amplitude
            .unwrap_or_else(|| self.config.safe_amplitude());

        let n = self.config.num_rings;

        // Phase offset so that wave peaks align with 12 o'clock (θ = −π/2 in
        // screen coordinates).  We need sin(f*(−π/2 + base_phase)) = 1,
        // i.e. base_phase = π/2 + π/(2f).
        let base_phase = PI / 2.0 + PI / (2.0 * self.config.wave_frequency);

        for i in 0..n {
            // Ring base radius — centred around config.base_radius
            let offset = (i as f64) - ((n as f64 - 1.0) / 2.0);
            let ring_base_radius = self.config.base_radius + offset * self.config.radius_step;

            // Phase oscillation — use the configured phase shape function
            // (dome arcs by default, or sin^e when circular_phase=0).
            let phase_t = 2.0 * PI * self.config.phase_oscillations * (i as f64) / (n as f64);
            let ring_phase = self.config.phase_shift * self.config.phase_shape_fn(phase_t);

            // Trace the ring
            let mut ring_points = Vec::with_capacity(self.config.resolution + 1);
            for j in 0..=self.config.resolution {
                let t = (j as f64) / (self.config.resolution as f64);
                let theta = 2.0 * PI * t;

                let wave_sin =
                    (self.config.wave_frequency * (theta + base_phase + ring_phase)).sin();
                let wave_val =
                    wave_sin.abs().powi(self.config.wave_exponent as i32) * wave_sin.signum();
                let r = ring_base_radius + amplitude * wave_val;

                let x = self.center_x + r * theta.cos();
                let y = self.center_y + r * theta.sin();
                ring_points.push(Point2D::new(x, y));
            }

            self.rings.push(ring_points);
        }
    }

    /// Get the generated rings
    pub fn rings(&self) -> &Vec<Vec<Point2D>> {
        &self.rings
    }

    /// Get all lines for rendering (alias for rings)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.rings
    }

    /// Export the pattern to SVG format
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;

        if self.rings.is_empty() {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for ring in &self.rings {
            for point in ring {
                min_x = min_x.min(point.x);
                max_x = max_x.max(point.x);
                min_y = min_y.min(point.y);
                max_y = max_y.max(point.y);
            }
        }

        let margin = 5.0;
        let width = max_x - min_x + 2.0 * margin;
        let height = max_y - min_y + 2.0 * margin;

        let mut document = Document::new()
            .set("width", format!("{}mm", width))
            .set("height", format!("{}mm", height))
            .set("viewBox", (min_x - margin, min_y - margin, width, height));

        for ring in &self.rings {
            if ring.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((ring[0].x, ring[0].y));
            for point in ring.iter().skip(1) {
                data = data.line_to((point.x, point.y));
            }

            let path = Path::new()
                .set("d", data)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.05);

            document = document.add(path);
        }

        svg::save(filename, &document)
            .map_err(|e| SpirographError::ExportError(format!("Failed to save SVG: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draperie_config_default() {
        let config = DraperieConfig::default();
        assert_eq!(config.num_rings, 96);
        assert!((config.radius_step - 0.44).abs() < 1e-10);
        assert!((config.wave_frequency - 12.0).abs() < 1e-10);
        assert!(config.amplitude.is_none());
    }

    #[test]
    fn test_draperie_config_new() {
        let config = DraperieConfig::new(80, 25.0);
        assert_eq!(config.num_rings, 80);
        assert!((config.base_radius - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_draperie_layer_creation() {
        let config = DraperieConfig::default();
        let layer = DraperieLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_draperie_layer_invalid_params() {
        // zero rings
        let config = DraperieConfig {
            num_rings: 0,
            ..Default::default()
        };
        assert!(DraperieLayer::new(config).is_err());

        // negative radius_step
        let config = DraperieConfig {
            radius_step: -1.0,
            ..Default::default()
        };
        assert!(DraperieLayer::new(config).is_err());

        // zero base_radius
        let config = DraperieConfig {
            base_radius: 0.0,
            ..Default::default()
        };
        assert!(DraperieLayer::new(config).is_err());
    }

    #[test]
    fn test_draperie_generate() {
        let config = DraperieConfig::new(20, 15.0).with_resolution(100);
        let mut layer = DraperieLayer::new(config).unwrap();
        layer.generate();

        assert_eq!(layer.rings().len(), 20);
        assert_eq!(layer.rings()[0].len(), 101); // resolution + 1
    }

    #[test]
    fn test_draperie_rings_non_overlapping() {
        let config = DraperieConfig::default();
        let mut layer = DraperieLayer::new(config).unwrap();
        layer.generate();

        // Check that adjacent rings never cross
        let rings = layer.rings();
        for i in 0..rings.len() - 1 {
            let inner = &rings[i];
            let outer = &rings[i + 1];
            let n = inner.len().min(outer.len());
            for j in 0..n {
                let r_inner = (inner[j].x.powi(2) + inner[j].y.powi(2)).sqrt();
                let r_outer = (outer[j].x.powi(2) + outer[j].y.powi(2)).sqrt();
                assert!(
                    r_outer >= r_inner - 1e-6,
                    "Ring {} crosses ring {} at point {}: r_inner={}, r_outer={}",
                    i + 1,
                    i,
                    j,
                    r_inner,
                    r_outer
                );
            }
        }
    }

    #[test]
    fn test_safe_amplitude_not_zero() {
        let config = DraperieConfig::default();
        let amp = config.safe_amplitude();
        assert!(amp > 0.0, "Safe amplitude should be positive, got {}", amp);
    }

    #[test]
    fn test_draperie_matches_rose_engine() {
        use crate::rose_engine::RoseEngineLatheRun;

        // Use defaults matching the mathematical module
        let num_rings = 96;
        let base_radius = 22.0;
        let radius_step = 0.44;
        let wave_frequency = 12.0;
        let phase_shift = PI / 12.0;
        let phase_oscillations = 2.5;
        let resolution = 1500;
        let phase_exponent = 3_u32;
        let wave_exponent = 1_u32;
        let circular_phase = 2.0_f64;

        // Create mathematical DraperieLayer
        let config = DraperieConfig {
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            amplitude: None,
            phase_shift,
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
        };
        let mut math_layer = DraperieLayer::new(config).unwrap();
        math_layer.generate();

        // Create equivalent rose engine draperie
        let mut rose_run = RoseEngineLatheRun::new_draperie(
            num_rings,
            base_radius,
            radius_step,
            wave_frequency,
            phase_shift,
            phase_oscillations,
            resolution,
            phase_exponent,
            wave_exponent,
            circular_phase,
            0.0,
            0.0,
        )
        .unwrap();
        rose_run.generate();

        // Both should have the same number of rings/lines
        let math_lines = math_layer.lines();
        let rose_lines = rose_run.lines();

        assert_eq!(
            math_lines.len(),
            rose_lines.len(),
            "DraperieLayer and RoseEngineLatheRun should have same number of rings"
        );

        // Each ring should have the same number of points
        for (i, (math_ring, rose_ring)) in math_lines.iter().zip(rose_lines.iter()).enumerate() {
            assert_eq!(
                math_ring.len(),
                rose_ring.len(),
                "Ring {} should have same number of points",
                i
            );

            // Compare all points — they should be identical (within floating point tolerance)
            for (j, (math_pt, rose_pt)) in math_ring.iter().zip(rose_ring.iter()).enumerate() {
                let dist =
                    ((math_pt.x - rose_pt.x).powi(2) + (math_pt.y - rose_pt.y).powi(2)).sqrt();
                assert!(
                    dist < 1e-10,
                    "Point {},{} differs: math=({}, {}), rose=({}, {}), dist={}",
                    i,
                    j,
                    math_pt.x,
                    math_pt.y,
                    rose_pt.x,
                    rose_pt.y,
                    dist
                );
            }
        }
    }
}
