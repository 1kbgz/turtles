use std::f64::consts::PI;

// Re-export common types for backward compatibility
pub use crate::common::{
    clock_to_cartesian, validate_radius, ExportConfig, Point2D, Point3D, SpirographError,
};

use crate::common::{validate_finite, validate_non_negative, validate_positive, MAX_ELEMENTS};

/// Validates the parameters shared by all three spirograph variants.
///
/// Factored out so the horizontal, vertical and spherical constructors cannot
/// drift apart: previously only `HorizontalSpirograph` checked `point_distance`,
/// and none of them rejected NaN, a zero `resolution`, or a `rotations *
/// resolution` product large enough to overflow.
fn validate_spirograph_params(
    outer_radius: f64,
    radius_ratio: f64,
    point_distance: f64,
    rotations: usize,
    resolution: usize,
    center_x: f64,
    center_y: f64,
) -> Result<(), SpirographError> {
    // A spirograph is a *layer*, not a watch case, so it gets a permissive
    // positivity check rather than the 26-44 mm dial rule. That rule is
    // enforced where it belongs - on the dial itself, in `GuillochePattern` and
    // `WatchFace`. Forcing it here made the `new_at_clock` / `new_at_polar`
    // sub-dial constructors useless, since no realistic sub-dial (a 6 mm
    // chronograph counter, say) could ever be built. `FlinqueLayer` already
    // documents and bypasses the same constraint for the same reason.
    validate_positive("outer_radius", outer_radius)?;
    validate_finite("radius_ratio", radius_ratio)?;
    validate_non_negative("point_distance", point_distance)?;
    validate_finite("point_distance", point_distance)?;
    validate_finite("center_x", center_x)?;
    validate_finite("center_y", center_y)?;

    if radius_ratio <= 0.0 || radius_ratio >= 1.0 {
        return Err(SpirographError::InvalidParameter(
            "radius_ratio must be between 0 and 1 (exclusive)".to_string(),
        ));
    }

    if rotations == 0 {
        return Err(SpirographError::InvalidParameter(
            "rotations must be at least 1".to_string(),
        ));
    }

    if resolution == 0 {
        return Err(SpirographError::InvalidParameter(
            "resolution must be at least 1".to_string(),
        ));
    }

    match rotations.checked_mul(resolution) {
        Some(total) if total <= MAX_ELEMENTS => Ok(()),
        _ => Err(SpirographError::InvalidParameter(format!(
            "rotations * resolution must not exceed {}, got {} * {}",
            MAX_ELEMENTS, rotations, resolution
        ))),
    }
}

/// Horizontal Spirograph - Traditional hypotrochoid/epitrochoid patterns
#[derive(Debug, Clone)]
pub struct HorizontalSpirograph {
    pub outer_radius: f64,   // R - fixed circle radius
    pub radius_ratio: f64,   // r/R - rolling circle radius as a fraction of R
    pub point_distance: f64, // d/r - pen offset as a fraction of the rolling circle radius
    pub rotations: usize,    // Number of rotations/revolutions
    pub resolution: usize,   // Points per revolution
    pub center_x: f64,       // X coordinate of center point
    pub center_y: f64,       // Y coordinate of center point
    points: Vec<Point2D>,    // Generated points
}

