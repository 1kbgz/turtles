use super::rosette::RosettePattern;

/// Configuration for the rose engine lathe
/// 
/// This configuration defines all parameters needed to generate a guilloché pattern
/// using a virtual rose engine lathe.
#[derive(Debug, Clone)]
pub struct RoseEngineConfig {
    /// Rosette/cam pattern that modulates the radial position
    pub rosette: RosettePattern,
    
    /// Amplitude of the rosette pattern modulation (in mm)
    /// This controls how much the radial position varies
    pub amplitude: f64,
    
    /// Base radius from the center (in mm)
    /// This is the average distance from center where the pattern is cut
    pub base_radius: f64,
    
    /// Start angle for spindle rotation (in radians)
    pub start_angle: f64,
    
    /// End angle for spindle rotation (in radians)
    pub end_angle: f64,
    
    /// Number of points to generate (resolution of the pattern)
    pub resolution: usize,
    
    /// Phase offset for the rosette pattern (in radians)
    /// This shifts the pattern rotationally
    pub phase: f64,
    
    /// Depth modulation factor (0.0 = constant depth, 1.0 = full modulation)
    pub depth_modulation: f64,
    
    /// Center position X coordinate (in mm)
    pub center_x: f64,
    
    /// Center position Y coordinate (in mm)
    pub center_y: f64,
}

impl RoseEngineConfig {
    /// Create a new rose engine configuration
    /// 
    /// # Arguments
    /// * `rosette` - The rosette pattern to use
    /// * `amplitude` - Amplitude of modulation in mm
    /// * `base_radius` - Base radius from center in mm
    /// * `resolution` - Number of points to generate
    /// 
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineConfig, RosettePattern};
    /// 
    /// let config = RoseEngineConfig::new(
    ///     RosettePattern::MultiLobe { lobes: 12 },
    ///     2.0,
    ///     20.0,
    ///     1000
    /// );
    /// ```
    pub fn new(
        rosette: RosettePattern,
        amplitude: f64,
        base_radius: f64,
        resolution: usize,
    ) -> Self {
        RoseEngineConfig {
            rosette,
            amplitude,
            base_radius,
            start_angle: 0.0,
            end_angle: 2.0 * std::f64::consts::PI,
            resolution,
            phase: 0.0,
            depth_modulation: 0.0,
            center_x: 0.0,
            center_y: 0.0,
        }
    }
    
    /// Create a configuration for a classic flinqué pattern
    /// 
    /// # Arguments
    /// * `num_petals` - Number of petals/lobes
    /// * `base_radius` - Base radius in mm
    pub fn flinque(num_petals: usize, base_radius: f64) -> Self {
        Self::new(
            RosettePattern::MultiLobe { lobes: num_petals },
            0.8,
            base_radius,
            1000,
        )
    }
    
    /// Create a configuration for a sunray pattern
    /// 
    /// # Arguments
    /// * `num_rays` - Number of rays
    /// * `base_radius` - Base radius in mm
    pub fn sunray(num_rays: usize, base_radius: f64) -> Self {
        Self::new(
            RosettePattern::MultiLobe { lobes: num_rays },
            1.5,
            base_radius,
            2000,
        )
    }
    
    /// Create a configuration for a grain de riz (rice grain) pattern
    /// 
    /// # Arguments
    /// * `base_radius` - Base radius in mm
    pub fn grain_de_riz(base_radius: f64) -> Self {
        Self::new(
            RosettePattern::Elliptical {
                major_axis: 2.0,
                minor_axis: 1.0,
            },
            0.5,
            base_radius,
            800,
        )
    }
    
    /// Create a configuration for a draperie (drapery) pattern
    /// 
    /// # Arguments
    /// * `base_radius` - Base radius in mm
    pub fn draperie(base_radius: f64) -> Self {
        Self::new(
            RosettePattern::Sinusoidal { frequency: 8.0 },
            1.2,
            base_radius,
            1500,
        )
    }
    
    /// Create a configuration for a diamond pattern
    /// 
    /// # Arguments
    /// * `base_radius` - Base radius in mm
    pub fn diamant(base_radius: f64) -> Self {
        Self::new(
            RosettePattern::MultiLobe { lobes: 4 },
            1.0,
            base_radius,
            1000,
        )
    }
    
    /// Create a configuration for a clou de paris (hobnail) pattern
    /// 
    /// # Arguments
    /// * `base_radius` - Base radius in mm
    pub fn clou_de_paris(base_radius: f64) -> Self {
        Self::new(
            RosettePattern::MultiLobe { lobes: 8 },
            0.6,
            base_radius,
            1200,
        )
    }
    
    /// Set the center position
    pub fn with_center(mut self, x: f64, y: f64) -> Self {
        self.center_x = x;
        self.center_y = y;
        self
    }
    
    /// Set the phase offset
    pub fn with_phase(mut self, phase: f64) -> Self {
        self.phase = phase;
        self
    }
    
    /// Set the angular range
    pub fn with_angle_range(mut self, start: f64, end: f64) -> Self {
        self.start_angle = start;
        self.end_angle = end;
        self
    }
    
    /// Set the depth modulation
    pub fn with_depth_modulation(mut self, modulation: f64) -> Self {
        self.depth_modulation = modulation;
        self
    }
}

impl Default for RoseEngineConfig {
    fn default() -> Self {
        Self::new(
            RosettePattern::default(),
            1.0,
            20.0,
            1000,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_config() {
        let config = RoseEngineConfig::new(
            RosettePattern::MultiLobe { lobes: 6 },
            2.0,
            15.0,
            500,
        );
        assert_eq!(config.amplitude, 2.0);
        assert_eq!(config.base_radius, 15.0);
        assert_eq!(config.resolution, 500);
    }
    
    #[test]
    fn test_default_config() {
        let config = RoseEngineConfig::default();
        assert_eq!(config.amplitude, 1.0);
        assert_eq!(config.base_radius, 20.0);
        assert_eq!(config.resolution, 1000);
    }
    
    #[test]
    fn test_flinque_preset() {
        let config = RoseEngineConfig::flinque(12, 25.0);
        assert_eq!(config.base_radius, 25.0);
        assert!(matches!(config.rosette, RosettePattern::MultiLobe { lobes: 12 }));
    }
    
    #[test]
    fn test_with_center() {
        let config = RoseEngineConfig::default()
            .with_center(10.0, 5.0);
        assert_eq!(config.center_x, 10.0);
        assert_eq!(config.center_y, 5.0);
    }
    
    #[test]
    fn test_with_phase() {
        let config = RoseEngineConfig::default()
            .with_phase(std::f64::consts::PI / 4.0);
        assert_eq!(config.phase, std::f64::consts::PI / 4.0);
    }
}
