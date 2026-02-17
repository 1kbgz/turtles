use crate::common::{Point2D, SpirographError};
use crate::draperie::DraperieConfig;
use crate::paon::{paon_wave_fn, PaonConfig};
use crate::rose_engine::{CuttingBit, RoseEngineConfig, RoseEngineLathe, RosettePattern};
use std::f64::consts::PI;

/// A multi-pass rose engine lathe run that creates complex guilloché patterns
/// by making multiple overlapping cuts at different rotations.
///
/// This simulates the traditional watchmaking technique where a rose engine lathe
/// is used to make multiple passes at different angular positions to create
/// intricate intersecting patterns.
#[derive(Debug, Clone)]
pub struct RoseEngineLatheRun {
    /// Base configuration for each pass
    pub base_config: RoseEngineConfig,
    /// Cutting bit configuration
    pub cutting_bit: CuttingBit,
    /// Number of rotational passes to make
    pub num_passes: usize,
    /// Number of segments per pass (creates gaps for classical guilloché appearance)
    pub segments_per_pass: usize,
    /// Radius step for concentric ring mode.
    /// When non-zero, each pass changes the base_radius by this amount
    /// instead of rotating the phase. Used for draperie and similar patterns
    /// where concentric non-overlapping rings are desired.
    pub radius_step: f64,
    /// Phase oscillation amplitude in concentric ring mode (radians).
    /// The phase of each ring varies sinusoidally across the ring stack:
    ///   phase = base_phase + phase_shift * phase_shape_fn(2π * phase_oscillations * i / num_passes)
    /// This creates the classic draperie "back and forth" fold effect
    /// where wave peaks sway left then right from center to edge.
    pub phase_shift: f64,
    /// Number of full sinusoidal cycles the phase completes across all rings.
    /// Controls how many times the wave peaks sway back and forth from
    /// center to edge. Default 1.0; the reference draperie image uses ~4-5.
    pub phase_oscillations: f64,
    /// Dome-shaped phase envelope exponent for concentric ring mode.
    /// When > 0, uses `sgn(sin(t)) · [1 − (1 − |sin(t)|)^n]`.
    /// When 0, uses `|sin(t)|^phase_exponent · sgn(sin(t))`.
    /// Default 0.0 (plain sin, backward compatible).
    pub circular_phase: f64,
    /// Exponent for the sin-power phase envelope (only when circular_phase == 0).
    /// Default 1 (plain sin, backward compatible).
    pub phase_exponent: u32,
    /// Center position of the pattern (x, y)
    pub center_x: f64,
    pub center_y: f64,

    /// Optional paon (linear pass) configuration.
    /// When set, `generate()` produces parallel vertical lines with sinusoidal
    /// displacement instead of circular lathe passes.
    linear_paon: Option<PaonConfig>,

    // Generated data
    passes: Vec<RoseEngineLathe>,
    segmented_lines: Vec<Vec<Point2D>>,
    generated: bool,
}

