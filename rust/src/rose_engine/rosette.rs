use std::f64::consts::PI;

/// Rosette pattern types for rose engine lathe
///
/// A rosette is a cam or pattern that modulates the radial position
/// of the cutting bit as the workpiece rotates, creating guilloché patterns.
#[derive(Debug, Clone)]
pub enum RosettePattern {
    /// Simple circular rosette (no modulation)
    Circular,

    /// Elliptical rosette with major and minor axes
    Elliptical {
        /// Major axis length
        major_axis: f64,
        /// Minor axis length
        minor_axis: f64,
    },

    /// Sinusoidal wave pattern
    Sinusoidal {
        /// Number of complete cycles
        frequency: f64,
    },

    /// Multi-lobe rosette (creates n-pointed patterns)
    MultiLobe {
        /// Number of lobes/petals
        lobes: usize,
    },

    /// Epicycloid pattern (mathematical rose curves)
    Epicycloid {
        /// Number of petals in the epicycloid
        petals: usize,
    },

    /// Custom pattern using a user-defined function
    /// The function takes an angle in radians and returns the radial displacement
    Custom {
        /// Name for the custom pattern
        name: String,
    },
}

impl RosettePattern {
    /// Calculate the radial displacement for a given angle
    ///
    /// # Arguments
    /// * `angle` - Angle in radians (0 to 2π represents one full rotation)
    /// * `amplitude` - Amplitude of the modulation
    ///
    /// # Returns
    /// Radial displacement value (typically -1.0 to 1.0, scaled by amplitude)
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::RosettePattern;
    ///
    /// let rosette = RosettePattern::MultiLobe { lobes: 6 };
    /// let displacement = rosette.calculate_displacement(0.0, 1.0);
    /// ```
    pub fn calculate_displacement(&self, angle: f64, amplitude: f64) -> f64 {
        let normalized = match self {
            RosettePattern::Circular => 0.0,

            RosettePattern::Elliptical {
                major_axis,
                minor_axis,
            } => {
                // Elliptical displacement varies with angle
                let ratio = minor_axis / major_axis;
                (1.0 - ratio) * angle.cos()
            }

            RosettePattern::Sinusoidal { frequency } => (angle * frequency).sin(),

            RosettePattern::MultiLobe { lobes } => {
                // Creates n-lobed patterns using sine wave
                // Multiply by lobes/2 to get correct number of peaks per rotation
                (angle * (*lobes as f64 / 2.0)).sin()
            }

            RosettePattern::Epicycloid { petals } => {
                // Rose curve: r = cos(k*θ)
                // For even k, you get 2k petals; for odd k, you get k petals
                let k = *petals as f64;
                (k * angle).cos()
            }

            RosettePattern::Custom { name: _ } => {
                // For custom patterns, we'll use a simple sine wave as default
                // In a real implementation, this would call a user-provided function
                angle.sin()
            }
        };

        amplitude * normalized
    }

    /// Get a descriptive name for this rosette pattern
    pub fn name(&self) -> String {
        match self {
            RosettePattern::Circular => "Circular".to_string(),
            RosettePattern::Elliptical { .. } => "Elliptical".to_string(),
            RosettePattern::Sinusoidal { frequency } => format!("Sinusoidal (freq={})", frequency),
            RosettePattern::MultiLobe { lobes } => format!("MultiLobe ({})", lobes),
            RosettePattern::Epicycloid { petals } => format!("Epicycloid ({})", petals),
            RosettePattern::Custom { name } => format!("Custom ({})", name),
        }
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
    fn test_circular_rosette() {
        let rosette = RosettePattern::Circular;
        assert_eq!(rosette.calculate_displacement(0.0, 1.0), 0.0);
        assert_eq!(rosette.calculate_displacement(PI, 1.0), 0.0);
    }

    #[test]
    fn test_sinusoidal_rosette() {
        let rosette = RosettePattern::Sinusoidal { frequency: 1.0 };
        let disp = rosette.calculate_displacement(PI / 2.0, 1.0);
        assert!((disp - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_multilobe_rosette() {
        let rosette = RosettePattern::MultiLobe { lobes: 4 };
        // At angle 0, sin(0) = 0
        let disp = rosette.calculate_displacement(0.0, 1.0);
        assert!(disp.abs() < 0.001);
    }

    #[test]
    fn test_rosette_name() {
        let rosette = RosettePattern::MultiLobe { lobes: 6 };
        assert_eq!(rosette.name(), "MultiLobe (6)");
    }

    #[test]
    fn test_default_rosette() {
        let rosette = RosettePattern::default();
        assert!(matches!(rosette, RosettePattern::MultiLobe { lobes: 12 }));
    }
}
