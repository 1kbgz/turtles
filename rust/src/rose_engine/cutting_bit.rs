use std::f64::consts::PI;
use crate::common::Point2D;

/// Shape of the cutting bit
#[derive(Debug, Clone)]
pub enum BitShape {
    /// V-shaped bit with specified angle (in degrees)
    VShaped {
        /// Angle of the V in degrees (e.g., 30, 60, 90)
        angle: f64,
    },
    
    /// Flat/straight bit
    Flat,
    
    /// Round/ball bit
    Round,
    
    /// Elliptical bit
    Elliptical {
        /// Ratio of major to minor axis
        aspect_ratio: f64,
    },
    
    /// Custom bit shape defined by profile points
    Custom {
        /// Profile points from center to edge (normalized 0-1)
        profile: Vec<Point2D>,
    },
}

/// Configuration for the cutting bit/tool
#[derive(Debug, Clone)]
pub struct CuttingBit {
    /// Shape of the bit
    pub shape: BitShape,
    
    /// Width/diameter of the bit in mm
    pub width: f64,
    
    /// Depth/height of the bit in mm (for 3D operations)
    pub depth: f64,
}

impl CuttingBit {
    /// Create a new V-shaped bit
    ///
    /// # Arguments
    /// * `angle` - Angle of the V in degrees (e.g., 30.0 for a 30° V-bit)
    /// * `width` - Width of the bit at the surface in mm
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::CuttingBit;
    /// 
    /// let bit = CuttingBit::v_shaped(30.0, 1.0);
    /// ```
    pub fn v_shaped(angle: f64, width: f64) -> Self {
        let depth = width / 2.0 / (angle.to_radians() / 2.0).tan();
        CuttingBit {
            shape: BitShape::VShaped { angle },
            width,
            depth,
        }
    }
    
    /// Create a new flat bit
    ///
    /// # Arguments
    /// * `width` - Width of the bit in mm
    /// * `depth` - Depth of the bit in mm
    pub fn flat(width: f64, depth: f64) -> Self {
        CuttingBit {
            shape: BitShape::Flat,
            width,
            depth,
        }
    }
    
    /// Create a new round/ball bit
    ///
    /// # Arguments
    /// * `diameter` - Diameter of the ball in mm
    pub fn round(diameter: f64) -> Self {
        CuttingBit {
            shape: BitShape::Round,
            width: diameter,
            depth: diameter / 2.0,
        }
    }
    
    /// Create a new elliptical bit
    ///
    /// # Arguments
    /// * `width` - Major axis width in mm
    /// * `aspect_ratio` - Ratio of major to minor axis
    pub fn elliptical(width: f64, aspect_ratio: f64) -> Self {
        CuttingBit {
            shape: BitShape::Elliptical { aspect_ratio },
            width,
            depth: width / aspect_ratio / 2.0,
        }
    }
    
    /// Create a custom bit from profile points
    ///
    /// # Arguments
    /// * `profile` - Profile points from center (0,0) to edge, normalized to 0-1 range
    /// * `width` - Width scaling factor in mm
    pub fn custom(profile: Vec<Point2D>, width: f64) -> Self {
        // Find maximum depth from profile
        let max_depth = profile.iter()
            .map(|p| p.y.abs())
            .fold(0.0f64, |a, b| a.max(b));
        
        CuttingBit {
            shape: BitShape::Custom { profile },
            width,
            depth: max_depth * width,
        }
    }
    
    /// Calculate the cross-sectional profile of the bit
    /// Returns points that define the bit's cross-section
    ///
    /// # Arguments
    /// * `num_points` - Number of points to generate for the profile
    ///
    /// # Returns
    /// Vector of points defining the bit profile from -width/2 to +width/2
    pub fn cross_section(&self, num_points: usize) -> Vec<Point2D> {
        let mut points = Vec::with_capacity(num_points);
        let half_width = self.width / 2.0;
        
        match &self.shape {
            BitShape::VShaped { angle } => {
                // V-shaped profile: two straight lines meeting at a point
                let angle_rad = angle.to_radians();
                for i in 0..num_points {
                    let t = (i as f64) / ((num_points - 1) as f64);
                    let x = -half_width + t * self.width;
                    let y = x.abs() / (angle_rad / 2.0).tan();
                    points.push(Point2D::new(x, y));
                }
            }
            
            BitShape::Flat => {
                // Flat profile: horizontal line
                for i in 0..num_points {
                    let t = (i as f64) / ((num_points - 1) as f64);
                    let x = -half_width + t * self.width;
                    points.push(Point2D::new(x, 0.0));
                }
            }
            
            BitShape::Round => {
                // Round profile: semicircle
                let radius = self.width / 2.0;
                for i in 0..num_points {
                    let t = (i as f64) / ((num_points - 1) as f64);
                    let x = -half_width + t * self.width;
                    let y = (radius * radius - x * x).max(0.0).sqrt();
                    points.push(Point2D::new(x, y));
                }
            }
            
            BitShape::Elliptical { aspect_ratio } => {
                // Elliptical profile: half ellipse
                let a = half_width;
                let b = half_width / aspect_ratio;
                for i in 0..num_points {
                    let t = (i as f64) / ((num_points - 1) as f64);
                    let x = -half_width + t * self.width;
                    let y = b * (1.0 - (x / a).powi(2)).max(0.0).sqrt();
                    points.push(Point2D::new(x, y));
                }
            }
            
            BitShape::Custom { profile } => {
                // Custom profile: interpolate from provided points
                for i in 0..num_points {
                    let t = (i as f64) / ((num_points - 1) as f64);
                    let x = -half_width + t * self.width;
                    
                    // Find corresponding y from profile
                    let normalized_x = (x + half_width) / self.width;
                    let y = self.interpolate_profile(profile, normalized_x);
                    
                    points.push(Point2D::new(x, y * self.width));
                }
            }
        }
        
        points
    }
    
