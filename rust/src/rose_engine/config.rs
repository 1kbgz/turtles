use crate::rose_engine::rosette::RosettePattern;

/// Configuration for the rose engine lathe
#[derive(Debug, Clone)]
pub struct RoseEngineConfig {
    /// Primary rosette pattern
    pub rosette: RosettePattern,
    
    /// Amplitude of the rosette pattern modulation in mm
    pub amplitude: f64,
    
    /// Base radius (distance from center to nominal cutting path) in mm
    pub base_radius: f64,
    
    /// Phase offset for the rosette pattern in radians
    pub phase: f64,
    
    /// Start angle for spindle rotation in radians
    pub start_angle: f64,
    
    /// End angle for spindle rotation in radians
    pub end_angle: f64,
    
    /// Number of points to generate along the path
    pub resolution: usize,
    
    /// Optional second rosette for compound motion
    pub secondary_rosette: Option<RosettePattern>,
    
    /// Amplitude of secondary rosette if present
    pub secondary_amplitude: f64,
    
    /// Phase offset for secondary rosette
    pub secondary_phase: f64,
    
    /// Depth modulation - if true, vary cut depth with angle
    pub depth_modulation: bool,
    
    /// Depth modulation amplitude (fraction of total depth)
    pub depth_modulation_amplitude: f64,
    
    /// Depth modulation frequency (cycles per revolution)
    pub depth_modulation_frequency: f64,
}

impl RoseEngineConfig {
    /// Create a new configuration with sensible defaults
    ///
    /// # Arguments
    /// * `base_radius` - Base radius in mm
    /// * `amplitude` - Amplitude of pattern modulation in mm
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineConfig, RosettePattern};
    /// 
    /// let mut config = RoseEngineConfig::new(20.0, 2.0);
    /// config.rosette = RosettePattern::MultiLobe { lobes: 8 };
    /// ```
    pub fn new(base_radius: f64, amplitude: f64) -> Self {
        RoseEngineConfig {
            rosette: RosettePattern::default(),
            amplitude,
            base_radius,
            phase: 0.0,
            start_angle: 0.0,
            end_angle: std::f64::consts::PI * 2.0,
            resolution: 1000,
            secondary_rosette: None,
            secondary_amplitude: 0.0,
            secondary_phase: 0.0,
            depth_modulation: false,
            depth_modulation_amplitude: 0.0,
            depth_modulation_frequency: 1.0,
        }
    }
    
    /// Add a secondary rosette for compound motion
    ///
    /// # Arguments
    /// * `rosette` - Secondary rosette pattern
    /// * `amplitude` - Amplitude of secondary pattern
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineConfig, RosettePattern};
    /// 
    /// let mut config = RoseEngineConfig::new(20.0, 2.0);
    /// config.with_secondary_rosette(
    ///     RosettePattern::Sinusoidal { frequency: 3.0 },
    ///     1.0
    /// );
    /// ```
    pub fn with_secondary_rosette(&mut self, rosette: RosettePattern, amplitude: f64) {
        self.secondary_rosette = Some(rosette);
        self.secondary_amplitude = amplitude;
    }
    
    /// Enable depth modulation
    ///
    /// # Arguments
    /// * `amplitude` - Depth variation as fraction of total depth (0.0 to 1.0)
    /// * `frequency` - Number of depth cycles per revolution
    pub fn with_depth_modulation(&mut self, amplitude: f64, frequency: f64) {
        self.depth_modulation = true;
        self.depth_modulation_amplitude = amplitude;
        self.depth_modulation_frequency = frequency;
    }
    
    /// Calculate the radial position at a given angle
    ///
    /// # Arguments
    /// * `angle` - Angle in radians
    ///
    /// # Returns
    /// Radius at the given angle
    pub fn radius_at_angle(&self, angle: f64) -> f64 {
        let primary_displacement = self.rosette.displacement(angle + self.phase);
        let mut total_displacement = self.amplitude * primary_displacement;
        
        if let Some(ref secondary) = self.secondary_rosette {
            let secondary_displacement = secondary.displacement(angle + self.secondary_phase);
            total_displacement += self.secondary_amplitude * secondary_displacement;
        }
        
        self.base_radius + total_displacement
    }
    
    /// Calculate the depth at a given angle (if depth modulation is enabled)
    ///
    /// # Arguments
    /// * `angle` - Angle in radians
    /// * `base_depth` - Base depth value
    ///
    /// # Returns
    /// Modulated depth value
    pub fn depth_at_angle(&self, angle: f64, base_depth: f64) -> f64 {
        if !self.depth_modulation {
            return base_depth;
        }
        
        let modulation = (angle * self.depth_modulation_frequency).sin();
        base_depth * (1.0 + self.depth_modulation_amplitude * modulation)
    }
}

