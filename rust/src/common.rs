use std::f64::consts::PI;

/// Common validation error for spirograph and flinque parameters
#[derive(Debug, Clone, PartialEq)]
pub enum SpirographError {
    InvalidRadius(String),
    InvalidParameter(String),
    ExportError(String),
}

impl std::fmt::Display for SpirographError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SpirographError::InvalidRadius(msg) => write!(f, "Invalid radius: {}", msg),
            SpirographError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            SpirographError::ExportError(msg) => write!(f, "Export error: {}", msg),
        }
    }
}

impl std::error::Error for SpirographError {}

/// Validates that a value is finite (not NaN, not infinite).
///
/// This must be called before any ordered comparison, because every ordered
/// comparison against `f64::NAN` evaluates to `false` — so a check such as
/// `if radius <= 0.0 { return Err(..) }` silently *accepts* NaN and lets it
/// propagate all the way into exported SVG/STL/STEP geometry.
pub fn validate_finite(name: &str, value: f64) -> Result<(), SpirographError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(SpirographError::InvalidParameter(format!(
            "{} must be a finite number, got {}",
            name, value
        )))
    }
}

/// Validates that a value is finite and strictly greater than zero.
pub fn validate_positive(name: &str, value: f64) -> Result<(), SpirographError> {
    validate_finite(name, value)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(SpirographError::InvalidParameter(format!(
            "{} must be greater than 0, got {}",
            name, value
        )))
    }
}

/// Validates that a value is finite and greater than or equal to zero.
pub fn validate_non_negative(name: &str, value: f64) -> Result<(), SpirographError> {
    validate_finite(name, value)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(SpirographError::InvalidParameter(format!(
            "{} must be non-negative, got {}",
            name, value
        )))
    }
}

/// Validates that a value is finite and lies within `[min, max]` inclusive.
pub fn validate_range(name: &str, value: f64, min: f64, max: f64) -> Result<(), SpirographError> {
    validate_finite(name, value)?;
    if value >= min && value <= max {
        Ok(())
    } else {
        Err(SpirographError::InvalidParameter(format!(
            "{} must be between {} and {}, got {}",
            name, min, max, value
        )))
    }
}

/// Upper bound on the number of discrete elements (lines, rings, curves,
/// groups) any single pattern may generate.
///
/// Derived counts such as `(radius / spacing).ceil() as i32` are otherwise
/// unbounded: a `spacing` of `0` makes the quotient infinite, and Rust's `as`
/// cast *saturates* infinity to `i32::MAX`, producing a ~4.3 billion iteration
/// loop that allocates on every pass. Bounding the count converts that hang
/// into an actionable error.
pub const MAX_ELEMENTS: usize = 1_000_000;

/// Converts a derived, potentially unbounded element count into a checked
/// `usize`, rejecting non-finite, negative, and absurdly large values.
pub fn checked_element_count(name: &str, count: f64) -> Result<usize, SpirographError> {
    validate_finite(name, count)?;
    if count < 0.0 {
        return Err(SpirographError::InvalidParameter(format!(
            "{} must be non-negative, got {}",
            name, count
        )));
    }
    if count > MAX_ELEMENTS as f64 {
        return Err(SpirographError::InvalidParameter(format!(
            "{} resolves to {} elements, which exceeds the maximum of {}. \
             Increase spacing or reduce radius.",
            name, count, MAX_ELEMENTS
        )));
    }
    Ok(count as usize)
}

