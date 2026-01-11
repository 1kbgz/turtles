use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Limaçon guilloché pattern
///
/// The limaçon pattern is formed by drawing limaçon curves (snail shapes) in polar
/// coordinates, rotated around the center at different angles. This produces the
/// same output as a rose engine with a sinusoidal rosette of frequency 1.
///
/// The limaçon equation in polar form is: r = base_radius + amplitude * sin(θ + phase)
#[derive(Debug, Clone)]
pub struct LimaconConfig {
    /// Number of limaçon curves to draw (more = denser mesh)
    pub num_curves: usize,
    /// Base radius (distance from center when sin=0)
    pub base_radius: f64,
    /// Amplitude of the sinusoidal modulation
    pub amplitude: f64,
    /// Resolution - number of points per curve
    pub resolution: usize,
}

impl Default for LimaconConfig {
    fn default() -> Self {
        LimaconConfig {
            num_curves: 72,
            base_radius: 20.0,
            amplitude: 20.0,
            resolution: 360,
        }
    }
}

impl LimaconConfig {
    /// Create a new limaçon configuration
    ///
    /// # Arguments
    /// * `num_curves` - Number of limaçon curves to draw around the center
    /// * `base_radius` - Base radius (center of modulation)
    /// * `amplitude` - Amplitude of sinusoidal modulation
    pub fn new(num_curves: usize, base_radius: f64, amplitude: f64) -> Self {
        LimaconConfig {
            num_curves,
            base_radius,
            amplitude,
            resolution: 360,
        }
    }

    /// Set the resolution (points per curve)
    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }
}

/// A Limaçon pattern layer that creates polar-coordinate guilloché effects
///
/// This pattern is created by drawing limaçon curves (r = a + b*sin(θ)) that are
/// rotated at equal angular intervals. The overlapping curves create intersection
/// patterns similar to rose engine output with a sinusoidal rosette.
#[derive(Debug, Clone)]
pub struct LimaconLayer {
    pub config: LimaconConfig,
    pub center_x: f64,
    pub center_y: f64,
    curves: Vec<Vec<Point2D>>,
}