impl HorizontalSpirograph {
    /// Create a new horizontal spirograph centered at origin
    pub fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
    ) -> Result<Self, SpirographError> {
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            0.0,
            0.0,
        )
    }

    /// Create a new horizontal spirograph with a custom center point
    pub fn new_with_center(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        validate_spirograph_params(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
        )?;

        Ok(HorizontalSpirograph {
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
            points: Vec::new(),
        })
    }

    /// Create a spirograph positioned at a given angle and distance from origin
    /// angle is in radians, distance is in mm
    pub fn new_at_polar(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let center_x = distance * angle.cos();
        let center_y = distance * angle.sin();
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
        )
    }

    /// Create a spirograph positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `outer_radius` - Outer circle radius
    /// * `radius_ratio` - Inner/outer radius ratio
    /// * `point_distance` - Pen offset as a fraction of the rolling circle's
    ///   radius (0 = centre, 1 = on the rim). Values above 1 place the pen
    ///   outside the rolling circle, which is still a valid curve.
    /// * `rotations` - Number of rotations
    /// * `resolution` - Points per revolution
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
        )
    }

    /// Generate the spirograph pattern points
    pub fn generate(&mut self) -> &Vec<Point2D> {
        let inner_radius = self.outer_radius * self.radius_ratio;
        let outer_r = self.outer_radius;
        // `point_distance` is a FRACTION of the rolling circle's radius, matching
        // the `radius_ratio` convention, not an absolute millimetre value. Using
        // it raw made the pen offset ~0.6 mm against a 9.5 mm arm - a 6% wobble -
        // so every spirograph degenerated into a plain circle.
        let d = self.point_distance * inner_radius;

        let total_points = self.rotations * self.resolution;
        self.points.clear();
        self.points.reserve(total_points);

        for i in 0..total_points {
            let t = 2.0 * PI * (i as f64) / (self.resolution as f64);

            // Hypotrochoid formula
            let x = (outer_r - inner_radius) * t.cos()
                + d * (((outer_r - inner_radius) / inner_radius) * t).cos();
            let y = (outer_r - inner_radius) * t.sin()
                - d * (((outer_r - inner_radius) / inner_radius) * t).sin();

            // Apply center offset
            self.points
                .push(Point2D::new(x + self.center_x, y + self.center_y));
        }

        &self.points
    }

    /// Get the generated points
    pub fn points(&self) -> &Vec<Point2D> {
        &self.points
    }

    /// Export pattern as SVG
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        svg_export::export_svg(filename, &self.points, self.outer_radius)
            .map_err(|e| SpirographError::ExportError(format!("SVG export failed: {}", e)))
    }

    /// Export pattern as STL with depth
    pub fn to_stl(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        stl::export_stl(filename, &self.points, config)
            .map_err(|e| SpirographError::ExportError(format!("STL export failed: {}", e)))
    }

    /// Export pattern as STEP (placeholder - requires full STEP implementation)
    pub fn to_step(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        step::export_step(filename, &self.points, config)
            .map_err(|e| SpirographError::ExportError(format!("STEP export failed: {}", e)))
    }
}

/// Vertical Spirograph - Spirograph patterns with vertical wave modulation
#[derive(Debug, Clone)]
pub struct VerticalSpirograph {
    pub outer_radius: f64,
    pub radius_ratio: f64,
    pub point_distance: f64,
    pub rotations: usize,
    pub resolution: usize,
    pub wave_amplitude: f64, // Vertical wave amplitude
    pub wave_frequency: f64, // Vertical wave frequency
    pub center_x: f64,       // X coordinate of center point
    pub center_y: f64,       // Y coordinate of center point
    points: Vec<Point2D>,
}

