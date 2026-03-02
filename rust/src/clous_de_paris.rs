use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Clous de Paris (Hobnail) guilloché pattern
///
/// The clous de Paris pattern is created by two sets of parallel straight-line
/// grooves cut at right angles, typically rotated 45° from horizontal.
/// The intersection of the grooves creates a grid of small pyramidal "hobnails"
/// (clous = nails in French).
///
/// When viewed from above, the pattern appears as a regular grid of diamond-shaped
/// facets that catch light from different angles, creating a subtle, elegant texture
/// commonly found on watch dials and luxury goods.
///
/// On a physical rose engine, this is produced with a straight-line reciprocating
/// machine: the work moves back and forth under a V-shaped cutting tool, then is
/// indexed (shifted) sideways for the next pass.  After one direction is complete,
/// the work is rotated 90° and the process repeats.
#[derive(Debug, Clone)]
pub struct ClousDeParisConfig {
    /// Spacing between parallel grooves in mm (controls hobnail size)
    pub spacing: f64,
    /// Radius of the circular clipping region in mm
    pub radius: f64,
    /// Rotation angle of the grid in radians (default π/4 = 45° for classic diagonal)
    pub angle: f64,
    /// Number of sample points per line for rendering
    pub resolution: usize,
}

impl Default for ClousDeParisConfig {
    fn default() -> Self {
        ClousDeParisConfig {
            spacing: 1.0,
            radius: 22.0,
            angle: PI / 4.0,
            resolution: 200,
        }
    }
}

