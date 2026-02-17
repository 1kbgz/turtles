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

    /// Huit-Eight (Figure-Eight) - interlocking figure-eight pattern
    HuitEight {
        /// Number of lobes/figure-eights around the circle
        lobes: usize,
    },

    /// Grain-de-Riz (Rice Grain) - small elongated oval shapes in rows
    GrainDeRiz {
        /// Size of each grain (controls frequency)
        grain_size: f64,
        /// Number of grain rows
        rows: usize,
    },

    /// Draperie (Drapery) - flowing fabric/wave pattern
    /// Creates concentric wavy rings resembling draped fabric.
    /// Each ring is a sinusoidal modulation at the given frequency,
    /// optionally raised to `wave_exponent` for softer crests.
    /// Use with RoseEngineLatheRun in concentric-ring mode (radius_step > 0)
    /// for multi-ring draperie generation.
    Draperie {
        /// Wave frequency (number of undulations per revolution)
        frequency: f64,
        /// Exponent applied to the wave shape: 1 = sinusoidal (default),
        /// 3 = softer rounded crests.
        wave_exponent: u32,
    },

    /// Paon (Peacock) - peacock-feather arch pattern for linear (non-rotational) passes.
    /// Each pass is a vertical line oscillating sinusoidally. The `frequency`
    /// controls the number of arch rows across the dial height.
    Paon {
        /// Wave frequency (number of oscillation cycles across the dial height)
        frequency: f64,
    },

    /// Diamant (Diamond) - geometric diamond/checkerboard pattern
    Diamant {
        /// Number of divisions (creates diamond grid)
        divisions: usize,
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

            RosettePattern::Elliptical {
                eccentricity,
                rotation,
            } => {
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

            RosettePattern::Sinusoidal { frequency } => (angle * frequency).sin(),

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

            RosettePattern::HuitEight { lobes } => {
                // Figure-eight pattern: overlapping sinusoidal waves
                // Use sin(n*θ) * cos(θ/2) for interlocking effect
                let n = *lobes as f64;
                (angle * n).sin() * (angle / 2.0).cos()
            }

            RosettePattern::GrainDeRiz { grain_size, rows } => {
                // Rice grain: small oval shapes in concentric rows
                // Create pointed ovals using modulated sine wave
                let row_angle = angle * (*rows as f64);
                let grain_modulation = (angle / grain_size).sin();
                // Combine row pattern with grain shape
                row_angle.sin().abs() * grain_modulation
            }

            RosettePattern::Draperie {
                frequency,
                wave_exponent,
            } => {
                // Drapery pattern: sinusoidal wave with optional exponent.
                let s = (angle * frequency).sin();
                if *wave_exponent <= 1 {
                    s
                } else {
                    s.abs().powi(*wave_exponent as i32) * s.signum()
                }
            }

            RosettePattern::Paon { frequency } => {
                // Paon: simple sinusoidal displacement.
                // The actual arch pattern emerges from the linear-pass mode
                // in RoseEngineLatheRun::new_paon which varies the phase
                // across passes. Here we just provide the base sine wave.
                (angle * frequency).sin()
            }

            RosettePattern::Diamant { divisions } => {
                // Diamond pattern: checkerboard created by two perpendicular waves
                // Use combination of sine waves at different frequencies
                let n = *divisions as f64;
                let wave1 = (angle * n).sin();
                let wave2 = (angle * n + PI / 4.0).sin();
                // Create sharp diamond intersections
                (wave1.abs() + wave2.abs()) / 2.0 * 2.0 - 1.0
            }

            RosettePattern::Custom { table, samples } => {
                // Interpolate from lookup table
                let normalized_angle = angle.rem_euclid(2.0 * PI) / (2.0 * PI);
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
    fn test_draperie_pattern_range() {
        // Verify displacement values stay within [-1.0, 1.0] for various angles
        let pattern = RosettePattern::Draperie {
            frequency: 6.0,
            wave_exponent: 1,
        };

        // Test various angles
        for i in 0..100 {
            let angle = (i as f64) * 2.0 * PI / 100.0;
            let displacement = pattern.displacement(angle);
            assert!(
                displacement >= -1.0 && displacement <= 1.0,
                "Displacement {} at angle {} is out of range [-1.0, 1.0]",
                displacement,
                angle
            );
        }
    }

    #[test]
    fn test_draperie_pattern_not_constant() {
        // Verify the pattern is not flat (produces varying displacement across angles)
        let pattern = RosettePattern::Draperie {
            frequency: 6.0,
            wave_exponent: 1,
        };

        let d0 = pattern.displacement(0.0);
        let mut found_different = false;

        for i in 1..100 {
            let angle = (i as f64) * 2.0 * PI / 100.0;
            let displacement = pattern.displacement(angle);
            if (displacement - d0).abs() > 0.01 {
                found_different = true;
                break;
            }
        }

        assert!(
            found_different,
            "Pattern should vary across different angles"
        );
    }

    #[test]
    fn test_draperie_pattern_symmetry() {
        // Verify the clean sine wave formula properties
        let pattern = RosettePattern::Draperie {
            frequency: 6.0,
            wave_exponent: 1,
        };

        // At angle=0, sin(0) = 0
        let d0 = pattern.displacement(0.0);
        assert!(
            d0.abs() < 0.0001,
            "At angle 0, displacement should be ~0, got {}",
            d0
        );

        // Verify formula: sin(6*angle)
        let angle = PI / 6.0;
        let expected = (6.0 * angle).sin();
        let actual = pattern.displacement(angle);
        assert!(
            (actual - expected).abs() < 0.0001,
            "Expected {}, got {}",
            expected,
            actual
        );
    }

    #[test]
    fn test_draperie_pattern_frequency_effect() {
        // Verify that different frequency values produce different displacement curves
        let pattern1 = RosettePattern::Draperie {
            frequency: 6.0,
            wave_exponent: 1,
        };
        let pattern2 = RosettePattern::Draperie {
            frequency: 3.0,
            wave_exponent: 1,
        };

        let mut found_different = false;
        for i in 1..50 {
            let angle = (i as f64) * 2.0 * PI / 50.0;
            let d1 = pattern1.displacement(angle);
            let d2 = pattern2.displacement(angle);
            if (d1 - d2).abs() > 0.01 {
                found_different = true;
                break;
            }
        }

        assert!(
            found_different,
            "Different frequencies should produce different displacement patterns"
        );
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
