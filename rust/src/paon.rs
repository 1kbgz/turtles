use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Compute the paon waveform value at angle `theta`.
///
/// When `n_harmonics == 0`, returns `sin(theta)` (pure sine wave, smooth arches).
///
/// When `n_harmonics >= 1`, returns a normalised Fourier triangle-wave
/// approximation using the first `n_harmonics + 1` terms:
///
///   `wave(θ) = Σ_{k=0}^{n} (-1)^k sin((2k+1)θ) / (2k+1)²`   (normalised to peak = 1)
///
/// Higher `n_harmonics` produces increasingly sharp, triangular arch cusps,
/// closely matching the pointed peacock-feather motif of traditional guilloché.
pub fn paon_wave_fn(theta: f64, n_harmonics: usize) -> f64 {
    if n_harmonics == 0 {
        theta.sin()
    } else {
        let mut sum = 0.0;
        let mut norm = 0.0;
        for k in 0..=n_harmonics {
            let sign = if k % 2 == 0 { 1.0 } else { -1.0 };
            let harmonic = (2 * k + 1) as f64;
            let coeff = sign / (harmonic * harmonic);
            sum += coeff * (harmonic * theta).sin();
            // Normalization: value at theta = π/2 (the peak)
            norm += coeff * (harmonic * PI / 2.0).sin();
        }
        sum / norm
    }
}

/// Configuration for the Paon (Peacock) guilloché pattern
///
/// The paon pattern uses **parallel horizontal lines** stacked from the
/// bottom of the circle to the top, each oscillating **vertically** with a
/// progressive phase offset.  This mirrors how a rose-engine lathe cuts the
/// pattern: a tool sweeps horizontally while the work-piece bobs up and
/// down via a rosette cam, then steps to the next horizontal pass.
///
/// The interference between neighbouring phase-shifted oscillating lines
/// creates the characteristic **peacock-feather arch bands** of traditional
/// guilloché.  The arch shape emerges because the oscillation uses a
/// constant spatial frequency (tied to absolute x-position across the
/// circle), so the phase-alignment curves become diagonal chords that
/// appear as nested arches when clipped to the circle.
///
/// Lines are clipped to a circle of the given `radius`.
#[derive(Debug, Clone)]
pub struct PaonConfig {
    /// Number of horizontal passes (more = denser, finer pattern)
    pub num_lines: usize,
    /// Radius of the circular dial (lines are clipped to this circle)
    pub radius: f64,
    /// Vertical oscillation amplitude (mm).
    /// Controls how far each line bobs up and down.
    pub amplitude: f64,
    /// Number of oscillation cycles across the full diameter.
    /// Higher values create more, finer arch bands.
    pub wave_frequency: f64,
    /// Number of arch columns across the circle.
    /// Each bump of the |sin| phase envelope creates one arch column.
    pub phase_rate: f64,
    /// Number of sample points per line
    pub resolution: usize,
    /// Number of Fourier harmonics for the waveform shape.
    /// 0 = pure sine (smooth arches), 1+ = triangle-wave approximation
    /// (sharper, more pointed arch cusps). 3 is a good default for a
    /// classic paon look.
    pub n_harmonics: usize,
    /// Phase amplitude: controls the height of the arch bands
    /// (in units of full wave cycles).  Larger values create taller,
    /// more pronounced arches.
    pub fan_angle: f64,
    /// Vanishing-point distance below the circle bottom, expressed as a
    /// fraction of the diameter.  Lines radiate from the vanishing point,
    /// so arches are narrow at the bottom and wide at the top.
    /// 0 = VP at circle bottom (extreme fan), large values → nearly
    /// parallel vertical lines.  Default 0.3 ≈ 20 % of diameter below
    /// the bottom edge.
    pub vanishing_point: f64,
}

impl Default for PaonConfig {
    fn default() -> Self {
        PaonConfig {
            num_lines: 500,
            radius: 22.0,
            amplitude: 0.035,
            wave_frequency: 10.0,
            phase_rate: 9.0,
            resolution: 800,
            n_harmonics: 3,
            fan_angle: 4.0,
            vanishing_point: 0.3,
        }
    }
}