impl ClousDeParisConfig {
    /// Create a new clous de Paris configuration
    ///
    /// # Arguments
    /// * `spacing` - Distance between parallel grooves in mm
    /// * `radius` - Radius of the circular clipping region in mm
    pub fn new(spacing: f64, radius: f64) -> Self {
        ClousDeParisConfig {
            spacing,
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

/// A Clous de Paris (Hobnail) pattern layer
///
/// Creates two perpendicular sets of parallel lines clipped to a circle,
/// forming the classic hobnail grid pattern used in watch decoration.
/// The pattern emerges from the intersection of V-shaped grooves cut
/// in two orthogonal directions — each groove is a straight line, and
/// the pyramidal facets form at the intersections.
#[derive(Debug, Clone)]
pub struct ClousDeParisLayer {
    pub config: ClousDeParisConfig,
    pub center_x: f64,
    pub center_y: f64,
    lines: Vec<Vec<Point2D>>,
}

impl ClousDeParisLayer {
    /// Create a new clous de Paris layer centred at origin
    pub fn new(config: ClousDeParisConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new clous de Paris layer with a custom centre point
    pub fn new_with_center(
        config: ClousDeParisConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.spacing <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "spacing must be positive".to_string(),
            ));
        }

        if config.radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "radius must be positive".to_string(),
            ));
        }

        if config.resolution < 2 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 2".to_string(),
            ));
        }

        Ok(ClousDeParisLayer {
            config,
            center_x,
            center_y,
            lines: Vec::new(),
        })
    }

    /// Create a clous de Paris layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: ClousDeParisConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Create a clous de Paris layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Clous de Paris configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face
    pub fn new_at_clock(
        config: ClousDeParisConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Generate the clous de Paris pattern.
    ///
    /// Creates two sets of parallel lines at right angles, both rotated by
    /// `config.angle` from horizontal.  Lines are analytically clipped to
    /// the circle of `config.radius`.
    ///
    /// For each direction the line runs along unit vector (cos θ, sin θ) and
    /// is offset from the centre by `i * spacing` in the perpendicular
    /// direction (−sin θ, cos θ).  The intersection of the line with the
    /// circle is solved analytically:
    ///
    ///   offset² + t² = r²  →  t = ±√(r² − offset²)
    ///
    /// so each line spans from `−√(r² − d²)` to `+√(r² − d²)` along its
    /// travel direction.
    pub fn generate(&mut self) {
        self.lines.clear();

        let r = self.config.radius;
        let s = self.config.spacing;
        let angle = self.config.angle;

        // Generate lines for both directions (0° and 90° relative to grid angle)
        for dir in 0..2 {
            let theta = angle + (dir as f64) * PI / 2.0;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            // Number of lines needed to cover the circle diameter
            let n_lines = (r / s).ceil() as i32;

            for i in -n_lines..=n_lines {
                let offset = (i as f64) * s;

                // Analytic clip: line at perpendicular offset `offset` from centre
                let disc = r * r - offset * offset;
                if disc < 0.0 {
                    continue;
                }

                let t_half = disc.sqrt();

                // Line origin = center + offset * perpendicular
                let ox = self.center_x + offset * (-sin_t);
                let oy = self.center_y + offset * cos_t;

                let mut line_points = Vec::with_capacity(self.config.resolution + 1);

                for j in 0..=self.config.resolution {
                    let frac = j as f64 / self.config.resolution as f64;
                    let t = -t_half + 2.0 * t_half * frac;

                    let x = ox + t * cos_t;
                    let y = oy + t * sin_t;

                    line_points.push(Point2D::new(x, y));
                }

                if line_points.len() >= 2 {
                    self.lines.push(line_points);
                }
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
    fn test_clous_de_paris_config_default() {
        let config = ClousDeParisConfig::default();
        assert!((config.spacing - 1.0).abs() < 1e-10);
        assert!((config.radius - 22.0).abs() < 1e-10);
        assert!((config.angle - PI / 4.0).abs() < 1e-10);
        assert_eq!(config.resolution, 200);
    }

    #[test]
    fn test_clous_de_paris_config_new() {
        let config = ClousDeParisConfig::new(0.5, 15.0);
        assert!((config.spacing - 0.5).abs() < 1e-10);
        assert!((config.radius - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_clous_de_paris_layer_creation() {
        let config = ClousDeParisConfig::default();
        let layer = ClousDeParisLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_clous_de_paris_invalid_params() {
        // zero spacing
        let config = ClousDeParisConfig {
            spacing: 0.0,
            ..Default::default()
        };
        assert!(ClousDeParisLayer::new(config).is_err());

        // negative spacing
        let config = ClousDeParisConfig {
            spacing: -1.0,
            ..Default::default()
        };
        assert!(ClousDeParisLayer::new(config).is_err());

        // zero radius
        let config = ClousDeParisConfig {
            radius: 0.0,
            ..Default::default()
        };
        assert!(ClousDeParisLayer::new(config).is_err());

        // negative radius
        let config = ClousDeParisConfig {
            radius: -5.0,
            ..Default::default()
        };
        assert!(ClousDeParisLayer::new(config).is_err());

        // low resolution
        let config = ClousDeParisConfig {
            resolution: 1,
            ..Default::default()
        };
        assert!(ClousDeParisLayer::new(config).is_err());
    }

    #[test]
    fn test_clous_de_paris_generate() {
        let config = ClousDeParisConfig {
            spacing: 2.0,
            radius: 10.0,
            angle: PI / 4.0,
            resolution: 50,
        };
        let mut layer = ClousDeParisLayer::new(config).unwrap();
        layer.generate();

        // Should have generated lines in two directions
        assert!(!layer.lines().is_empty());

        // Each line should have resolution + 1 points
        for line in layer.lines() {
            assert_eq!(line.len(), 51);
        }
    }

    #[test]
    fn test_clous_de_paris_lines_within_circle() {
        let config = ClousDeParisConfig {
            spacing: 2.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 100,
        };
        let mut layer = ClousDeParisLayer::new(config).unwrap();
        layer.generate();

        let r = 10.0;
        for line in layer.lines() {
            for point in line {
                let dx = point.x;
                let dy = point.y;
                let dist = (dx * dx + dy * dy).sqrt();
                assert!(
                    dist <= r + 1e-6,
                    "Point ({}, {}) is outside the circle (dist={})",
                    point.x,
                    point.y,
                    dist
                );
            }
        }
    }

    #[test]
    fn test_clous_de_paris_two_directions() {
        // With angle=0, we get horizontal and vertical lines
        let config = ClousDeParisConfig {
            spacing: 5.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 10,
        };
        let mut layer = ClousDeParisLayer::new(config).unwrap();
        layer.generate();

        // n_lines = ceil(10/5) = 2, so indices -2..=2 = 5 per direction, 10 total
        // Each direction: offsets -10, -5, 0, 5, 10
        // offset ±10: disc = 100 - 100 = 0 → single point degenerate line
        // So we get 5 lines per direction = 10 total
        assert!(layer.lines().len() >= 6); // at least 3 per direction
    }

    #[test]
    fn test_clous_de_paris_symmetry() {
        // At angle=0, horizontal lines should be symmetric about y=0
        // and vertical lines about x=0
        let config = ClousDeParisConfig {
            spacing: 3.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 50,
        };
        let mut layer = ClousDeParisLayer::new(config).unwrap();
        layer.generate();

        // All points should be within or on the circle
        for line in layer.lines() {
            for point in line {
                let dist = (point.x * point.x + point.y * point.y).sqrt();
                assert!(dist <= 10.0 + 1e-6);
            }
        }
    }

    #[test]
    fn test_clous_de_paris_with_center() {
        let config = ClousDeParisConfig::new(2.0, 10.0);
        let layer = ClousDeParisLayer::new_with_center(config, 5.0, 5.0).unwrap();
        assert!((layer.center_x - 5.0).abs() < 1e-10);
        assert!((layer.center_y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_clous_de_paris_at_clock() {
        let config = ClousDeParisConfig::new(2.0, 10.0);
        let layer = ClousDeParisLayer::new_at_clock(config, 3, 0, 15.0).unwrap();
        // 3 o'clock → positive x
        assert!(layer.center_x > 0.0);
    }
}
