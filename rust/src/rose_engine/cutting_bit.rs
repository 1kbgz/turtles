use std::f64::consts::PI;

/// Cutting bit shape types for rose engine lathe
/// 
/// The cutting bit determines the cross-sectional profile of the cut
/// made into the material. Different bit shapes create different visual effects.
#[derive(Debug, Clone)]
pub enum BitShape {
    /// V-shaped bit with a specified angle
    VShaped {
        /// Angle of the V in degrees (e.g., 30, 60, 90)
        angle: f64,
    },
    
    /// Flat/straight bit
    Flat,
    
    /// Round/ball-nose bit
    Round {
        /// Radius of the round bit
        radius: f64,
    },
    
    /// Elliptical bit
    Elliptical {
        /// Width of the ellipse
        width: f64,
        /// Height of the ellipse
        height: f64,
    },
    
    /// Custom bit shape defined by profile points
    Custom {
        /// Name for the custom bit
        name: String,
        /// Profile points (x, y) defining the bit cross-section
        profile: Vec<(f64, f64)>,
    },
}

/// Configuration for a cutting bit/tool
#[derive(Debug, Clone)]
pub struct CuttingBit {
    /// Shape of the cutting bit
    pub shape: BitShape,
    
    /// Width/diameter of the bit at its widest point (in mm)
    pub width: f64,
    
    /// Maximum depth/height of the bit (in mm)
    pub depth: f64,
}

impl CuttingBit {
    /// Create a new V-shaped cutting bit
    /// 
    /// # Arguments
    /// * `angle` - Angle of the V in degrees
    /// * `width` - Width at the top of the V in mm
    /// 
    /// # Example
    /// ```
    /// use turtles::rose_engine::CuttingBit;
    /// 
    /// let bit = CuttingBit::v_shaped(30.0, 1.0);
    /// ```
    pub fn v_shaped(angle: f64, width: f64) -> Self {
        // Calculate depth from angle and width
        // For a V-bit: depth = (width/2) * tan(angle/2)
        let angle_rad = angle * PI / 180.0;
        let depth = (width / 2.0) * (angle_rad / 2.0).tan();
        
        CuttingBit {
            shape: BitShape::VShaped { angle },
            width,
            depth,
        }
    }
    
    /// Create a new flat cutting bit
    /// 
    /// # Arguments
    /// * `width` - Width of the flat bit in mm
    /// * `depth` - Maximum depth of cut in mm
    pub fn flat(width: f64, depth: f64) -> Self {
        CuttingBit {
            shape: BitShape::Flat,
            width,
            depth,
        }
    }
    
    /// Create a new round/ball-nose cutting bit
    /// 
    /// # Arguments
    /// * `radius` - Radius of the ball in mm
    pub fn round(radius: f64) -> Self {
        CuttingBit {
            shape: BitShape::Round { radius },
            width: radius * 2.0,
            depth: radius,
        }
    }
    
    /// Create a new elliptical cutting bit
    /// 
    /// # Arguments
    /// * `width` - Width of the ellipse in mm
    /// * `height` - Height of the ellipse in mm
    pub fn elliptical(width: f64, height: f64) -> Self {
        CuttingBit {
            shape: BitShape::Elliptical { width, height },
            width,
            depth: height,
        }
    }
    
    /// Calculate the width of the cut at a specific depth
    /// 
    /// # Arguments
    /// * `cut_depth` - Depth of the cut (0.0 = surface, positive = deeper)
    /// 
    /// # Returns
    /// Width of the cut at the specified depth
    pub fn width_at_depth(&self, cut_depth: f64) -> f64 {
        if cut_depth <= 0.0 {
            return self.width;
        }
        
        match &self.shape {
            BitShape::VShaped { angle } => {
                if cut_depth >= self.depth {
                    return 0.0; // Beyond the bit depth
                }
                // For V-bit, width decreases linearly with depth
                // At surface: width = self.width
                // At tip: width = 0
                let angle_rad = angle * PI / 180.0;
                let half_width_at_depth = (self.depth - cut_depth) * (angle_rad / 2.0).tan();
                half_width_at_depth * 2.0
            }
            
            BitShape::Flat => {
                // Flat bit maintains constant width regardless of depth
                self.width
            }
            
            BitShape::Round { radius } => {
                // For round bit, use circle equation: x^2 + y^2 = r^2
                // Width at depth d: w = 2 * sqrt(r^2 - d^2)
                let r = *radius;
                if cut_depth >= r {
                    0.0
                } else {
                    2.0 * (r * r - cut_depth * cut_depth).sqrt()
                }
            }
            
            BitShape::Elliptical { width, height } => {
                // For ellipse: x^2/a^2 + y^2/b^2 = 1
                // Width at depth d: w = 2a * sqrt(1 - (d/b)^2)
                let a = width / 2.0;
                let b = *height;
                let ratio = cut_depth / b;
                if ratio >= 1.0 {
                    0.0
                } else {
                    2.0 * a * (1.0 - ratio * ratio).sqrt()
                }
            }
            
            BitShape::Custom { profile: _, .. } => {
                if cut_depth >= self.depth {
                    return 0.0;
                }
                // For custom bits, interpolate from profile
                // This is a simplified implementation
                self.width * (1.0 - cut_depth / self.depth)
            }
        }
    }
    
    /// Get the profile of the bit as a series of points
    /// 
    /// # Arguments
    /// * `resolution` - Number of points to generate
    /// 
    /// # Returns
    /// Vector of (x, y) points representing the bit profile
    pub fn profile_points(&self, resolution: usize) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        
        match &self.shape {
            BitShape::Custom { profile, .. } => {
                return profile.clone();
            }
            _ => {
                // Generate points based on shape
                for i in 0..=resolution {
                    let depth = (i as f64 / resolution as f64) * self.depth;
                    let half_width = self.width_at_depth(depth) / 2.0;
                    points.push((half_width, depth));
                }
            }
        }
        
        points
    }
}

impl Default for CuttingBit {
    fn default() -> Self {
        // Default to a 30-degree V-bit with 1mm width
        Self::v_shaped(30.0, 1.0)
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
    }
    
    #[test]
    fn test_flat_bit() {
        let bit = CuttingBit::flat(1.0, 0.5);
        assert_eq!(bit.width, 1.0);
        assert_eq!(bit.depth, 0.5);
    }
    
    #[test]
    fn test_round_bit() {
        let bit = CuttingBit::round(0.5);
        assert_eq!(bit.width, 1.0);
        assert_eq!(bit.depth, 0.5);
    }
    
    #[test]
    fn test_width_at_depth_flat() {
        let bit = CuttingBit::flat(2.0, 1.0);
        assert_eq!(bit.width_at_depth(0.0), 2.0);
        assert_eq!(bit.width_at_depth(0.5), 2.0);
        assert_eq!(bit.width_at_depth(1.0), 2.0);
    }
    
    #[test]
    fn test_width_at_depth_v() {
        let bit = CuttingBit::v_shaped(90.0, 2.0);
        // At surface, width should be close to 2.0
        assert!((bit.width_at_depth(0.0) - 2.0).abs() < 0.1);
        // At tip, width should be 0
        assert!(bit.width_at_depth(bit.depth) < 0.01);
    }
    
    #[test]
    fn test_profile_points() {
        let bit = CuttingBit::round(1.0);
        let points = bit.profile_points(10);
        assert_eq!(points.len(), 11); // 0..=10 is 11 points
    }
}