impl LimaconLayer {
    /// Create a new limaçon layer centered at origin
    pub fn new(config: LimaconConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new limaçon layer with a custom center point
    pub fn new_with_center(
        config: LimaconConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.base_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "base_radius must be positive".to_string(),
            ));
        }

        if config.num_curves == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_curves must be at least 1".to_string(),
            ));
        }

        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }

        Ok(LimaconLayer {
            config,
            center_x,
            center_y,
            curves: Vec::new(),
        })
    }

    /// Create a limaçon layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: LimaconConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Create a limaçon layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `config` - Limaçon configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        config: LimaconConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Generate the limaçon pattern
    ///
    /// Each curve is a limaçon: r = base_radius + amplitude * sin(θ + phase)
    /// where phase is rotated for each curve to distribute them around the center.
    /// This produces identical output to a rose engine with sinusoidal frequency=1.
    pub fn generate(&mut self) {
        self.curves.clear();

        let phase_step = 2.0 * PI / (self.config.num_curves as f64);

        for i in 0..self.config.num_curves {
            // Phase offset for this curve (equivalent to rotating the rose engine)
            let phase = (i as f64) * phase_step;

            // Generate points along this limaçon curve
            let mut curve_points = Vec::with_capacity(self.config.resolution + 1);

            for j in 0..=self.config.resolution {
                let t = (j as f64) / (self.config.resolution as f64);
                let theta = 2.0 * PI * t;

                // Limaçon in polar coordinates: r = base_radius + amplitude * sin(θ + phase)
                let r = self.config.base_radius + self.config.amplitude * (theta + phase).sin();

                // Convert to Cartesian
                let x = self.center_x + r * theta.cos();
                let y = self.center_y + r * theta.sin();

                curve_points.push(Point2D::new(x, y));
            }

            self.curves.push(curve_points);
        }
    }

    /// Get the generated curves as a vector of point vectors
    pub fn curves(&self) -> &Vec<Vec<Point2D>> {
        &self.curves
    }

    /// Get all lines for rendering (alias for curves)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.curves
    }

    /// Export the pattern to SVG format
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;

        if self.curves.is_empty() {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for curve in &self.curves {
            for point in curve {
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

        // Draw each curve
        for curve in &self.curves {
            if curve.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((curve[0].x, curve[0].y));

            for point in curve.iter().skip(1) {
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
    fn test_limacon_config_default() {
        let config = LimaconConfig::default();
        assert_eq!(config.num_curves, 72);
        assert_eq!(config.base_radius, 20.0);
        assert_eq!(config.amplitude, 20.0);
        assert_eq!(config.resolution, 360);
    }

    #[test]
    fn test_limacon_config_new() {
        let config = LimaconConfig::new(48, 15.0, 10.0);
        assert_eq!(config.num_curves, 48);
        assert_eq!(config.base_radius, 15.0);
        assert_eq!(config.amplitude, 10.0);
    }

    #[test]
    fn test_limacon_layer_creation() {
        let config = LimaconConfig::default();
        let layer = LimaconLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_limacon_layer_invalid_radius() {
        let config = LimaconConfig::new(48, -10.0, 5.0);
        let layer = LimaconLayer::new(config);
        assert!(layer.is_err());
    }

    #[test]
    fn test_limacon_layer_generate() {
        let config = LimaconConfig::new(12, 10.0, 10.0).with_resolution(36);
        let mut layer = LimaconLayer::new(config).unwrap();
        layer.generate();

        assert_eq!(layer.curves().len(), 12);
        assert_eq!(layer.curves()[0].len(), 37); // resolution + 1 for closed curve
    }

    #[test]
    fn test_limacon_passes_through_origin_when_amp_equals_base() {
        // When amplitude = base_radius, the limaçon passes through the origin
        let config = LimaconConfig::new(4, 10.0, 10.0).with_resolution(360);
        let mut layer = LimaconLayer::new(config).unwrap();
        layer.generate();

        // Each curve should pass through or very close to the origin
        for curve in layer.curves() {
            let min_dist = curve
                .iter()
                .map(|p| (p.x * p.x + p.y * p.y).sqrt())
                .fold(f64::INFINITY, f64::min);

            // The minimum distance to origin should be very small
            assert!(
                min_dist < 0.1,
                "Limaçon should pass through origin when amp=base, min_dist = {}",
                min_dist
            );
        }
    }

    #[test]
    fn test_limacon_matches_rose_engine() {
        use crate::rose_engine::{
            CuttingBit, RoseEngineConfig, RoseEngineLatheRun, RosettePattern,
        };

        // Parameters for comparison
        let num_curves = 12;
        let base_radius = 20.0;
        let amplitude = 20.0;
        let resolution = 360;

        // Create LimaconLayer
        let config =
            LimaconConfig::new(num_curves, base_radius, amplitude).with_resolution(resolution);
        let mut limacon = LimaconLayer::new(config).unwrap();
        limacon.generate();

        // Create equivalent RoseEngineLatheRun with sinusoidal frequency=1
        let mut rose_config = RoseEngineConfig::new(base_radius, amplitude);
        rose_config.rosette = RosettePattern::Sinusoidal { frequency: 1.0 };
        rose_config.resolution = resolution;

        let bit = CuttingBit::v_shaped(30.0, 0.02);
        let mut rose_run = RoseEngineLatheRun::new_with_segments(
            rose_config,
            bit,
            num_curves,
            1, // segments_per_pass=1 for complete shapes
            0.0,
            0.0,
        )
        .unwrap();
        rose_run.generate();

        // Both should have the same number of curves/lines
        let limacon_lines = limacon.lines();
        let rose_lines = rose_run.lines();

        assert_eq!(
            limacon_lines.len(),
            rose_lines.len(),
            "LimaconLayer and RoseEngineLatheRun should have same number of curves"
        );

        // Each curve should have the same number of points
        for (i, (lim_curve, rose_curve)) in limacon_lines.iter().zip(rose_lines.iter()).enumerate()
        {
            assert_eq!(
                lim_curve.len(),
                rose_curve.len(),
                "Curve {} should have same number of points",
                i
            );

            // Compare all points - they should be identical (within floating point tolerance)
            for (j, (lim_pt, rose_pt)) in lim_curve.iter().zip(rose_curve.iter()).enumerate() {
                let dist = ((lim_pt.x - rose_pt.x).powi(2) + (lim_pt.y - rose_pt.y).powi(2)).sqrt();
                assert!(
                    dist < 1e-10,
                    "Point {},{} differs: limacon=({}, {}), rose=({}, {}), dist={}",
                    i,
                    j,
                    lim_pt.x,
                    lim_pt.y,
                    rose_pt.x,
                    rose_pt.y,
                    dist
                );
            }
        }
    }
}