    /// Helper function to interpolate a value from the custom profile
    fn interpolate_profile(&self, profile: &[Point2D], x: f64) -> f64 {
        if profile.is_empty() {
            return 0.0;
        }
        
        if x <= 0.0 {
            return profile[0].y;
        }
        
        if x >= 1.0 {
            return profile[profile.len() - 1].y;
        }
        
        // Find the two points to interpolate between
        for i in 0..profile.len() - 1 {
            if profile[i].x <= x && x <= profile[i + 1].x {
                let t = (x - profile[i].x) / (profile[i + 1].x - profile[i].x);
                return profile[i].y * (1.0 - t) + profile[i + 1].y * t;
            }
        }
        
        profile[profile.len() - 1].y
    }
    
    /// Calculate the footprint of the bit at a given position and angle
    ///
    /// # Arguments
    /// * `center` - Center position of the bit
    /// * `angle` - Angle of the tool path at this point (radians)
    /// * `num_points` - Number of points to generate for each edge
    ///
    /// # Returns
    /// Tuple of (left_edge, right_edge) points defining the cut boundaries
    pub fn footprint(
        &self,
        center: Point2D,
        angle: f64,
        _num_points: usize,
    ) -> (Vec<Point2D>, Vec<Point2D>) {
        let mut left_edge = Vec::new();
        let mut right_edge = Vec::new();
        
        let half_width = self.width / 2.0;
        
        // Perpendicular to the tool path
        let perp_angle = angle + PI / 2.0;
        let perp_x = perp_angle.cos();
        let perp_y = perp_angle.sin();
        
        // For a simple approximation, create edges offset by half_width
        // perpendicular to the path direction
        left_edge.push(Point2D::new(
            center.x - perp_x * half_width,
            center.y - perp_y * half_width,
        ));
        
        right_edge.push(Point2D::new(
            center.x + perp_x * half_width,
            center.y + perp_y * half_width,
        ));
        
        (left_edge, right_edge)
    }
}

impl Default for CuttingBit {
    fn default() -> Self {
        CuttingBit::v_shaped(60.0, 0.5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_v_shaped_bit() {
        let bit = CuttingBit::v_shaped(60.0, 2.0);
        assert_eq!(bit.width, 2.0);
        assert!(bit.depth > 0.0);
        
        match bit.shape {
            BitShape::VShaped { angle } => assert_eq!(angle, 60.0),
            _ => panic!("Should be VShaped"),
        }
    }
    
    #[test]
    fn test_flat_bit() {
        let bit = CuttingBit::flat(1.0, 0.5);
        assert_eq!(bit.width, 1.0);
        assert_eq!(bit.depth, 0.5);
    }
    
    #[test]
    fn test_round_bit() {
        let bit = CuttingBit::round(2.0);
        assert_eq!(bit.width, 2.0);
        assert_eq!(bit.depth, 1.0);
    }
    
    #[test]
    fn test_cross_section_v_shaped() {
        let bit = CuttingBit::v_shaped(90.0, 2.0);
        let profile = bit.cross_section(10);
        assert_eq!(profile.len(), 10);
        
        // Center point should be deepest
        let center = &profile[profile.len() / 2];
        assert!(center.y > 0.0);
    }
    
    #[test]
    fn test_cross_section_flat() {
        let bit = CuttingBit::flat(2.0, 0.5);
        let profile = bit.cross_section(5);
        
        // All y values should be 0 for flat bit
        for point in profile {
            assert_eq!(point.y, 0.0);
        }
    }
    
    #[test]
    fn test_cross_section_round() {
        let bit = CuttingBit::round(2.0);
        let profile = bit.cross_section(10);
        
        // Center should be deepest
        let center = &profile[profile.len() / 2];
        assert!((center.y - 1.0).abs() < 0.01);
        
        // Edges should be at surface (y = 0)
        assert!(profile[0].y < 0.01);
        assert!(profile[profile.len() - 1].y < 0.01);
    }
    
    #[test]
    fn test_default_bit() {
        let bit = CuttingBit::default();
        assert_eq!(bit.width, 0.5);
        match bit.shape {
            BitShape::VShaped { angle } => assert_eq!(angle, 60.0),
            _ => panic!("Default should be VShaped"),
        }
    }
}