impl VerticalSpirograph {
    /// Create a new vertical spirograph centered at origin
    pub fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
    ) -> Result<Self, SpirographError> {
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            wave_amplitude,
            wave_frequency,
            0.0,
            0.0,
        )
    }

    /// Create a new vertical spirograph with a custom center point
    pub fn new_with_center(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        validate_spirograph_params(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
        )?;
        validate_non_negative("wave_amplitude", wave_amplitude)?;
        validate_finite("wave_frequency", wave_frequency)?;

        Ok(VerticalSpirograph {
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            wave_amplitude,
            wave_frequency,
            center_x,
            center_y,
            points: Vec::new(),
        })
    }

    /// Create a spirograph positioned at a given angle and distance from origin
    pub fn new_at_polar(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let center_x = distance * angle.cos();
        let center_y = distance * angle.sin();
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            wave_amplitude,
            wave_frequency,
            center_x,
            center_y,
        )
    }

    /// Create a spirograph positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        wave_amplitude: f64,
        wave_frequency: f64,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            wave_amplitude,
            wave_frequency,
            center_x,
            center_y,
        )
    }

    /// Generate the vertical spirograph pattern
    pub fn generate(&mut self) -> &Vec<Point2D> {
        let inner_radius = self.outer_radius * self.radius_ratio;
        let outer_r = self.outer_radius;
        // `point_distance` is a FRACTION of the rolling circle's radius, matching
        // the `radius_ratio` convention, not an absolute millimetre value. Using
        // it raw made the pen offset ~0.6 mm against a 9.5 mm arm - a 6% wobble -
        // so every spirograph degenerated into a plain circle.
        let d = self.point_distance * inner_radius;

        let total_points = self.rotations * self.resolution;
        self.points.clear();
        self.points.reserve(total_points);

        for i in 0..total_points {
            let t = 2.0 * PI * (i as f64) / (self.resolution as f64);

            // Base hypotrochoid
            let base_x = (outer_r - inner_radius) * t.cos()
                + d * (((outer_r - inner_radius) / inner_radius) * t).cos();
            let base_y = (outer_r - inner_radius) * t.sin()
                - d * (((outer_r - inner_radius) / inner_radius) * t).sin();

            // Add vertical wave modulation
            let wave = self.wave_amplitude * (self.wave_frequency * t).sin();
            let x = base_x + self.center_x;
            let y = base_y + wave + self.center_y;

            self.points.push(Point2D::new(x, y));
        }

        &self.points
    }

    pub fn points(&self) -> &Vec<Point2D> {
        &self.points
    }

    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        svg_export::export_svg(filename, &self.points, self.outer_radius)
            .map_err(|e| SpirographError::ExportError(format!("SVG export failed: {}", e)))
    }

    pub fn to_stl(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        stl::export_stl(filename, &self.points, config)
            .map_err(|e| SpirographError::ExportError(format!("STL export failed: {}", e)))
    }

    pub fn to_step(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        step::export_step(filename, &self.points, config)
            .map_err(|e| SpirographError::ExportError(format!("STEP export failed: {}", e)))
    }
}

/// Spherical Spirograph - 3D spirograph patterns projected onto a spherical surface
#[derive(Debug, Clone)]
pub struct SphericalSpirograph {
    pub outer_radius: f64,
    pub radius_ratio: f64,
    pub point_distance: f64,
    pub rotations: usize,
    pub resolution: usize,
    pub dome_height: f64,    // Height of the dome
    pub center_x: f64,       // X coordinate of center point
    pub center_y: f64,       // Y coordinate of center point
    points_2d: Vec<Point2D>, // 2D projection
    points_3d: Vec<Point3D>, // 3D points on sphere
}

