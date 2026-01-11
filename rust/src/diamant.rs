use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Diamant (Diamond) guilloché pattern
///
/// The diamant pattern is formed by drawing equally-sized circles that are
/// tangent to the center point, rotated around the center at different angles.
/// The overlapping circles create the characteristic diamond/mesh appearance.
#[derive(Debug, Clone)]
pub struct DiamantConfig {
    /// Number of circles to draw (more = denser mesh)
    pub num_circles: usize,
    /// Radius of each individual circle
    pub circle_radius: f64,
    /// Resolution - number of points per circle
    pub resolution: usize,
}

impl Default for DiamantConfig {
    fn default() -> Self {
        DiamantConfig {
            num_circles: 72,
            circle_radius: 20.0,
            resolution: 360,
        }
    }
}

impl DiamantConfig {
    /// Create a new diamant configuration
    ///
    /// # Arguments
    /// * `num_circles` - Number of circles to draw around the center
    /// * `circle_radius` - Radius of each individual circle
    pub fn new(num_circles: usize, circle_radius: f64) -> Self {
        DiamantConfig {
            num_circles,
            circle_radius,
            resolution: 360,
        }
    }

    /// Set the resolution (points per circle)
    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }
}

/// A Diamant pattern layer that creates the diamond guilloché effect
///
/// This pattern is created by drawing circles that are tangent to the center
/// and rotated at equal angular intervals. The overlapping circles create
/// diamond-shaped intersection patterns that radiate from the center.
#[derive(Debug, Clone)]
pub struct DiamantLayer {
    pub config: DiamantConfig,
    pub center_x: f64,
    pub center_y: f64,
    circles: Vec<Vec<Point2D>>,
}

impl DiamantLayer {
    /// Create a new diamant layer centered at origin
    pub fn new(config: DiamantConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new diamant layer with a custom center point
    pub fn new_with_center(
        config: DiamantConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.circle_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "circle_radius must be positive".to_string(),
            ));
        }

        if config.num_circles == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_circles must be at least 1".to_string(),
            ));
        }

        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }

        Ok(DiamantLayer {
            config,
            center_x,
            center_y,
            circles: Vec::new(),
        })
    }

    /// Create a diamant layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: DiamantConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Create a diamant layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `config` - Diamant configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        config: DiamantConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Generate the diamant pattern
    ///
    /// Each circle is positioned so that it is tangent to the center point.
    /// The center of each circle is at distance `circle_radius` from the origin,
    /// at an angle determined by dividing the full rotation among all circles.
    pub fn generate(&mut self) {
        self.circles.clear();

        let angle_step = 2.0 * PI / (self.config.num_circles as f64);
        let r = self.config.circle_radius;

        for i in 0..self.config.num_circles {
            // Angle for this circle's center position
            let rotation_angle = (i as f64) * angle_step;

            // Position the center of this circle at distance r from origin
            // This makes the circle tangent to the origin
            let circle_center_x = self.center_x + r * rotation_angle.cos();
            let circle_center_y = self.center_y + r * rotation_angle.sin();

            // Generate points along this circle
            let mut circle_points = Vec::with_capacity(self.config.resolution + 1);

            for j in 0..=self.config.resolution {
                let t = (j as f64) / (self.config.resolution as f64);
                let angle = 2.0 * PI * t;

                let x = circle_center_x + r * angle.cos();
                let y = circle_center_y + r * angle.sin();

                circle_points.push(Point2D::new(x, y));
            }

            self.circles.push(circle_points);
        }
    }

    /// Get the generated circles as a vector of point vectors
    pub fn circles(&self) -> &Vec<Vec<Point2D>> {
        &self.circles
    }

    /// Get all lines for rendering (alias for circles)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.circles
    }

    /// Export the pattern to SVG format
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;

        if self.circles.is_empty() {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for circle in &self.circles {
            for point in circle {
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

        // Draw each circle
        for circle in &self.circles {
            if circle.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((circle[0].x, circle[0].y));

            for point in circle.iter().skip(1) {
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
    fn test_diamant_config_default() {
        let config = DiamantConfig::default();
        assert_eq!(config.num_circles, 72);
        assert_eq!(config.circle_radius, 20.0);
        assert_eq!(config.resolution, 360);
    }

    #[test]
    fn test_diamant_config_new() {
        let config = DiamantConfig::new(48, 15.0);
        assert_eq!(config.num_circles, 48);
        assert_eq!(config.circle_radius, 15.0);
    }

    #[test]
    fn test_diamant_layer_creation() {
        let config = DiamantConfig::default();
        let layer = DiamantLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_diamant_layer_invalid_radius() {
        let config = DiamantConfig::new(48, -10.0);
        let layer = DiamantLayer::new(config);
        assert!(layer.is_err());
    }

    #[test]
    fn test_diamant_layer_generate() {
        let config = DiamantConfig::new(12, 10.0).with_resolution(36);
        let mut layer = DiamantLayer::new(config).unwrap();
        layer.generate();

        assert_eq!(layer.circles().len(), 12);
        assert_eq!(layer.circles()[0].len(), 37); // resolution + 1 for closed circle
    }

    #[test]
    fn test_diamant_circles_tangent_to_center() {
        let config = DiamantConfig::new(4, 10.0).with_resolution(360);
        let mut layer = DiamantLayer::new(config).unwrap();
        layer.generate();

        // Each circle should pass through or very close to the origin
        for circle in layer.circles() {
            let min_dist = circle
                .iter()
                .map(|p| (p.x * p.x + p.y * p.y).sqrt())
                .fold(f64::INFINITY, f64::min);

            // The minimum distance to origin should be very small (tangent point)
            assert!(
                min_dist < 0.1,
                "Circle should be tangent to origin, min_dist = {}",
                min_dist
            );
        }
    }
}
