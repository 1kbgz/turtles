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

/// Validates that a radius is within the required range for watch faces (26mm-44mm)
pub fn validate_radius(radius: f64) -> Result<(), SpirographError> {
    if radius < 26.0 || radius > 44.0 {
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