impl SphericalSpirograph {
    /// Create a new spherical spirograph centered at origin
    pub fn new(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        dome_height: f64,
    ) -> Result<Self, SpirographError> {
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            dome_height,
            0.0,
            0.0,
        )
    }

    /// Create a new spherical spirograph with a custom center point
    pub fn new_with_center(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        dome_height: f64,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        validate_spirograph_params(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            center_x,
            center_y,
        )?;

        // `generate()` derives `sphere_radius = (R^2 + h^2) / (2h)`. At h == 0
        // that is infinite, and the subsequent `z = sphere_radius * cos(..) -
        // (sphere_radius - h)` evaluates to `inf - inf = NaN`, poisoning every
        // 3D point. A negative height silently inverts the dome.
        validate_positive("dome_height", dome_height)?;

        Ok(SphericalSpirograph {
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            dome_height,
            center_x,
            center_y,
            points_2d: Vec::new(),
            points_3d: Vec::new(),
        })
    }

    /// Create a spirograph positioned at a given angle and distance from origin
    pub fn new_at_polar(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        dome_height: f64,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let center_x = distance * angle.cos();
        let center_y = distance * angle.sin();
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            dome_height,
            center_x,
            center_y,
        )
    }

    /// Create a spirograph positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn new_at_clock(
        outer_radius: f64,
        radius_ratio: f64,
        point_distance: f64,
        rotations: usize,
        resolution: usize,
        dome_height: f64,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(
            outer_radius,
            radius_ratio,
            point_distance,
            rotations,
            resolution,
            dome_height,
            center_x,
            center_y,
        )
    }

    /// Minimum distance to prevent division by zero in spherical projection
    const MIN_RADIUS: f64 = 0.0001;

    /// Generate the spherical spirograph pattern
    pub fn generate(&mut self) -> &Vec<Point3D> {
        let inner_radius = self.outer_radius * self.radius_ratio;
        let outer_r = self.outer_radius;
        // `point_distance` is a FRACTION of the rolling circle's radius, matching
        // the `radius_ratio` convention, not an absolute millimetre value. Using
        // it raw made the pen offset ~0.6 mm against a 9.5 mm arm - a 6% wobble -
        // so every spirograph degenerated into a plain circle.
        let d = self.point_distance * inner_radius;

        let total_points = self.rotations * self.resolution;
        self.points_2d.clear();
        self.points_3d.clear();
        self.points_2d.reserve(total_points);
        self.points_3d.reserve(total_points);

        // Calculate sphere radius for dome projection
        let sphere_radius =
            (outer_r * outer_r + self.dome_height * self.dome_height) / (2.0 * self.dome_height);

        for i in 0..total_points {
            let t = 2.0 * PI * (i as f64) / (self.resolution as f64);

            // Base hypotrochoid in 2D
            let x_2d = (outer_r - inner_radius) * t.cos()
                + d * (((outer_r - inner_radius) / inner_radius) * t).cos();
            let y_2d = (outer_r - inner_radius) * t.sin()
                - d * (((outer_r - inner_radius) / inner_radius) * t).sin();

            // Apply center offset for 2D
            self.points_2d
                .push(Point2D::new(x_2d + self.center_x, y_2d + self.center_y));

            // Project onto sphere.
            //
            // `radius_from_center` can exceed `sphere_radius` when the traced
            // point reaches beyond the dome's own radius (the ratio peaks when
            // dome_height == outer_radius). `asin` is only defined on [-1, 1]
            // and returns NaN outside it, so clamp before projecting; the
            // clamped value corresponds to the dome's equator.
            let radius_from_center = (x_2d * x_2d + y_2d * y_2d).sqrt();
            let angle_from_top = (radius_from_center / sphere_radius).clamp(-1.0, 1.0).asin();

            let z = sphere_radius * angle_from_top.cos() - (sphere_radius - self.dome_height);
            let xy_scale =
                sphere_radius * angle_from_top.sin() / radius_from_center.max(Self::MIN_RADIUS);

            let x_3d = x_2d * xy_scale + self.center_x;
            let y_3d = y_2d * xy_scale + self.center_y;

            self.points_3d.push(Point3D::new(x_3d, y_3d, z));
        }

        &self.points_3d
    }

    pub fn points_2d(&self) -> &Vec<Point2D> {
        &self.points_2d
    }

    pub fn points_3d(&self) -> &Vec<Point3D> {
        &self.points_3d
    }

    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if self.points_2d.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        svg_export::export_svg(filename, &self.points_2d, self.outer_radius)
            .map_err(|e| SpirographError::ExportError(format!("SVG export failed: {}", e)))
    }

    pub fn to_stl(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points_3d.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        stl::export_stl_3d(filename, &self.points_3d, config)
            .map_err(|e| SpirographError::ExportError(format!("STL export failed: {}", e)))
    }

    pub fn to_step(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if self.points_3d.is_empty() {
            return Err(SpirographError::ExportError(
                "No points generated. Call generate() first.".to_string(),
            ));
        }

        step::export_step_3d(filename, &self.points_3d, config)
            .map_err(|e| SpirographError::ExportError(format!("STEP export failed: {}", e)))
    }
}

/// Module for SVG export
mod svg_export {
    use super::*;
    use crate::common::compute_bounds;
    use ::svg::node::element::path::Data;
    use ::svg::node::element::Path;
    use ::svg::Document;

