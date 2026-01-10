use std::f64::consts::PI;

/// Rosette pattern type - defines how the radius modulates with angle
#[derive(Debug, Clone)]
pub enum RosettePattern {
    /// Simple circular pattern (no modulation)
    Circular,
    
    /// Elliptical pattern with major and minor axis
    Elliptical {
        /// Ratio of major axis to minor axis
        eccentricity: f64,
        /// Rotation angle of the ellipse in radians
        rotation: f64,
    },
    
    /// Sinusoidal wave pattern
    Sinusoidal {
        /// Number of wave cycles around the circle
        frequency: f64,
    },
    
    /// Multi-lobe rosette (classic rose engine pattern)
    MultiLobe {
        /// Number of lobes/petals
        lobes: usize,
    },
    
    /// Epicycloid/hypocycloid mathematical rose curve
    Epicycloid {
        /// Number of petals (for rose curve r = cos(n*θ))
        petals: usize,
    },
    
    /// Custom pattern defined by a function
    Custom {
        /// Function that takes angle (radians) and returns displacement (-1.0 to 1.0)
        /// Stored as index into a lookup table for cloning support
        table: Vec<f64>,
        /// Number of samples in the lookup table
        samples: usize,
    },
}

impl RosettePattern {
    /// Calculate the radial displacement at a given angle
    /// Returns a value typically in the range [-1.0, 1.0] that will be multiplied by amplitude
    ///
    /// # Arguments
    /// * `angle` - Angle in radians (0 to 2π)
    ///
    /// # Returns
    /// Displacement value to be scaled by amplitude
    pub fn displacement(&self, angle: f64) -> f64 {
        match self {
            RosettePattern::Circular => 0.0,
            
            RosettePattern::Elliptical { eccentricity, rotation } => {
                // Ellipse formula: r(θ) = a*b / sqrt((b*cos(θ))² + (a*sin(θ))²)
                // We want displacement, so normalize to get variation from mean
                let rotated_angle = angle - rotation;
                let a = 1.0; // major axis (normalized)
                let b = 1.0 / eccentricity; // minor axis
                
                let cos_a = rotated_angle.cos();
                let sin_a = rotated_angle.sin();
                
                let r = (a * b) / ((b * cos_a).powi(2) + (a * sin_a).powi(2)).sqrt();
                
                // Normalize so the mean is 0 and range is roughly -1 to 1
                (r - 1.0) * eccentricity
            }
            
            RosettePattern::Sinusoidal { frequency } => {
                (angle * frequency).sin()
            }
            
            RosettePattern::MultiLobe { lobes } => {
                // Multi-lobe pattern: creates pointed petals
                // Use abs(sin(n*θ/2)) for n petals
                let phase = angle * (*lobes as f64) / 2.0;
                phase.sin().abs() * 2.0 - 1.0 // Scale to -1 to 1
            }
            
            RosettePattern::Epicycloid { petals } => {
                // Rose curve: r = cos(n*θ)
                (angle * (*petals as f64)).cos()
            }
            
            RosettePattern::Custom { table, samples } => {
                // Interpolate from lookup table
                let normalized_angle = (angle % (2.0 * PI)) / (2.0 * PI);
                let index_f = normalized_angle * (*samples as f64);
                let index = index_f.floor() as usize % *samples;
                let next_index = (index + 1) % *samples;
                let t = index_f - index_f.floor();
                
                // Linear interpolation
                table[index] * (1.0 - t) + table[next_index] * t
            }
        }
    }
    
    /// Create a custom rosette pattern from a function
    ///
    /// # Arguments
    /// * `func` - Function that takes angle (0 to 2π) and returns displacement (-1.0 to 1.0)
    /// * `samples` - Number of samples to use for the lookup table (default: 1000)
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::RosettePattern;
    /// 
    /// // Create a custom sawtooth pattern
    /// let pattern = RosettePattern::from_function(
    ///     |angle| (angle / std::f64::consts::PI) % 2.0 - 1.0,
    ///     500
    /// );
    /// ```
    pub fn from_function<F>(func: F, samples: usize) -> Self 
    where
        F: Fn(f64) -> f64,
    {
        let mut table = Vec::with_capacity(samples);
        for i in 0..samples {
            let angle = (i as f64) * 2.0 * PI / (samples as f64);
            table.push(func(angle));
        }
        
        RosettePattern::Custom { table, samples }
    }
}

impl Default for RosettePattern {
    fn default() -> Self {
        RosettePattern::MultiLobe { lobes: 12 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_circular_pattern() {
        let pattern = RosettePattern::Circular;
        assert_eq!(pattern.displacement(0.0), 0.0);
        assert_eq!(pattern.displacement(PI), 0.0);
        assert_eq!(pattern.displacement(PI / 2.0), 0.0);
    }
    
    #[test]
    fn test_sinusoidal_pattern() {
        let pattern = RosettePattern::Sinusoidal { frequency: 1.0 };
        assert!(pattern.displacement(0.0).abs() < 0.0001);
        assert!((pattern.displacement(PI / 2.0) - 1.0).abs() < 0.0001);
        assert!(pattern.displacement(PI).abs() < 0.0001);
    }
    
    #[test]
    fn test_multi_lobe_pattern() {
        let pattern = RosettePattern::MultiLobe { lobes: 6 };
        let d0 = pattern.displacement(0.0);
        let d_half = pattern.displacement(PI / 6.0);
        // At 0 the pattern should be at -1 (trough)
        assert!((d0 + 1.0).abs() < 0.1);
        // At PI/6 we should be near a peak
        assert!(d_half > 0.5);
    }
    
    #[test]
    fn test_epicycloid_pattern() {
        let pattern = RosettePattern::Epicycloid { petals: 5 };
        assert!((pattern.displacement(0.0) - 1.0).abs() < 0.0001);
        // At PI/5, cos(5*PI/5) = cos(PI) = -1
        assert!((pattern.displacement(PI / 5.0) + 1.0).abs() < 0.0001);
    }
    
    #[test]
    fn test_custom_pattern() {
        let pattern = RosettePattern::from_function(|angle| angle.sin(), 100);
        assert!(pattern.displacement(0.0).abs() < 0.1);
        let d_half = pattern.displacement(PI / 2.0);
        assert!((d_half - 1.0).abs() < 0.1);
    }
    
    #[test]
    fn test_default_pattern() {
        let pattern = RosettePattern::default();
        match pattern {
            RosettePattern::MultiLobe { lobes } => assert_eq!(lobes, 12),
            _ => panic!("Default should be MultiLobe with 12 lobes"),
        }
    }
}
