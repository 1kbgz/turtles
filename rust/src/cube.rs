use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Cube (tumbling blocks) guilloché pattern
///
/// The cube pattern creates the optical illusion of three-dimensional cubes
/// tiled across a surface.  It is produced by parallel zigzag (triangular-wave)
/// cutting passes grouped in sets of `cuts_per_group` with `gap_per_group`
/// spacings of empty gap between groups.  Alternating groups are phase-shifted
/// by half a zigzag period so that peaks nest into troughs, forming
/// interlocking diamond shapes that read as cube faces.
///
/// ### Parameters
///
/// | field             | meaning |
/// |-------------------|---------|
/// | `spacing`         | Distance between adjacent zigzag lines |
/// | `radius`          | Radius of the circular clipping region |
/// | `angle`           | Base rotation of the entire pattern in radians |
/// | `resolution`      | Number of sample points per line (kept for API compat) |
/// | `cuts_per_group`  | Number of zigzag lines per cutting group |
/// | `gap_per_group`   | Number of line-spacings of empty gap between groups |
/// | `amplitude`       | Half peak-to-trough zigzag height (0 = auto so diamonds close) |
/// | `leg_angle`       | Angle of each zigzag leg from horizontal in degrees |
#[derive(Debug, Clone)]
pub struct CubeConfig {
    /// Spacing between adjacent zigzag lines in mm
    pub spacing: f64,
    /// Radius of the circular clipping region in mm
    pub radius: f64,
    /// Base rotation angle of the pattern in radians
    pub angle: f64,
    /// Number of sample points per line (kept for API compatibility)
    pub resolution: usize,
    /// Number of zigzag lines per cutting group
    pub cuts_per_group: usize,
    /// Number of line-spacings of empty gap between groups
    pub gap_per_group: usize,
    /// Half peak-to-trough zigzag height in mm (0.0 = auto-compute so diamonds close)
    pub amplitude: f64,
    /// Angle of each zigzag leg from horizontal in degrees (smaller = flatter)
    pub leg_angle: f64,
}

impl Default for CubeConfig {
    fn default() -> Self {
        CubeConfig {
            spacing: 0.5,
            radius: 22.0,
            angle: 0.0,
            resolution: 200,
            cuts_per_group: 8,
            gap_per_group: 8,
            amplitude: 0.0,
            leg_angle: 30.0,
        }
    }
}

