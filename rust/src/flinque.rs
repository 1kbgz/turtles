use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for radial sunburst flinqué pattern (engine-turned guilloche)
#[derive(Debug, Clone)]
pub struct FlinqueConfig {
    /// Number of radial "petals" or segments (typically 8-16)
    pub num_petals: usize,
    /// Number of concentric wave lines per petal
    pub num_waves: usize,
    /// Wave amplitude (how much the lines undulate)
    pub wave_amplitude: f64,
    /// Wave frequency (number of oscillations per line)
    pub wave_frequency: f64,
    /// Inner radius where pattern starts (as fraction of outer radius)
    pub inner_radius_ratio: f64,
}

impl Default for FlinqueConfig {
    fn default() -> Self {
        FlinqueConfig {
            num_petals: 12,
            num_waves: 60,
            wave_amplitude: 0.8,
            wave_frequency: 20.0,
            inner_radius_ratio: 0.05,
        }
    }
}

/// A flinqué (engine-turned) layer with configurable center point
#[derive(Debug, Clone)]
pub struct FlinqueLayer {
    pub config: FlinqueConfig,
    pub radius: f64,
    pub center_x: f64,
    pub center_y: f64,
    lines: Vec<Vec<Point2D>>, // Each wave line is a series of points
}

impl FlinqueLayer {
    /// Create a new flinqué layer centered at origin
    pub fn new(radius: f64, config: FlinqueConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(radius, config, 0.0, 0.0)
    }

    /// Create a new flinqué layer with a custom center point
    pub fn new_with_center(
        radius: f64,
        config: FlinqueConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        // For flinque layers, we don't validate against watch radius constraints
        // since they may be subdials or smaller elements
        if radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "radius must be positive".to_string(),
            ));
        }

        Ok(FlinqueLayer {
            config,
            radius,
            center_x,
            center_y,
            lines: Vec::new(),
        })
    }

    /// Create a flinqué layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        radius: f64,
        config: FlinqueConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = polar_to_cartesian(angle, distance);
        Self::new_with_center(radius, config, center_x, center_y)
    }

    /// Create a flinqué layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `radius` - Radius of the flinqué pattern
    /// * `config` - Flinqué configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        radius: f64,
        config: FlinqueConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(radius, config, center_x, center_y)
    }

    /// Generate the flinqué pattern lines
    /// Creates continuous concentric arcs around the entire circle.
    /// Each arc has chevron peaks that create the petal appearance.
    /// num_petals = number of chevron peaks per ring
    /// num_waves = number of concentric rings
    pub fn generate(&mut self) {
        let inner_r = self.radius * self.config.inner_radius_ratio;
        let outer_r = self.radius;

        self.lines.clear();

        // The wave amplitude is constant - same angular chevrons at all radii
        let wave_amplitude = self.config.wave_amplitude;

        // Calculate minimum radius to avoid self-intersection
        // The trough of a ring is at base_r, peak is at base_r + wave_amplitude
        // We need the trough to stay positive and rings not to cross themselves
        // A small fraction of amplitude is sufficient as the minimum
        let min_radius = wave_amplitude * 0.1;

        // Generate concentric rings (num_waves controls how many rings)
        for ring_idx in 0..self.config.num_waves {
            // Position along the radius (0 = inner, 1 = outer)
            let t = (ring_idx as f64 + 0.5) / self.config.num_waves as f64;
            let base_r = inner_r + (outer_r - inner_r) * t;

            // Skip rings that are too close to center (would self-intersect)
            if base_r < min_radius {
                continue;
            }

            let mut line_points = Vec::new();
            // More points for smoother arcs
            let points_per_ring = self.config.num_petals * 80;

            // Sweep full 360 degrees
            for i in 0..=points_per_ring {
                let angle = 2.0 * PI * (i as f64) / (points_per_ring as f64);

                // Chevron wave: creates num_petals peaks around the circle
                // Divide by 2 because |sin| has period π, so |sin(x/2)| gives correct count
                let petal_phase = angle * self.config.num_petals as f64 / 2.0;

                // Use |sin| wave: smooth rounded peaks at max, sharp V troughs at zero
                // sin goes from -1 to 1, abs(sin) goes from 0 to 1
                // This gives: sharp troughs (at 0, pi, 2pi...) and smooth peaks (at pi/2, 3pi/2...)
                let wave = petal_phase.sin().abs();

                // Constant amplitude - same chevron depth at all radii
                let chevron = wave_amplitude * wave;

                // Optional fine ripple for texture
                let ripple =
                    0.05 * wave_amplitude * (petal_phase * self.config.wave_frequency).sin();

                // Radius varies to create the wavy chevron effect
                let r_mod = base_r + chevron + ripple;

                let x = r_mod * angle.cos() + self.center_x;
                let y = r_mod * angle.sin() + self.center_y;

                line_points.push(Point2D::new(x, y));
            }

            self.lines.push(line_points);
        }
    }

    /// Get the generated lines
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flinque_config_default() {
        let config = FlinqueConfig::default();
        assert_eq!(config.num_petals, 12);
        assert_eq!(config.num_waves, 60);
        assert_eq!(config.wave_amplitude, 0.8);
        assert_eq!(config.wave_frequency, 20.0);
        assert_eq!(config.inner_radius_ratio, 0.05);
    }

    #[test]
    fn test_flinque_layer_creation() {
        let config = FlinqueConfig::default();
        let layer = FlinqueLayer::new(10.0, config);
        assert!(layer.is_ok());

        let config = FlinqueConfig::default();
        let layer_bad = FlinqueLayer::new(-1.0, config);
        assert!(layer_bad.is_err());
    }

    #[test]
    fn test_flinque_layer_with_center() {
        let config = FlinqueConfig::default();
        let layer = FlinqueLayer::new_with_center(10.0, config, 5.0, 5.0).unwrap();
        assert_eq!(layer.center_x, 5.0);
        assert_eq!(layer.center_y, 5.0);
    }

    #[test]
    fn test_flinque_layer_generate() {
        let config = FlinqueConfig {
            num_petals: 6,
            num_waves: 10,
            wave_amplitude: 0.5,
            wave_frequency: 10.0,
            inner_radius_ratio: 0.1,
        };
        let mut layer = FlinqueLayer::new(10.0, config).unwrap();
        layer.generate();
        assert!(!layer.lines().is_empty());
    }

    #[test]
    fn test_flinque_at_clock() {
        let config = FlinqueConfig::default();
        let layer = FlinqueLayer::new_at_clock(10.0, config, 3, 0, 20.0).unwrap();
        // At 3 o'clock, x should be positive, y should be ~0
        assert!(layer.center_x > 0.0);
        assert!(layer.center_y.abs() < 0.001);
    }
}
