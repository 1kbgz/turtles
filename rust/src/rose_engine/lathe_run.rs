use crate::common::{Point2D, SpirographError};
use crate::diamant::DiamantConfig;
use crate::draperie::DraperieConfig;
use crate::flinque::FlinqueConfig;
use crate::huiteight::HuitEightConfig;
use crate::limacon::LimaconConfig;
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

    /// Optional diamant (diamond) configuration.
    /// When set, `generate()` produces circles tangent to centre, matching
    /// the mathematical `DiamantLayer` point-for-point.
    circular_diamant: Option<DiamantConfig>,

    /// Optional limacon configuration.
    /// When set, `generate()` produces limaçon polar curves, matching
    /// the mathematical `LimaconLayer` point-for-point.
    polar_limacon: Option<LimaconConfig>,

    /// Optional flinque (engine-turned) configuration.
    /// When set, `generate()` produces concentric chevron rings, matching
    /// the mathematical `FlinqueLayer` point-for-point.
    concentric_flinque: Option<FlinqueConfig>,

    /// Optional huit-eight (figure-eight) configuration.
    /// When set, `generate()` produces lemniscate curves passing through
    /// the centre, matching the mathematical `HuitEightLayer` point-for-point.
    circular_huiteight: Option<HuitEightConfig>,

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
            circular_diamant: None,
            polar_limacon: None,
            concentric_flinque: None,
            circular_huiteight: None,
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

    /// Create a rose engine diamant (diamond) pattern that produces identical
    /// output to the mathematical `DiamantLayer`.
    ///
    /// ## Physical model
    ///
    /// On a physical rose engine the diamant pattern is produced with a **round
    /// eccentric cam** (a perfect circle mounted off-centre on the spindle).
    /// The cam has eccentricity *e = R* (equal to the desired circle radius),
    /// so the spindle displacement follows a pure sinusoidal of frequency 1:
    ///
    ///   d(θ) = R · sin(θ + φ)
    ///
    /// This is equivalent to `RosettePattern::Sinusoidal { frequency: 1.0 }`
    /// with `base_radius = 0` and `amplitude = 2R`.  The polar curve
    /// ρ = 2R sin(θ + φ) is *geometrically* a circle of radius R centred at
    /// distance R from the origin, tangent to the centre.
    ///
    /// ## Why the Cartesian parameterisation is used
    ///
    /// The polar form ρ = 2R sin(θ + φ) traces the same geometric circle but
    /// visits each point *twice* per revolution (once with positive ρ, once
    /// reflected through the origin when ρ < 0).  The angular speed along the
    /// circle is non-uniform.  The mathematical `DiamantLayer` uses the
    /// standard Cartesian circle parameterisation (x = cx + R cos t,
    /// y = cy + R sin t) which visits each point exactly once with uniform
    /// arc-length spacing.
    ///
    /// Both produce the **same set of circles**; only the point sampling
    /// differs.  For 1-to-1 point matching with `DiamantLayer`, this
    /// constructor uses the Cartesian parameterisation directly.
    ///
    /// # Arguments
    /// * `num_circles` – Number of circles (= number of lathe passes)
    /// * `circle_radius` – Radius of each individual circle
    /// * `resolution` – Number of points per circle
    /// * `center_x` / `center_y` – Pattern centre
    pub fn new_diamant(
        num_circles: usize,
        circle_radius: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        let diamant_config = DiamantConfig {
            num_circles,
            circle_radius,
            resolution,
        };

        // The equivalent rose engine setup:
        //   rosette = Sinusoidal { frequency: 1 }
        //   base_radius ≈ 0  (eccentricity = circle_radius)
        //   amplitude  = 2 * circle_radius
        // We use a small positive base_radius to satisfy the constructor
        // constraint, but the actual generation bypasses the lathe path.
        let re_config = RoseEngineConfig::new(circle_radius, circle_radius);
        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut run = Self::new_with_segments(re_config, bit, num_circles, 1, center_x, center_y)?;
        run.circular_diamant = Some(diamant_config);
        Ok(run)
    }

    /// Create a rose engine limaçon pattern that produces identical output
    /// to the mathematical `LimaconLayer`.
    ///
    /// ## Physical model
    ///
    /// On a physical rose engine the limaçon is the *natural* curve cut by a
    /// **round eccentric rosette** (sinusoidal cam, frequency 1).  As the
    /// spindle rotates, the work-piece is displaced sinusoidally toward and
    /// away from the fixed cutting tool, producing the polar curve:
    ///
    ///   ρ(θ) = base_radius + amplitude · sin(θ + φ)
    ///
    /// Multiple passes at different phase offsets (φ_i = 2πi/N) create the
    /// overlapping limaçon mesh.
    ///
    /// This constructor simply wraps the standard phase-rotation mode with
    /// `RosettePattern::Sinusoidal { frequency: 1.0 }`.  The output matches
    /// `LimaconLayer` point-for-point.
    ///
    /// # Arguments
    /// * `num_curves` – Number of curves (= number of lathe passes)
    /// * `base_radius` – Base radius (limaçon *a* parameter)
    /// * `amplitude` – Sinusoidal amplitude (limaçon *b* parameter)
    /// * `resolution` – Number of points per curve
    /// * `center_x` / `center_y` – Pattern centre
    pub fn new_limacon(
        num_curves: usize,
        base_radius: f64,
        amplitude: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        let mut re_config = RoseEngineConfig::new(base_radius, amplitude);
        re_config.rosette = RosettePattern::Sinusoidal { frequency: 1.0 };
        re_config.resolution = resolution;

        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let run = Self::new_with_segments(re_config, bit, num_curves, 1, center_x, center_y)?;
        // No special fields needed – the standard phase-rotation generate()
        // with Sinusoidal{freq=1} already produces exact limaçon curves.
        Ok(run)
    }

    /// Create a rose engine flinqué (engine-turned) pattern that produces
    /// identical output to the mathematical `FlinqueLayer`.
    ///
    /// ## Physical model
    ///
    /// On a physical rose engine the flinqué sunburst is produced with a
    /// **multi-lobe rosette** having *n* lobes (one per petal) and a
    /// **secondary sinusoidal rosette** for fine ripple texture.  The lathe
    /// makes multiple concentric-ring passes (radius-step mode), each at a
    /// different base radius from the inner to the outer edge of the dial.
    ///
    /// ### Primary rosette
    ///
    /// The physical cam profile is |sin(n θ / 2)|, creating *n* identical
    /// rounded lobes.  This is the same as `RosettePattern::MultiLobe` with
    /// the DC offset and scaling adjusted to match the flinqué amplitude
    /// convention:
    ///
    ///   r = base_r + A · |sin(n θ / 2)|
    ///
    /// The MultiLobe rosette displacement maps to [-1, 1]:
    ///   d_ML(θ) = |sin(n θ / 2)| · 2 − 1
    ///
    /// Setting base_radius_RE = base_r + A/2 and amplitude_RE = A/2 gives:
    ///   r = (base_r + A/2) + (A/2)(2|sin| − 1) = base_r + A|sin|  ✓
    ///
    /// ### Secondary rosette (ripple)
    ///
    /// A sinusoidal rosette at frequency n · wave_frequency / 2 with
    /// amplitude 0.05 · A adds the fine radial texture:
    ///
    ///   ripple = 0.05 · A · sin(n · wave_frequency · θ / 2)
    ///
    /// ### Why exact 1-to-1 matching requires direct computation
    ///
    /// The mathematical `FlinqueLayer` spaces its concentric rings uniformly
    /// between `inner_radius` and `outer_radius` with a half-step offset from
    /// each edge, and skips rings whose base radius is below a safe minimum.
    /// The standard rose engine radius-step mode centres rings symmetrically
    /// around `base_radius`, producing a different set of radii.  For
    /// point-for-point matching, this constructor reproduces the `FlinqueLayer`
    /// ring-spacing logic directly.
    ///
    /// # Arguments
    /// * `radius` – Outer radius of the sunburst
    /// * `num_petals` – Number of chevron peaks per ring (lobe count)
    /// * `num_waves` – Number of concentric rings
    /// * `wave_amplitude` – Chevron amplitude (depth of the V peaks)
    /// * `wave_frequency` – Fine ripple frequency multiplier
    /// * `inner_radius_ratio` – Inner radius as fraction of outer radius
    /// * `center_x` / `center_y` – Pattern centre
    pub fn new_flinque(
        radius: f64,
        num_petals: usize,
        num_waves: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        inner_radius_ratio: f64,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        let flinque_config = FlinqueConfig {
            num_petals,
            num_waves,
            wave_amplitude,
            wave_frequency,
            inner_radius_ratio,
        };

        // The equivalent rose engine setup:
        //   primary rosette  = MultiLobe { lobes: num_petals }
        //   secondary rosette = Sinusoidal { frequency: num_petals * wave_frequency / 2 }
        //   base_radius_RE = base_r + wave_amplitude / 2  (per ring)
        //   amplitude_RE   = wave_amplitude / 2
        //   secondary_amp  = 0.05 * wave_amplitude
        //   concentric ring mode (radius_step)
        let re_config = RoseEngineConfig::new(radius, wave_amplitude / 2.0);
        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut run = Self::new_with_segments(re_config, bit, num_waves, 1, center_x, center_y)?;
        run.concentric_flinque = Some(flinque_config);
        // Store the outer radius for generation
        run.base_config.base_radius = radius;
        Ok(run)
    }

    /// Create a rose engine huit-eight (figure-eight) pattern that produces
    /// identical output to the mathematical `HuitEightLayer`.
    ///
    /// ## Physical model
    ///
    /// On a physical rose engine the huit-eight pattern is produced with a
    /// **figure-eight cam** (lemniscate-shaped) mounted on the spindle.
    /// As the spindle rotates, the cam displacement traces the lemniscate
    /// of Bernoulli:
    ///
    ///   x(t) = a · cos(t) / (1 + sin²(t))
    ///   y(t) = a · sin(t) · cos(t) / (1 + sin²(t))
    ///
    /// where `a` = `scale` is the half-width of the figure-eight.  The curve
    /// passes through the origin twice per revolution, creating a smooth
    /// figure-eight that is tangent to itself at the centre.
    ///
    /// Multiple passes at different angular rotations create the overlapping
    /// lemniscate mesh.
    ///
    /// ## Why the Cartesian parameterisation is used
    ///
    /// The existing `RosettePattern::HuitEight` displacement function
    /// `sin(n·θ) · cos(θ/2)` produces a modulated radial curve, not a true
    /// lemniscate.  For point-for-point matching with `HuitEightLayer`, this
    /// constructor uses the Cartesian lemniscate parameterisation directly.
    ///
    /// # Arguments
    /// * `num_curves` – Number of figure-eight curves (= number of passes)
    /// * `scale` – Half-width of each lemniscate
    /// * `resolution` – Number of points per curve
    /// * `center_x` / `center_y` – Pattern centre
    /// * `num_clusters` – Group curves into N clusters (0 = uniform)
    /// * `cluster_spread` – Angular spread per cluster in radians (0 = auto)
    pub fn new_huiteight(
        num_curves: usize,
        scale: f64,
        resolution: usize,
        center_x: f64,
        center_y: f64,
        num_clusters: usize,
        cluster_spread: f64,
    ) -> Result<Self, SpirographError> {
        let he_config = HuitEightConfig {
            num_curves,
            scale,
            resolution,
            num_clusters,
            cluster_spread,
        };

        let re_config = RoseEngineConfig::new(scale, scale);
        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut run = Self::new_with_segments(re_config, bit, num_curves, 1, center_x, center_y)?;
        run.circular_huiteight = Some(he_config);
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

        // ── Diamant mode: concentric circles tangent to centre ────────
        if let Some(ref diamant_cfg) = self.circular_diamant {
            let r = diamant_cfg.circle_radius;
            let n = diamant_cfg.num_circles;
            let res = diamant_cfg.resolution;
            let angle_step = 2.0 * PI / (n as f64);

            for i in 0..n {
                let rotation_angle = (i as f64) * angle_step;
                let circle_cx = self.center_x + r * rotation_angle.cos();
                let circle_cy = self.center_y + r * rotation_angle.sin();

                let mut circle_points = Vec::with_capacity(res + 1);
                for j in 0..=res {
                    let t = (j as f64) / (res as f64);
                    let theta = 2.0 * PI * t;
                    circle_points.push(Point2D::new(
                        circle_cx + r * theta.cos(),
                        circle_cy + r * theta.sin(),
                    ));
                }
                self.segmented_lines.push(circle_points);
            }

            self.generated = true;
            return;
        }

        // ── Huit-eight mode: lemniscate (figure-eight) curves ─────────
        if let Some(ref he_cfg) = self.circular_huiteight {
            let n = he_cfg.num_curves;
            let a = he_cfg.scale;
            let res = he_cfg.resolution;

            // Build rotation angles (matches HuitEightLayer::generate exactly)
            let rotations: Vec<f64> = if he_cfg.num_clusters > 0 && he_cfg.num_clusters < n {
                let nc = he_cfg.num_clusters;
                let curves_per_cluster = n / nc;
                let remainder = n % nc;
                let sector = 2.0 * PI / (nc as f64);
                let spread = if he_cfg.cluster_spread > 0.0 {
                    he_cfg.cluster_spread
                } else {
                    sector * 0.5
                };

                let mut rots = Vec::with_capacity(n);
                for k in 0..nc {
                    let cluster_center = (k as f64) * sector;
                    let count = curves_per_cluster + if k < remainder { 1 } else { 0 };
                    for c in 0..count {
                        let t = if count > 1 {
                            (c as f64) / ((count - 1) as f64) - 0.5
                        } else {
                            0.0
                        };
                        rots.push(cluster_center + t * spread);
                    }
                }
                rots
            } else {
                let angle_step = 2.0 * PI / (n as f64);
                (0..n).map(|i| (i as f64) * angle_step).collect()
            };

            for rot in &rotations {
                let cos_rot = rot.cos();
                let sin_rot = rot.sin();

                let mut pts = Vec::with_capacity(res + 1);
                for j in 0..=res {
                    let t = 2.0 * PI * (j as f64) / (res as f64);
                    let sin_t = t.sin();
                    let cos_t = t.cos();
                    let denom = 1.0 + sin_t * sin_t;
                    let lx = a * cos_t / denom;
                    let ly = a * sin_t * cos_t / denom;

                    // Rotate and translate
                    pts.push(Point2D::new(
                        self.center_x + lx * cos_rot - ly * sin_rot,
                        self.center_y + lx * sin_rot + ly * cos_rot,
                    ));
                }
                self.segmented_lines.push(pts);
            }

            self.generated = true;
            return;
        }

        // ── Flinqué mode: concentric chevron rings ────────────────────
        if let Some(ref flinque_cfg) = self.concentric_flinque {
            let outer_r = self.base_config.base_radius; // stored in new_flinque
            let inner_r = outer_r * flinque_cfg.inner_radius_ratio;
            let wave_amplitude = flinque_cfg.wave_amplitude;
            let min_radius = wave_amplitude * 0.1;
            let num_petals = flinque_cfg.num_petals;
            let wave_frequency = flinque_cfg.wave_frequency;

            for ring_idx in 0..flinque_cfg.num_waves {
                let t = (ring_idx as f64 + 0.5) / flinque_cfg.num_waves as f64;
                let base_r = inner_r + (outer_r - inner_r) * t;

                if base_r < min_radius {
                    continue;
                }

                let points_per_ring = num_petals * 80;
                let mut line_points = Vec::with_capacity(points_per_ring + 1);

                for i in 0..=points_per_ring {
                    let angle = 2.0 * PI * (i as f64) / (points_per_ring as f64);
                    let petal_phase = angle * num_petals as f64 / 2.0;

                    // Primary: multi-lobe |sin| chevron
                    let wave = petal_phase.sin().abs();
                    let chevron = wave_amplitude * wave;

                    // Secondary: fine sinusoidal ripple
                    let ripple = 0.05 * wave_amplitude * (petal_phase * wave_frequency).sin();

                    let r_mod = base_r + chevron + ripple;
                    line_points.push(Point2D::new(
                        r_mod * angle.cos() + self.center_x,
                        r_mod * angle.sin() + self.center_y,
                    ));
                }

                self.segmented_lines.push(line_points);
            }

            self.generated = true;
            return;
        }

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