impl CubeConfig {
    /// Create a new cube configuration
    ///
    /// # Arguments
    /// * `spacing` - Distance between parallel lines in mm
    /// * `radius`  - Radius of the circular clipping region in mm
    pub fn new(spacing: f64, radius: f64) -> Self {
        CubeConfig {
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

/// A Cube (tumbling blocks) pattern layer
///
/// Generates parallel zigzag (triangular-wave) lines grouped in sets of
/// `cuts_per_group`, with equal-sized gaps between groups.  Alternating
/// groups are phase-shifted by half a zigzag period, creating interlocking
/// diamond-shaped uncut regions that produce the optical illusion of
/// three-dimensional cubes.
#[derive(Debug, Clone)]
pub struct CubeLayer {
    pub config: CubeConfig,
    pub center_x: f64,
    pub center_y: f64,
    lines: Vec<Vec<Point2D>>,
}

/// Find where a line segment intersects a circle centred at the origin.
/// Returns the first valid intersection point with parameter `t` in [0, 1].
fn line_circle_intersect(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Option<(f64, f64)> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let a = dx * dx + dy * dy;
    if a < 1e-12 {
        return None;
    }
    let b = 2.0 * (x1 * dx + y1 * dy);
    let c = x1 * x1 + y1 * y1 - r * r;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    let t = if (0.0..=1.0).contains(&t1) {
        t1
    } else if (0.0..=1.0).contains(&t2) {
        t2
    } else {
        return None;
    };
    Some((x1 + t * dx, y1 + t * dy))
}

/// Find both intersection points of a line segment with a circle at origin.
fn line_circle_intersect_both(x1: f64, y1: f64, x2: f64, y2: f64, r: f64) -> Vec<(f64, f64)> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let a = dx * dx + dy * dy;
    if a < 1e-12 {
        return vec![];
    }
    let b = 2.0 * (x1 * dx + y1 * dy);
    let c = x1 * x1 + y1 * y1 - r * r;
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return vec![];
    }
    let sqrt_disc = disc.sqrt();
    let t1 = (-b - sqrt_disc) / (2.0 * a);
    let t2 = (-b + sqrt_disc) / (2.0 * a);
    let mut result = Vec::new();
    if (0.0..=1.0).contains(&t1) {
        result.push((x1 + t1 * dx, y1 + t1 * dy));
    }
    if (0.0..=1.0).contains(&t2) && (t2 - t1).abs() > 1e-12 {
        result.push((x1 + t2 * dx, y1 + t2 * dy));
    }
    result
}

impl CubeLayer {
    /// Create a new cube layer centred at origin
    pub fn new(config: CubeConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new cube layer with a custom centre point
    pub fn new_with_center(
        config: CubeConfig,
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
        if config.cuts_per_group < 1 {
            return Err(SpirographError::InvalidParameter(
                "cuts_per_group must be at least 1".to_string(),
            ));
        }
        if config.gap_per_group < 1 {
            return Err(SpirographError::InvalidParameter(
                "gap_per_group must be at least 1".to_string(),
            ));
        }
        if config.leg_angle <= 0.0 || config.leg_angle >= 90.0 {
            return Err(SpirographError::InvalidParameter(
                "leg_angle must be between 0 and 90 degrees (exclusive)".to_string(),
            ));
        }
        if config.amplitude < 0.0 {
            return Err(SpirographError::InvalidParameter(
                "amplitude must be non-negative (0 = auto)".to_string(),
            ));
        }
        Ok(CubeLayer {
            config,
            center_x,
            center_y,
            lines: Vec::new(),
        })
    }

    /// Create a cube layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: CubeConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Create a cube layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Cube configuration
    /// * `hour`   - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face
    pub fn new_at_clock(
        config: CubeConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (cx, cy) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, cx, cy)
    }