/// Clips a polyline to the interior of a circle, splitting it wherever it
/// leaves and re-enters.
///
/// Returns the pieces that lie inside the circle; a stroke that exits and
/// re-enters becomes two separate polylines rather than one with a chord
/// across the gap.
///
/// This is real geometric clipping, not an SVG `clip-path`. A `clip-path` only
/// hides ink in a viewer — the underlying coordinates still reach the STL and
/// STEP exports, so a dial whose pattern overflows would be *machined* outside
/// its own boundary.
pub fn clip_polyline_to_circle(
    points: &[Point2D],
    center_x: f64,
    center_y: f64,
    radius: f64,
) -> Vec<Vec<Point2D>> {
    if points.len() < 2 || !radius.is_finite() || radius <= 0.0 {
        return Vec::new();
    }

    let r2 = radius * radius;
    let inside = |p: &Point2D| {
        let dx = p.x - center_x;
        let dy = p.y - center_y;
        dx * dx + dy * dy <= r2
    };

    // Parameters t in [0,1] where the segment p0->p1 crosses the circle.
    let crossings = |p0: &Point2D, p1: &Point2D| -> (Option<f64>, Option<f64>) {
        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;
        let fx = p0.x - center_x;
        let fy = p0.y - center_y;
        let a = dx * dx + dy * dy;
        if a == 0.0 {
            return (None, None);
        }
        let b = 2.0 * (fx * dx + fy * dy);
        let c = fx * fx + fy * fy - r2;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 {
            return (None, None);
        }
        let sq = disc.sqrt();
        let t1 = (-b - sq) / (2.0 * a);
        let t2 = (-b + sq) / (2.0 * a);
        let keep = |t: f64| {
            if (0.0..=1.0).contains(&t) {
                Some(t)
            } else {
                None
            }
        };
        (keep(t1), keep(t2))
    };

    let lerp = |p0: &Point2D, p1: &Point2D, t: f64| {
        Point2D::new(p0.x + (p1.x - p0.x) * t, p0.y + (p1.y - p0.y) * t)
    };

    let mut out: Vec<Vec<Point2D>> = Vec::new();
    let mut current: Vec<Point2D> = Vec::new();

    for pair in points.windows(2) {
        let (p0, p1) = (&pair[0], &pair[1]);
        let (in0, in1) = (inside(p0), inside(p1));
        let (t1, t2) = crossings(p0, p1);

        match (in0, in1) {
            (true, true) => {
                if current.is_empty() {
                    current.push(*p0);
                }
                current.push(*p1);
            }
            (true, false) => {
                if current.is_empty() {
                    current.push(*p0);
                }
                if let Some(t) = t2.or(t1) {
                    current.push(lerp(p0, p1, t));
                }
                out.push(std::mem::take(&mut current));
            }
            (false, true) => {
                if !current.is_empty() {
                    out.push(std::mem::take(&mut current));
                }
                if let Some(t) = t1.or(t2) {
                    current.push(lerp(p0, p1, t));
                }
                current.push(*p1);
            }
            (false, false) => {
                // The chord case: both endpoints outside, but the segment may
                // still cut across the circle.
                if let (Some(ta), Some(tb)) = (t1, t2) {
                    if !current.is_empty() {
                        out.push(std::mem::take(&mut current));
                    }
                    out.push(vec![lerp(p0, p1, ta), lerp(p0, p1, tb)]);
                }
            }
        }
    }

    if !current.is_empty() {
        out.push(current);
    }
    out.retain(|seg| seg.len() >= 2);
    out
}

/// Clips a set of polylines to a circle centred on the origin.
pub fn clip_lines_to_radius(lines: &[Vec<Point2D>], radius: f64) -> Vec<Vec<Point2D>> {
    lines
        .iter()
        .flat_map(|line| clip_polyline_to_circle(line, 0.0, 0.0, radius))
        .collect()
}

/// Computes the bounding box of a set of polylines for SVG `viewBox` emission.
///
/// Returns `(min_x, min_y, max_x, max_y)`. Callers used to fold with
/// `f64::min`/`f64::max` seeded at infinity and use the result unconditionally:
/// with no points that yields a `viewBox` of `inf inf -inf -inf` and a negative
/// width, and because `f64::min` *ignores* NaN a single NaN coordinate slips
/// past the bounds only to be written into the path data as the literal `NaN`.
/// Both cases are rejected here instead.
pub fn compute_bounds(lines: &[Vec<Point2D>]) -> Result<(f64, f64, f64, f64), SpirographError> {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    let mut count = 0usize;

    for line in lines {
        for point in line {
            if !point.x.is_finite() || !point.y.is_finite() {
                return Err(SpirographError::ExportError(format!(
                    "Non-finite coordinate ({}, {}) in generated geometry; \
                     cannot compute SVG bounds.",
                    point.x, point.y
                )));
            }
            min_x = min_x.min(point.x);
            max_x = max_x.max(point.x);
            min_y = min_y.min(point.y);
            max_y = max_y.max(point.y);
            count += 1;
        }
    }

    if count == 0 {
        return Err(SpirographError::ExportError(
            "No geometry to export; the generated pattern contains no points.".to_string(),
        ));
    }

    Ok((min_x, min_y, max_x, max_y))
}

/// Validates that a radius is within the required range for watch faces (26mm-44mm)
pub fn validate_radius(radius: f64) -> Result<(), SpirographError> {
    validate_finite("Radius", radius)?;
    if !(26.0..=44.0).contains(&radius) {
        Err(SpirographError::InvalidRadius(format!(
            "Radius must be between 26mm and 44mm, got {}mm",
            radius
        )))
    } else {
        Ok(())
    }
}