impl PaonConfig {
    /// Create a new paon configuration with sensible defaults.
    ///
    /// # Arguments
    /// * `num_lines` - Number of parallel lines (more = denser)
    /// * `radius` - Radius of the circular dial in mm
    pub fn new(num_lines: usize, radius: f64) -> Self {
        PaonConfig {
            num_lines,
            radius,
            ..Default::default()
        }
    }

    /// Set the resolution (points per line)
    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }
}

/// A Paon (Peacock) pattern layer that creates the arch/peacock-feather guilloché effect
///
/// Parallel horizontal lines oscillate vertically with progressive phase
/// offsets.  The interference between neighbours creates moiré-style arch
/// bands that resemble peacock feathers.
#[derive(Debug, Clone)]
pub struct PaonLayer {
    pub config: PaonConfig,
    pub center_x: f64,
    pub center_y: f64,
    lines: Vec<Vec<Point2D>>,
}

impl PaonLayer {
    /// Create a new paon layer centred at origin
    pub fn new(config: PaonConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new paon layer with a custom centre point
    pub fn new_with_center(
        config: PaonConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.num_lines == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_lines must be at least 1".to_string(),
            ));
        }

        if config.radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "radius must be positive".to_string(),
            ));
        }

        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }

        if config.amplitude < 0.0 {
            return Err(SpirographError::InvalidParameter(
                "amplitude must be non-negative".to_string(),
            ));
        }

        Ok(PaonLayer {
            config,
            center_x,
            center_y,
            lines: Vec::new(),
        })
    }

    /// Create a paon layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: PaonConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Create a paon layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Paon configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face
    pub fn new_at_clock(
        config: PaonConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Generate the paon pattern
    ///
    /// Lines radiate from a **vanishing point** above the circle top in
    /// mathematical coordinates.  Because SVG renders y downward, the VP
    /// appears at the **bottom** of the rendered image — lines converge
    /// at the bottom and fan out toward the top, matching the classic
    /// peacock-feather guilloché reference.
    ///
    /// Lines are uniformly spaced by **angle** from the VP.  The angle
    /// range is computed to cover the entire circle: the critical
    /// y-level where the circle edge subtends the maximum angle from the
    /// VP is found analytically, guaranteeing every point inside the
    /// circle has lines through it.
    ///
    /// The oscillation phase uses a **logarithmic** distance scale from
    /// the VP, so the oscillation wavelength grows with distance.  This
    /// makes the moiré arch bands **narrow at the bottom** (near the VP
    /// in SVG) and **wide at the top** (far from the VP in SVG).
    ///
    /// The arch columns are created by an **|sin| per-line phase offset**:
    ///
    ///   `line_phase = 2π · fan_angle · |sin(π · phase_rate · frac)|`
    ///
    /// * `phase_rate` controls the number of arch columns.
    /// * `fan_angle` controls the arch height (in wave-cycle units).
    /// * `vanishing_point` controls how strongly lines fan out.
    ///
    /// `amplitude` must be small relative to the inter-line spacing so
    /// that neighbouring lines **never cross** — the visual pattern is a
    /// pure moiré density illusion.
    pub fn generate(&mut self) {
        self.lines.clear();

        let r = self.config.radius;
        let n = self.config.num_lines;
        let nh = self.config.n_harmonics;
        let diameter = 2.0 * r;

        // VP above circle top in math coords (= below circle in SVG).
        // Lines converge at the SVG bottom and fan out toward the SVG top.
        let y_vp = r + self.config.vanishing_point * diameter;

        // Critical y where the angle from VP to the circle edge is maximised.
        // Ensures the fan is wide enough to cover every point on the circle.
        let y_crit = (r * r / y_vp).min(r);
        let angle_max = ((r * r - y_crit * y_crit).sqrt() / (y_vp - y_crit)).atan();

        // Reference distance: VP to nearest circle edge (math top = SVG bottom).
        let dist_near = y_vp - r; // = vanishing_point * diameter

        for i in 0..n {
            let frac = if n > 1 {
                i as f64 / (n - 1) as f64
            } else {
                0.5
            };

            // Uniform angular spacing from the VP
            let angle = -angle_max + 2.0 * angle_max * frac;
            let tan_a = angle.tan();

            // Negative |sin| phase offset → arches open UPWARD (M-shape)
            let line_phase = -2.0
                * PI
                * self.config.fan_angle
                * (PI * self.config.phase_rate * frac).sin().abs();

            let mut line_points = Vec::with_capacity(self.config.resolution + 1);

            for j in 0..=self.config.resolution {
                let t_frac = j as f64 / self.config.resolution as f64;

                // y sweeps from −r (SVG top, far from VP) to +r (SVG bottom, near VP)
                let y = -r + diameter * t_frac;

                // x position along the radiating line from VP
                let x_base = (y_vp - y) * tan_a;

                // Distance from VP (always positive within the circle)
                let dist = y_vp - y;

                // Log-scaled oscillation phase: wavelength grows with
                // distance from VP, so arches are narrow near the VP
                // (SVG bottom) and wide far from the VP (SVG top).
                let theta =
                    2.0 * PI * self.config.wave_frequency * (dist / dist_near).ln() + line_phase;

                let offset = self.config.amplitude * paon_wave_fn(theta, nh);

                // Horizontal oscillation
                let x = x_base + offset;

                // Clip to circle
                if x * x + y * y <= r * r {
                    line_points.push(Point2D::new(self.center_x + x, self.center_y + y));
                }
            }

            if line_points.len() >= 2 {
                self.lines.push(line_points);
            }
        }
    }

    /// Get the generated lines
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.lines
    }

    /// Export the pattern to SVG format
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;

        if self.lines.is_empty() {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for line in &self.lines {
            for point in line {
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

        for line in &self.lines {
            if line.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((line[0].x, line[0].y));
            for point in line.iter().skip(1) {
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
    fn test_paon_config_default() {
        let config = PaonConfig::default();
        assert_eq!(config.num_lines, 500);
        assert!((config.radius - 22.0).abs() < 1e-10);
        assert!((config.amplitude - 0.035).abs() < 1e-10);
        assert!((config.wave_frequency - 10.0).abs() < 1e-10);
        assert!((config.phase_rate - 9.0).abs() < 1e-10);
        assert_eq!(config.n_harmonics, 3);
        assert!((config.fan_angle - 4.0).abs() < 1e-10);
        assert!((config.vanishing_point - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_paon_config_new() {
        let config = PaonConfig::new(150, 25.0);
        assert_eq!(config.num_lines, 150);
        assert!((config.radius - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_paon_layer_creation() {
        let config = PaonConfig::default();
        let layer = PaonLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_paon_layer_invalid_params() {
        // zero lines
        let config = PaonConfig {
            num_lines: 0,
            ..Default::default()
        };
        assert!(PaonLayer::new(config).is_err());

        // negative radius
        let config = PaonConfig {
            radius: -1.0,
            ..Default::default()
        };
        assert!(PaonLayer::new(config).is_err());

        // zero radius
        let config = PaonConfig {
            radius: 0.0,
            ..Default::default()
        };
        assert!(PaonLayer::new(config).is_err());

        // low resolution
        let config = PaonConfig {
            resolution: 5,
            ..Default::default()
        };
        assert!(PaonLayer::new(config).is_err());

        // negative amplitude
        let config = PaonConfig {
            amplitude: -1.0,
            ..Default::default()
        };
        assert!(PaonLayer::new(config).is_err());
    }

    #[test]
    fn test_paon_generate() {
        let config = PaonConfig {
            num_lines: 50,
            radius: 20.0,
            amplitude: 0.5,
            wave_frequency: 6.0,
            phase_rate: 4.0,
            resolution: 200,
            n_harmonics: 0,
            fan_angle: 1.4,
            vanishing_point: 0.3,
        };
        let mut layer = PaonLayer::new(config).unwrap();
        layer.generate();

        // Should have generated some lines (close to 50, minus those at edges)
        assert!(!layer.lines().is_empty());
        assert!(layer.lines().len() <= 50);

        // Each line should have multiple points
        for line in layer.lines() {
            assert!(line.len() >= 2);
        }
    }

    #[test]
    fn test_paon_lines_within_circle() {
        let config = PaonConfig {
            num_lines: 100,
            radius: 20.0,
            amplitude: 0.5,
            wave_frequency: 6.0,
            phase_rate: 4.0,
            resolution: 200,
            n_harmonics: 0,
            fan_angle: 1.4,
            vanishing_point: 0.3,
        };
        let mut layer = PaonLayer::new(config).unwrap();
        layer.generate();

        let r = 20.0;
        for line in layer.lines() {
            for point in line {
                let dist = (point.x.powi(2) + point.y.powi(2)).sqrt();
                assert!(
                    dist <= r + 0.01,
                    "Point ({}, {}) is outside the circle (dist={})",
                    point.x,
                    point.y,
                    dist
                );
            }
        }
    }

    #[test]
    fn test_paon_with_center() {
        let config = PaonConfig::default();
        let layer = PaonLayer::new_with_center(config, 5.0, 10.0).unwrap();
        assert!((layer.center_x - 5.0).abs() < 1e-10);
        assert!((layer.center_y - 10.0).abs() < 1e-10);
    }

    #[test]
    fn test_paon_at_clock() {
        let config = PaonConfig::default();
        let layer = PaonLayer::new_at_clock(config, 3, 0, 20.0).unwrap();
        // At 3 o'clock, x should be positive, y should be ~0
        assert!(layer.center_x > 0.0);
        assert!(layer.center_y.abs() < 0.001);
    }

    #[test]
    fn test_paon_svg_export() {
        let config = PaonConfig {
            num_lines: 30,
            radius: 15.0,
            amplitude: 0.5,
            wave_frequency: 4.0,
            phase_rate: 3.0,
            resolution: 100,
            n_harmonics: 0,
            fan_angle: 1.4,
            vanishing_point: 0.3,
        };
        let mut layer = PaonLayer::new(config).unwrap();
        layer.generate();

        let tmpfile = std::env::temp_dir().join("test_paon.svg");
        let result = layer.to_svg(tmpfile.to_str().expect("temp dir path is valid UTF-8"));
        assert!(result.is_ok());
        // Cleanup
        let _ = std::fs::remove_file(&tmpfile);
    }

    #[test]
    fn test_paon_wave_fn() {
        // n_harmonics=0 should be pure sine
        assert!((paon_wave_fn(0.0, 0) - 0.0).abs() < 1e-10);
        assert!((paon_wave_fn(PI / 2.0, 0) - 1.0).abs() < 1e-10);
        assert!((paon_wave_fn(PI, 0) - 0.0).abs() < 1e-10);

        // n_harmonics >= 1: peak should be normalized to 1
        for nh in 1..=5 {
            let peak = paon_wave_fn(PI / 2.0, nh);
            assert!(
                (peak - 1.0).abs() < 1e-10,
                "n_harmonics={}: peak={}, expected 1.0",
                nh,
                peak
            );
            // At theta=0, should be 0
            assert!(
                paon_wave_fn(0.0, nh).abs() < 1e-10,
                "n_harmonics={}: wave(0) should be 0",
                nh
            );
        }
    }

    #[test]
    fn test_paon_matches_rose_engine() {
        use crate::rose_engine::RoseEngineLatheRun;

        let num_lines = 100;
        let radius = 22.0;
        let amplitude = 0.5;
        let wave_frequency = 8.0;
        let phase_rate = 5.0;
        let resolution = 400;
        let n_harmonics: usize = 3;
        let fan_angle: f64 = 1.4;

        // Mathematical PaonLayer
        let config = PaonConfig {
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            vanishing_point: 0.3,
        };
        let mut math_layer = PaonLayer::new(config).unwrap();
        math_layer.generate();

        // Rose engine PaonLayer
        let mut rose_run = RoseEngineLatheRun::new_paon(
            num_lines,
            radius,
            amplitude,
            wave_frequency,
            phase_rate,
            resolution,
            n_harmonics,
            fan_angle,
            0.3,
            0.0,
            0.0,
        )
        .unwrap();
        rose_run.generate();

        let math_lines = math_layer.lines();
        let rose_lines = rose_run.lines();

        assert_eq!(
            math_lines.len(),
            rose_lines.len(),
            "PaonLayer and RoseEngineLatheRun should have same number of lines: math={}, rose={}",
            math_lines.len(),
            rose_lines.len(),
        );

        for (i, (math_line, rose_line)) in math_lines.iter().zip(rose_lines.iter()).enumerate() {
            assert_eq!(
                math_line.len(),
                rose_line.len(),
                "Line {} should have same number of points: math={}, rose={}",
                i,
                math_line.len(),
                rose_line.len(),
            );

            for (j, (math_pt, rose_pt)) in math_line.iter().zip(rose_line.iter()).enumerate() {
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