    pub fn export_svg(
        filename: &str,
        points: &[Point2D],
        radius: f64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if points.is_empty() {
            return Err("No points to export".into());
        }

        let mut data = Data::new().move_to((points[0].x, points[0].y));

        for point in points.iter().skip(1) {
            data = data.line_to((point.x, point.y));
        }

        // Note: Not closing the path to avoid an unwanted line back to start
        // data = data.close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 0.1)
            .set("d", data);

        // Fit the canvas to the actual geometry. This was a fixed
        // `radius * 2.5` half-extent, i.e. a canvas 5x the nominal radius, so a
        // 53mm curve was drawn on a 190mm page and appeared as a speck. Every
        // other module fits its bounds; `radius` is kept only as a floor so a
        // degenerate curve still gets a sensible page.
        let lines = [points.to_vec()];
        let (min_x, min_y, max_x, max_y) =
            compute_bounds(&lines).map_err(|e| -> Box<dyn std::error::Error> { Box::new(e) })?;

        let margin = 5.0;
        let width = (max_x - min_x).max(radius * 0.1) + 2.0 * margin;
        let height = (max_y - min_y).max(radius * 0.1) + 2.0 * margin;

        let document = Document::new()
            .set("viewBox", (min_x - margin, min_y - margin, width, height))
            .set("width", format!("{}mm", width))
            .set("height", format!("{}mm", height))
            .add(path);

        ::svg::save(filename, &document)?;
        Ok(())
    }
}

/// Module for STL export
mod stl {
    use super::*;
    use stl_io::{Normal, Triangle, Vertex};

    pub fn export_stl(
        filename: &str,
        points: &[Point2D],
        config: &ExportConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple 3D extrusion from 2D points
        let mut triangles = Vec::new();

        // Create base surface at z=0
        // Create groove surface at z=-depth
        let depth = config.depth;
        let num_points = points.len();

        // For each line segment in the path, create a rectangular groove
        for i in 0..num_points {
            let p1 = points[i];
            let p2 = points[(i + 1) % num_points];

            // Create vertices for the groove
            let v1_top = Vertex::new([p1.x as f32, p1.y as f32, 0.0]);
            let v2_top = Vertex::new([p2.x as f32, p2.y as f32, 0.0]);
            let v1_bottom = Vertex::new([p1.x as f32, p1.y as f32, -depth as f32]);
            let v2_bottom = Vertex::new([p2.x as f32, p2.y as f32, -depth as f32]);

            // Create triangles for the groove sides
            let normal = Normal::new([0.0, 0.0, 1.0]);

            // Top face (pointing up)
            triangles.push(Triangle {
                normal,
                vertices: [v1_top, v2_top, v1_bottom],
            });
            triangles.push(Triangle {
                normal,
                vertices: [v2_top, v2_bottom, v1_bottom],
            });
        }

        let mut file = std::fs::File::create(filename)?;
        stl_io::write_stl(&mut file, triangles.iter())?;
        Ok(())
    }

    pub fn export_stl_3d(
        filename: &str,
        points: &[Point3D],
        config: &ExportConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Similar to 2D but uses 3D points directly
        let mut triangles = Vec::new();
        let depth = config.depth;
        let num_points = points.len();

        for i in 0..num_points {
            let p1 = points[i];
            let p2 = points[(i + 1) % num_points];

            let v1_top = Vertex::new([p1.x as f32, p1.y as f32, p1.z as f32]);
            let v2_top = Vertex::new([p2.x as f32, p2.y as f32, p2.z as f32]);
            let v1_bottom = Vertex::new([p1.x as f32, p1.y as f32, (p1.z - depth) as f32]);
            let v2_bottom = Vertex::new([p2.x as f32, p2.y as f32, (p2.z - depth) as f32]);

            let normal = Normal::new([0.0, 0.0, 1.0]);

            triangles.push(Triangle {
                normal,
                vertices: [v1_top, v2_top, v1_bottom],
            });
            triangles.push(Triangle {
                normal,
                vertices: [v2_top, v2_bottom, v1_bottom],
            });
        }

        let mut file = std::fs::File::create(filename)?;
        stl_io::write_stl(&mut file, triangles.iter())?;
        Ok(())
    }
}

/// STEP export adapters.
///
/// The actual ISO-10303-21 serialisation lives in `crate::step`; these wrappers
/// only lift the 2D/3D point lists into the polyline form it expects.
mod step {
    use super::*;
    use crate::common::Point3D;

    pub fn export_step(
        filename: &str,
        points: &[Point2D],
        _config: &ExportConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path: Vec<Point3D> = points.iter().map(|p| Point3D::new(p.x, p.y, 0.0)).collect();
        crate::step::write_step_polylines(filename, "Spirograph Pattern", &[path])?;
        Ok(())
    }