    /// Generate the cube pattern.
    ///
    /// Creates parallel zigzag (triangular-wave) lines in groups of
    /// `cuts_per_group`, with equal-sized gaps between groups.  Alternating
    /// groups are phase-shifted by half a zigzag period so that peaks nest
    /// into troughs, forming interlocking diamond-shaped uncut regions.
    pub fn generate(&mut self) {
        self.lines.clear();

        let r = self.config.radius;
        let s = self.config.spacing;
        let cuts = self.config.cuts_per_group;
        let gap = self.config.gap_per_group;
        let base_angle = self.config.angle;

        // Amplitude: if 0 (auto), set so diamond tips close exactly
        let amplitude = if self.config.amplitude > 0.0 {
            self.config.amplitude
        } else {
            ((gap as f64) + 1.0) * s / 2.0
        };

        // Period from leg angle: tan(leg_angle) = 2·amplitude / half_period
        let leg_rad = self.config.leg_angle.to_radians();
        let period = 4.0 * amplitude / leg_rad.tan();
        let half_period = period / 2.0;

        // Group cycle: cuts lines on, gap spacings off
        let group_cycle = (cuts as f64 + gap as f64) * s;

        let cos_a = base_angle.cos();
        let sin_a = base_angle.sin();
        let r_sq = r * r;
        let cx = self.center_x;
        let cy = self.center_y;

        let n_groups = (r / group_cycle).ceil() as i32 + 2;

        for g in -n_groups..=n_groups {
            let group_base = (g as f64) * group_cycle;
            // Alternate groups shift phase by half a period
            let phase = if g.rem_euclid(2) == 0 { 0.0 } else { 0.5 };

            for i in 0..(cuts as i32) {
                let baseline = group_base + (i as f64) * s;

                // Early skip if the entire zigzag is outside the circle
                if baseline - amplitude > r || baseline + amplitude < -r {
                    continue;
                }

                // Zigzag vertices at every half-period (the sharp corners)
                let x_extent = r + period;
                let phase_offset = phase * period;

                let k_start = ((-x_extent + phase_offset) / half_period).floor() as i32;
                let k_end = ((x_extent + phase_offset) / half_period).ceil() as i32;

                let cap = (k_end - k_start + 1).max(0) as usize;
                let mut vertices: Vec<(f64, f64)> = Vec::with_capacity(cap);
                for k in k_start..=k_end {
                    let x = (k as f64) * half_period - phase_offset;
                    let sign = if k.rem_euclid(2) == 0 { 1.0 } else { -1.0 };
                    let y = baseline + amplitude * sign;
                    vertices.push((x, y));
                }

                // Walk the vertices, clipping each segment to the circle
                let mut current_segment: Vec<Point2D> = Vec::new();

                for v_idx in 0..vertices.len() {
                    let (x, y) = vertices[v_idx];
                    let inside = x * x + y * y <= r_sq;

                    if v_idx > 0 {
                        let (px, py) = vertices[v_idx - 1];
                        let prev_inside = px * px + py * py <= r_sq;

                        if prev_inside && !inside {
                            // Exiting circle — add intersection, close segment
                            if let Some((ix, iy)) = line_circle_intersect(px, py, x, y, r) {
                                let rx = cx + ix * cos_a - iy * sin_a;
                                let ry = cy + ix * sin_a + iy * cos_a;
                                current_segment.push(Point2D::new(rx, ry));
                            }
                            if current_segment.len() >= 2 {
                                self.lines.push(std::mem::take(&mut current_segment));
                            }
                            current_segment.clear();
                        } else if !prev_inside && inside {
                            // Entering circle — add intersection, start segment
                            if let Some((ix, iy)) = line_circle_intersect(px, py, x, y, r) {
                                let rx = cx + ix * cos_a - iy * sin_a;
                                let ry = cy + ix * sin_a + iy * cos_a;
                                current_segment.push(Point2D::new(rx, ry));
                            }
                        } else if !prev_inside && !inside {
                            // Both outside — segment may still cross circle
                            let both = line_circle_intersect_both(px, py, x, y, r);
                            if both.len() == 2 {
                                let (ix1, iy1) = both[0];
                                let (ix2, iy2) = both[1];
                                let rx1 = cx + ix1 * cos_a - iy1 * sin_a;
                                let ry1 = cy + ix1 * sin_a + iy1 * cos_a;
                                let rx2 = cx + ix2 * cos_a - iy2 * sin_a;
                                let ry2 = cy + ix2 * sin_a + iy2 * cos_a;
                                self.lines
                                    .push(vec![Point2D::new(rx1, ry1), Point2D::new(rx2, ry2)]);
                            }
                        }
                    }

                    if inside {
                        let rx = cx + x * cos_a - y * sin_a;
                        let ry = cy + x * sin_a + y * cos_a;
                        current_segment.push(Point2D::new(rx, ry));
                    }
                }

                if current_segment.len() >= 2 {
                    self.lines.push(current_segment);
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
    fn test_cube_config_default() {
        let config = CubeConfig::default();
        assert!((config.spacing - 0.5).abs() < 1e-10);
        assert!((config.radius - 22.0).abs() < 1e-10);
        assert!((config.angle - 0.0).abs() < 1e-10);
        assert_eq!(config.resolution, 200);
        assert_eq!(config.cuts_per_group, 8);
        assert_eq!(config.gap_per_group, 8);
        assert!((config.amplitude - 0.0).abs() < 1e-10);
        assert!((config.leg_angle - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_cube_config_new() {
        let config = CubeConfig::new(2.0, 15.0);
        assert!((config.spacing - 2.0).abs() < 1e-10);
        assert!((config.radius - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_cube_layer_creation() {
        let config = CubeConfig::default();
        let layer = CubeLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_cube_invalid_params() {
        // zero spacing
        let config = CubeConfig {
            spacing: 0.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // negative spacing
        let config = CubeConfig {
            spacing: -1.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // zero radius
        let config = CubeConfig {
            radius: 0.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // low resolution
        let config = CubeConfig {
            resolution: 1,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // zero cuts_per_group
        let config = CubeConfig {
            cuts_per_group: 0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // zero gap_per_group
        let config = CubeConfig {
            gap_per_group: 0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // invalid leg_angle (0)
        let config = CubeConfig {
            leg_angle: 0.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // invalid leg_angle (90)
        let config = CubeConfig {
            leg_angle: 90.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());

        // negative amplitude
        let config = CubeConfig {
            amplitude: -1.0,
            ..Default::default()
        };
        assert!(CubeLayer::new(config).is_err());
    }

    #[test]
    fn test_cube_generate() {
        let config = CubeConfig {
            spacing: 1.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 100,
            ..Default::default()
        };
        let mut layer = CubeLayer::new(config).unwrap();
        layer.generate();

        // Should have generated zigzag lines
        assert!(!layer.lines().is_empty());
        assert!(layer.lines().len() >= 2);
    }

    #[test]
    fn test_cube_lines_within_circle() {
        let config = CubeConfig {
            spacing: 3.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 200,
            ..Default::default()
        };
        let mut layer = CubeLayer::new(config).unwrap();
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
    fn test_cube_zigzag_grouping() {
        // Verify grouping: with cuts_per_group=4 and gap_per_group=4
        let config = CubeConfig {
            spacing: 2.0,
            radius: 20.0,
            angle: 0.0,
            resolution: 50,
            cuts_per_group: 4,
            gap_per_group: 4,
            ..Default::default()
        };
        let mut layer = CubeLayer::new(config).unwrap();
        layer.generate();

        // Should have generated zigzag line segments
        assert!(layer.lines().len() >= 4);
    }

    #[test]
    fn test_cube_custom_amplitude_and_angle() {
        let config = CubeConfig {
            spacing: 2.0,
            radius: 15.0,
            cuts_per_group: 4,
            gap_per_group: 4,
            amplitude: 5.0,
            leg_angle: 30.0,
            ..Default::default()
        };
        let mut layer = CubeLayer::new(config).unwrap();
        layer.generate();
        assert!(!layer.lines().is_empty());
    }

    #[test]
    fn test_cube_with_center() {
        let config = CubeConfig::new(2.0, 10.0);
        let layer = CubeLayer::new_with_center(config, 5.0, 5.0).unwrap();
        assert!((layer.center_x - 5.0).abs() < 1e-10);
        assert!((layer.center_y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_cube_at_polar() {
        let config = CubeConfig::new(2.0, 10.0);
        let layer = CubeLayer::new_at_polar(config, 0.0, 10.0).unwrap();
        assert!((layer.center_x - 10.0).abs() < 1e-6);
        assert!(layer.center_y.abs() < 1e-6);
    }

    #[test]
    fn test_cube_at_clock() {
        let config = CubeConfig::new(2.0, 10.0);
        // 3 o'clock should be positive x
        let layer = CubeLayer::new_at_clock(config, 3, 0, 10.0).unwrap();
        assert!(layer.center_x > 0.0);
    }

    #[test]
    fn test_cube_rotation() {
        use std::f64::consts::PI;
        // Different base angles should produce different patterns
        let config_0 = CubeConfig {
            spacing: 1.0,
            radius: 10.0,
            angle: 0.0,
            resolution: 20,
            ..Default::default()
        };
        let config_30 = CubeConfig {
            spacing: 1.0,
            radius: 10.0,
            angle: PI / 6.0,
            resolution: 20,
            ..Default::default()
        };

        let mut layer_0 = CubeLayer::new(config_0).unwrap();
        let mut layer_30 = CubeLayer::new(config_30).unwrap();
        layer_0.generate();
        layer_30.generate();

        // Both should have lines
        assert!(!layer_0.lines().is_empty());
        assert!(!layer_30.lines().is_empty());
    }
}