/// Convert clock position (hour, minute) and distance from center to cartesian coordinates.
///
/// # Arguments
/// * `hour` - Hour position (1-12, where 12 is at the top)
/// * `minute` - Minute position (0-59)
/// * `distance` - Distance from the center of the clock face
///
/// # Returns
/// (x, y) coordinates where 12 o'clock is up (negative y in screen coords)
pub fn clock_to_cartesian(hour: u32, minute: u32, distance: f64) -> (f64, f64) {
    // Convert hour (1-12) and minute (0-59) to total minutes from 12:00
    let h = hour % 12; // 12 becomes 0
    let total_minutes = (h as f64) * 60.0 + (minute as f64);

    // Fraction of full rotation (720 minutes = 12 hours)
    let fraction = total_minutes / 720.0;

    // Angle: start at 12 o'clock (-π/2) and go clockwise
    // In screen coordinates (y down), clockwise means positive angle
    let angle = -PI / 2.0 + fraction * 2.0 * PI;

    let x = distance * angle.cos();
    let y = distance * angle.sin();

    (x, y)
}

/// Convert polar coordinates (angle, distance) to cartesian (x, y)
pub fn polar_to_cartesian(angle: f64, distance: f64) -> (f64, f64) {
    (distance * angle.cos(), distance * angle.sin())
}

/// A 2D point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Point2D { x, y }
    }
}

/// A 3D point (for spherical spirographs)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3D { x, y, z }
    }
}

/// Configuration for export formats
#[derive(Debug, Clone)]
pub struct ExportConfig {
    pub depth: f64,          // Groove/channel depth in mm
    pub base_thickness: f64, // Base plate thickness in mm
    pub tool_radius: f64,    // Tool radius compensation in mm
}

impl Default for ExportConfig {
    fn default() -> Self {
        ExportConfig {
            depth: 0.1,
            base_thickness: 2.0,
            tool_radius: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(p: &Point2D) -> f64 {
        (p.x * p.x + p.y * p.y).sqrt()
    }

    #[test]
    fn test_clip_keeps_fully_inside_line_intact() {
        let line = vec![
            Point2D::new(-5.0, 0.0),
            Point2D::new(0.0, 2.0),
            Point2D::new(5.0, 0.0),
        ];
        let out = clip_polyline_to_circle(&line, 0.0, 0.0, 10.0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].len(), 3);
    }

    #[test]
    fn test_clip_drops_fully_outside_line() {
        let line = vec![Point2D::new(50.0, 50.0), Point2D::new(60.0, 60.0)];
        assert!(clip_polyline_to_circle(&line, 0.0, 0.0, 10.0).is_empty());
    }

    #[test]
    fn test_clip_trims_to_the_boundary() {
        let line = vec![Point2D::new(0.0, 0.0), Point2D::new(100.0, 0.0)];
        let out = clip_polyline_to_circle(&line, 0.0, 0.0, 10.0);
        assert_eq!(out.len(), 1);
        let last = out[0].last().unwrap();
        assert!(
            (r(last) - 10.0).abs() < 1e-9,
            "exit point must land on the circle"
        );
    }

    /// A stroke that leaves and comes back must become two pieces, not one
    /// piece with a chord bridging the gap.
    #[test]
    fn test_clip_splits_on_re_entry() {
        let line = vec![
            Point2D::new(-9.0, 0.0),
            Point2D::new(-9.0, 20.0),
            Point2D::new(9.0, 20.0),
            Point2D::new(9.0, 0.0),
        ];
        let out = clip_polyline_to_circle(&line, 0.0, 0.0, 10.0);
        assert_eq!(out.len(), 2, "expected two disjoint pieces");
        for seg in &out {
            for p in seg {
                assert!(r(p) <= 10.0 + 1e-9);
            }
        }
    }

    /// Both endpoints outside but the segment cuts across the disc.
    #[test]
    fn test_clip_handles_chord_through_circle() {
        let line = vec![Point2D::new(-50.0, 0.0), Point2D::new(50.0, 0.0)];
        let out = clip_polyline_to_circle(&line, 0.0, 0.0, 10.0);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].len(), 2);
        assert!((r(&out[0][0]) - 10.0).abs() < 1e-9);
        assert!((r(&out[0][1]) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_clip_respects_offset_centre() {
        let line = vec![Point2D::new(20.0, 0.0), Point2D::new(20.0, 100.0)];
        let out = clip_polyline_to_circle(&line, 20.0, 0.0, 5.0);
        assert_eq!(out.len(), 1);
        let last = out[0].last().unwrap();
        assert!(((last.y - 0.0).abs() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_clip_lines_to_radius_bounds_everything() {
        let lines = vec![
            vec![Point2D::new(0.0, 0.0), Point2D::new(42.0, 0.0)],
            vec![Point2D::new(0.0, -42.0), Point2D::new(0.0, 42.0)],
        ];
        for seg in clip_lines_to_radius(&lines, 38.0) {
            for p in &seg {
                assert!(r(p) <= 38.0 + 1e-9, "point at r={} escaped the dial", r(p));
            }
        }
    }
}