    pub fn export_step_3d(
        filename: &str,
        points: &[Point3D],
        _config: &ExportConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        crate::step::write_step_polylines(
            filename,
            "Spherical Spirograph Pattern",
            &[points.to_vec()],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `new_at_clock` / `new_at_polar` exist to place a spirograph as a
    /// sub-dial, which naturally wants a small radius (a 6 mm chronograph
    /// counter). The 26-44 mm watch-case rule belongs on the dial itself, not
    /// on the elements drawn inside it - `FlinqueLayer` already documents and
    /// bypasses it for exactly this reason.
    #[test]
    fn test_subdial_sized_spirographs_are_allowed() {
        assert!(
            HorizontalSpirograph::new_at_clock(6.0, 0.75, 0.6, 5, 360, 3, 0, 12.0).is_ok(),
            "6mm chronograph sub-dial must be constructible"
        );
        assert!(
            VerticalSpirograph::new_at_polar(8.0, 0.6, 0.5, 5, 360, 2.0, 5.0, 0.0, 12.0).is_ok()
        );
        assert!(SphericalSpirograph::new_at_polar(9.0, 0.75, 0.6, 5, 360, 10.0, 0.0, 12.0).is_ok());

        // Still rejects genuinely nonsensical radii.
        assert!(HorizontalSpirograph::new_at_clock(0.0, 0.75, 0.6, 5, 360, 3, 0, 12.0).is_err());
        assert!(HorizontalSpirograph::new_at_clock(-5.0, 0.75, 0.6, 5, 360, 3, 0, 12.0).is_err());
        assert!(
            HorizontalSpirograph::new_at_clock(f64::NAN, 0.75, 0.6, 5, 360, 3, 0, 12.0).is_err()
        );
    }

    /// The watch-case rule still applies to the dial itself.
    #[test]
    fn test_dial_radius_still_constrained() {
        use crate::guilloche::GuillochePattern;
        assert!(GuillochePattern::new(38.0).is_ok());
        assert!(GuillochePattern::new(50.0).is_err());
        assert!(GuillochePattern::new(6.0).is_err());
    }

    #[test]
    fn test_validate_radius() {
        assert!(validate_radius(26.0).is_ok());
        assert!(validate_radius(35.0).is_ok());
        assert!(validate_radius(44.0).is_ok());
        assert!(validate_radius(25.9).is_err());
        assert!(validate_radius(44.1).is_err());
    }

    #[test]
    fn test_horizontal_spirograph_creation() {
        let spiro = HorizontalSpirograph::new(40.0, 0.75, 0.6, 50, 360);
        assert!(spiro.is_ok());

        // A spirograph is a layer, not a watch case: an out-of-case radius is
        // no longer rejected here (the 26-44 mm rule lives on the dial - see
        // `test_dial_radius_still_constrained`), but a non-physical one is.
        assert!(HorizontalSpirograph::new(50.0, 0.75, 0.6, 50, 360).is_ok());
        assert!(HorizontalSpirograph::new(0.0, 0.75, 0.6, 50, 360).is_err());
        assert!(HorizontalSpirograph::new(-40.0, 0.75, 0.6, 50, 360).is_err());
        assert!(HorizontalSpirograph::new(f64::NAN, 0.75, 0.6, 50, 360).is_err());
    }

    #[test]
    fn test_horizontal_spirograph_generate() {
        let mut spiro = HorizontalSpirograph::new(40.0, 0.75, 0.6, 50, 360).unwrap();
        let points = spiro.generate();
        assert_eq!(points.len(), 50 * 360);
    }

    #[test]
    fn test_vertical_spirograph_creation() {
        let spiro = VerticalSpirograph::new(35.0, 0.6, 0.5, 30, 360, 2.0, 5.0);
        assert!(spiro.is_ok());
    }

    #[test]
    fn test_spherical_spirograph_creation() {
        let spiro = SphericalSpirograph::new(38.0, 0.7, 0.4, 40, 360, 5.0);
        assert!(spiro.is_ok());
    }

    #[test]
    fn test_point_2d() {
        let p = Point2D::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_point_3d() {
        let p = Point3D::new(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }
}