impl RoseEngineLatheRun {
    /// Create a new multi-pass rose engine lathe run
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes (typically 8-24)
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern};
    ///
    /// let mut config = RoseEngineConfig::new(20.0, 2.0);
    /// config.rosette = RosettePattern::MultiLobe { lobes: 12 };
    ///
    /// let bit = CuttingBit::v_shaped(30.0, 0.5);
    /// let mut run = RoseEngineLatheRun::new(config, bit, 12).unwrap();
    /// run.generate();
    /// run.to_svg("guilloche_pattern.svg").unwrap();
    /// ```
    pub fn new(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
    ) -> Result<Self, SpirographError> {
        // Default to 24 segments per pass for classical guilloché appearance
        Self::new_with_segments(config, cutting_bit, num_passes, 24, 0.0, 0.0)
    }

    /// Create a new multi-pass rose engine lathe run with custom segmentation
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes
    /// * `segments_per_pass` - Number of arc segments per pass (creates gaps between segments)
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_with_segments(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
        segments_per_pass: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if num_passes == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_passes must be at least 1".to_string(),
            ));
        }

        if segments_per_pass == 0 {
            return Err(SpirographError::InvalidParameter(
                "segments_per_pass must be at least 1".to_string(),
            ));
        }

        if config.base_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "base_radius must be positive".to_string(),
            ));
        }

        Ok(RoseEngineLatheRun {
            base_config: config,
            cutting_bit,
            num_passes,
            segments_per_pass,
            radius_step: 0.0,
            phase_shift: 0.0,
            phase_oscillations: 1.0,
            circular_phase: 0.0,
            phase_exponent: 1,
            center_x,
            center_y,
            linear_paon: None,
            passes: Vec::new(),
            segmented_lines: Vec::new(),
            generated: false,
        })
    }

    /// Create a new multi-pass rose engine lathe run with custom center position
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_with_center(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        Self::new_with_segments(config, cutting_bit, num_passes, 24, center_x, center_y)
    }

    /// Create a rose engine draperie pattern that produces identical output
    /// to the mathematical `DraperieLayer`.
    ///
    /// This configures the rose engine lathe run in concentric-ring mode with
    /// the correct rosette pattern, amplitude, phase alignment, and phase shape
    /// function so the output matches point-for-point.
    ///
    /// # Arguments
    /// * `num_rings` - Number of concentric rings (= number of passes)
    /// * `base_radius` - Centre of the ring band in mm
    /// * `radius_step` - Radial spacing between ring centres
    /// * `wave_frequency` - Number of wave undulations per revolution
    /// * `phase_shift` - Peak angular oscillation amplitude in radians
    /// * `phase_oscillations` - Number of full sinusoidal phase cycles
    /// * `resolution` - Number of points per ring
    /// * `phase_exponent` - Exponent for sin-power phase (only when circular_phase=0)
    /// * `wave_exponent` - Exponent for the wave shape (1 = sinusoidal)
    /// * `circular_phase` - Dome-shaped phase exponent (0 = disabled, 2.0 = rounded folds)
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_draperie(
        num_rings: usize,
        base_radius: f64,
        radius_step: f64,
        wave_frequency: f64,
        phase_shift: f64,
        phase_oscillations: f64,
        resolution: usize,
        phase_exponent: u32,
        wave_exponent: u32,
        circular_phase: f64,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        // Compute safe amplitude using the same logic as DraperieConfig
        let draperie_config = DraperieConfig {
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
        let amplitude = draperie_config.safe_amplitude();

        // Set up the rose engine config with base_phase for 12 o'clock alignment
        let base_phase = PI / 2.0 + PI / (2.0 * wave_frequency);
        let mut re_config = RoseEngineConfig::new(base_radius, amplitude);
        re_config.rosette = RosettePattern::Draperie {
            frequency: wave_frequency,
            wave_exponent,
        };
        re_config.resolution = resolution;
        re_config.phase = base_phase;

        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut run = Self::new_with_segments(re_config, bit, num_rings, 1, center_x, center_y)?;
        run.radius_step = radius_step;
        run.phase_shift = phase_shift;
        run.phase_oscillations = phase_oscillations;
        run.circular_phase = circular_phase;
        run.phase_exponent = phase_exponent;
        Ok(run)
    }

    /// Create a rose engine paon (peacock) pattern that produces identical output
    /// to the mathematical `PaonLayer`.
    ///
    /// This configures the rose engine lathe run in linear-pass mode, where each
    /// pass is a fan ray emanating from 6 o'clock with zigzag oscillation
    /// perpendicular to the travel direction, creating the characteristic
    /// peacock-feather arch pattern.
    ///
    /// # Arguments
    /// * `num_lines` - Number of fan lines (= number of passes)
    /// * `radius` - Radius of the circular dial in mm
    /// * `amplitude` - Perpendicular oscillation amplitude
    /// * `wave_frequency` - Number of zigzag cycles per line
    /// * `phase_rate` - Phase change across the fan (controls arch band count)
    /// * `resolution` - Number of sample points per line
    /// * `n_harmonics` - Fourier harmonics for triangle-wave sharpness (0=sine)
    /// * `fan_angle` - Total angular spread of the fan in radians
    /// * `vanishing_point` - VP distance below circle bottom (fraction of diameter)
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_paon(
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
    ) -> Result<Self, SpirographError> {
        let paon_config = PaonConfig {
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

        // Set up a dummy rose engine config (the linear_paon path will bypass it)
        let re_config = RoseEngineConfig::new(radius, amplitude);
        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut run = Self::new_with_segments(re_config, bit, num_lines, 1, center_x, center_y)?;
        run.linear_paon = Some(paon_config);
        Ok(run)
    }

    /// Evaluate the phase-shape function at parameter `t`.
    ///
    /// * **dome mode** (`circular_phase > 0`):
    ///   Uses `sgn(sin(t)) · [1 − (1 − |sin(t)|)^n]` where `n = circular_phase`.
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

    /// Generate all passes of the rose engine pattern
    ///
    /// This creates multiple lathe passes, each rotated by an equal angular increment.
    /// Each pass is segmented into multiple arcs with gaps to create the characteristic
    /// guilloché mesh appearance.
    ///
    /// For patterns like diamant (sinusoidal with frequency=1), rotating the phase
    /// rotates the entire circle around the center, creating the overlapping circles
    /// pattern. For multi-lobe patterns, rotating the phase rotates the pattern itself.
    pub fn generate(&mut self) {
        self.passes.clear();
        self.segmented_lines.clear();

        // Linear paon mode: radiating lines from vanishing point
        if let Some(ref paon_cfg) = self.linear_paon {
            let r = paon_cfg.radius;
            let n = paon_cfg.num_lines;
            let nh = paon_cfg.n_harmonics;
            let diameter = 2.0 * r;

            // VP above circle top in math coords (= below circle in SVG)
            let y_vp = r + paon_cfg.vanishing_point * diameter;
            let y_crit = (r * r / y_vp).min(r);
            let angle_max = ((r * r - y_crit * y_crit).sqrt() / (y_vp - y_crit)).atan();
            let dist_near = y_vp - r;

            for i in 0..n {
                let frac = if n > 1 {
                    i as f64 / (n - 1) as f64
                } else {
                    0.5
                };

                let angle = -angle_max + 2.0 * angle_max * frac;
                let tan_a = angle.tan();

                // Negative |sin| phase offset → arches open UPWARD (M-shape)
                let line_phase =
                    -2.0 * PI * paon_cfg.fan_angle * (PI * paon_cfg.phase_rate * frac).sin().abs();

                let mut line_points = Vec::with_capacity(paon_cfg.resolution + 1);

                for j in 0..=paon_cfg.resolution {
                    let t_frac = j as f64 / paon_cfg.resolution as f64;

                    let y = -r + diameter * t_frac;
                    let x_base = (y_vp - y) * tan_a;
                    let dist = y_vp - y;

                    let theta =
                        2.0 * PI * paon_cfg.wave_frequency * (dist / dist_near).ln() + line_phase;
                    let offset = paon_cfg.amplitude * paon_wave_fn(theta, nh);

                    let x = x_base + offset;

                    if x * x + y * y <= r * r {
                        line_points.push(Point2D::new(self.center_x + x, self.center_y + y));
                    }
                }

                if line_points.len() >= 2 {
                    self.segmented_lines.push(line_points);
                }
            }

            self.generated = true;
            return;
        }

        let rotation_step = 2.0 * PI / (self.num_passes as f64);

        for i in 0..self.num_passes {
            let mut pass_config = self.base_config.clone();

            if self.radius_step != 0.0 {
                // Concentric ring mode: vary base_radius and optionally oscillate phase.
                // Rings are centred around the original base_radius.
                let offset = (i as f64) - ((self.num_passes - 1) as f64) / 2.0;
                pass_config.base_radius = self.base_config.base_radius + offset * self.radius_step;
                // Sinusoidal phase oscillation: peaks sway back and forth across
                // the ring stack, creating the classic draperie fold effect.
                // Uses the configurable phase shape function (dome or sin^e).
                let phase_t =
                    2.0 * PI * self.phase_oscillations * (i as f64) / (self.num_passes as f64);
                pass_config.phase =
                    self.base_config.phase + self.phase_shift * self.phase_shape_fn(phase_t);
            } else {
                // Phase-rotation mode (default): rotate the pattern for each pass.
                let rotation = (i as f64) * rotation_step;
                pass_config.phase = self.base_config.phase + rotation;
            }

            // Create and generate the lathe for this pass
            if let Ok(mut lathe) = RoseEngineLathe::new_with_center(
                pass_config,
                self.cutting_bit.clone(),
                self.center_x,
                self.center_y,
            ) {
                lathe.generate();

                // Get the complete circular path from this pass
                let rendered = lathe.rendered_output();
                if !rendered.lines.is_empty() && !rendered.lines[0].is_empty() {
                    let complete_path = &rendered.lines[0];

                    // Segment this path into multiple arcs with gaps
                    self.segment_path(complete_path);
                }

                self.passes.push(lathe);
            }
        }

        self.generated = true;
    }

    /// Segment a complete circular path into multiple arcs with gaps
    fn segment_path(&mut self, path: &[Point2D]) {
        if path.is_empty() || self.segments_per_pass == 0 {
            return;
        }

        // Special case: segments_per_pass=1 means draw the complete path without gaps
        if self.segments_per_pass == 1 {
            self.segmented_lines.push(path.to_vec());
            return;
        }

        let total_points = path.len();

        // Calculate points per segment
        // Each segment takes up a fraction of the circle with a gap
        // For visual effect: 70% drawing, 30% gap
        let draw_ratio = 0.7;
        let points_per_cycle = total_points / self.segments_per_pass;
        let draw_points = (points_per_cycle as f64 * draw_ratio) as usize;

        for seg_idx in 0..self.segments_per_pass {
            let start_idx = seg_idx * points_per_cycle;
            let end_idx = (start_idx + draw_points).min(total_points);

            if start_idx < total_points && end_idx > start_idx {
                let segment: Vec<Point2D> = path[start_idx..end_idx].to_vec();
                if !segment.is_empty() {
                    self.segmented_lines.push(segment);
                }
            }
        }
    }

    /// Export combined pattern to SVG format
    ///
    /// # Arguments
    /// * `filename` - Output SVG file path
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if !self.generated {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        use svg::node::element::{path::Data, Path};
        use svg::Document;

        // Use segmented lines instead of complete passes
        let all_lines = &self.segmented_lines;

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for line in all_lines {
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

        // Add each segmented line
        for line in all_lines.iter() {
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

        svg::save(filename, &document).map_err(|e| {
            SpirographError::ExportError(format!("Failed to save SVG file '{}': {}", filename, e))
        })
    }

    /// Get the number of passes
    pub fn num_passes(&self) -> usize {
        self.num_passes
    }

    /// Get reference to individual passes
    pub fn passes(&self) -> &[RoseEngineLathe] {
        &self.passes
    }

    /// Get reference to the segmented lines (the generated pattern curves)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.segmented_lines
    }
}