impl Default for RoseEngineConfig {
    fn default() -> Self {
        RoseEngineConfig::new(20.0, 2.0)
    }
}

/// Preset configurations for common rose engine patterns
impl RoseEngineConfig {
    /// Classic multi-lobe pattern (most common rose engine pattern)
    pub fn classic_multi_lobe(base_radius: f64, lobes: usize, amplitude: f64) -> Self {
        let mut config = RoseEngineConfig::new(base_radius, amplitude);
        config.rosette = RosettePattern::MultiLobe { lobes };
        config
    }
    
    /// Circular sunburst pattern
    pub fn sunburst(base_radius: f64, rays: usize, amplitude: f64) -> Self {
        let mut config = RoseEngineConfig::new(base_radius, amplitude);
        config.rosette = RosettePattern::MultiLobe { lobes: rays };
        config.resolution = 2000; // Higher resolution for crisp rays
        config
    }
    
    /// Wave pattern with sinusoidal modulation
    pub fn wave(base_radius: f64, frequency: f64, amplitude: f64) -> Self {
        let mut config = RoseEngineConfig::new(base_radius, amplitude);
        config.rosette = RosettePattern::Sinusoidal { frequency };
        config
    }
    
    /// Rose curve (mathematical rose pattern)
    pub fn rose_curve(base_radius: f64, petals: usize, amplitude: f64) -> Self {
        let mut config = RoseEngineConfig::new(base_radius, amplitude);
        config.rosette = RosettePattern::Epicycloid { petals };
        config
    }
    
    /// Compound pattern with two rosettes
    pub fn compound(
        base_radius: f64,
        primary_lobes: usize,
        primary_amplitude: f64,
        secondary_frequency: f64,
        secondary_amplitude: f64,
    ) -> Self {
        let mut config = RoseEngineConfig::new(base_radius, primary_amplitude);
        config.rosette = RosettePattern::MultiLobe { lobes: primary_lobes };
        config.with_secondary_rosette(
            RosettePattern::Sinusoidal { frequency: secondary_frequency },
            secondary_amplitude,
        );
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    
    #[test]
    fn test_config_new() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        assert_eq!(config.base_radius, 20.0);
        assert_eq!(config.amplitude, 2.0);
        assert_eq!(config.resolution, 1000);
    }
    
    #[test]
    fn test_config_default() {
        let config = RoseEngineConfig::default();
        assert_eq!(config.base_radius, 20.0);
        assert_eq!(config.amplitude, 2.0);
    }
    
    #[test]
    fn test_radius_at_angle() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let r0 = config.radius_at_angle(0.0);
        let r_half = config.radius_at_angle(PI);
        
        // Should be within reasonable range
        assert!(r0 >= 18.0 && r0 <= 22.0);
        assert!(r_half >= 18.0 && r_half <= 22.0);
    }
    
    #[test]
    fn test_secondary_rosette() {
        let mut config = RoseEngineConfig::new(20.0, 2.0);
        config.with_secondary_rosette(
            RosettePattern::Sinusoidal { frequency: 2.0 },
            1.0,
        );
        
        assert!(config.secondary_rosette.is_some());
        assert_eq!(config.secondary_amplitude, 1.0);
    }
    
    #[test]
    fn test_depth_modulation() {
        let mut config = RoseEngineConfig::new(20.0, 2.0);
        config.with_depth_modulation(0.5, 2.0);
        
        assert!(config.depth_modulation);
        
        let d0 = config.depth_at_angle(0.0, 1.0);
        let d_quarter = config.depth_at_angle(PI / 4.0, 1.0);
        
        // Depths should vary (sin(0) = 0, sin(2*PI/4) = sin(PI/2) = 1)
        // d0 = 1.0 * (1 + 0.5 * 0) = 1.0
        // d_quarter = 1.0 * (1 + 0.5 * sin(2*PI/4)) = 1.0 * (1 + 0.5 * 1) = 1.5
        assert!((d0 - 1.0).abs() < 0.01);
        assert!((d_quarter - 1.5).abs() < 0.01);
    }
    
    #[test]
    fn test_preset_classic_multi_lobe() {
        let config = RoseEngineConfig::classic_multi_lobe(20.0, 12, 2.0);
        assert_eq!(config.base_radius, 20.0);
        assert_eq!(config.amplitude, 2.0);
        
        match config.rosette {
            RosettePattern::MultiLobe { lobes } => assert_eq!(lobes, 12),
            _ => panic!("Should be MultiLobe"),
        }
    }
    
    #[test]
    fn test_preset_compound() {
        let config = RoseEngineConfig::compound(20.0, 8, 2.0, 3.0, 1.0);
        assert!(config.secondary_rosette.is_some());
        assert_eq!(config.secondary_amplitude, 1.0);
    }
}
