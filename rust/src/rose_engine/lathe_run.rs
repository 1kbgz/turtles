use std::f64::consts::PI;
use crate::common::{Point2D, SpirographError};
use crate::rose_engine::{RoseEngineConfig, RoseEngineLathe, CuttingBit};

/// A multi-pass rose engine lathe run that creates complex guilloché patterns
/// by making multiple overlapping cuts at different rotations.
///
/// This simulates the traditional watchmaking technique where a rose engine lathe
/// is used to make multiple passes at different angular positions to create
/// intricate intersecting patterns.
#[derive(Debug, Clone)]
pub struct RoseEngineLatheRun {
    /// Base configuration for each pass
    pub base_config: RoseEngineConfig,
    /// Cutting bit configuration
    pub cutting_bit: CuttingBit,
    /// Number of rotational passes to make
    pub num_passes: usize,
    /// Center position of the pattern (x, y)
    pub center_x: f64,
    pub center_y: f64,
    
    // Generated data
    passes: Vec<RoseEngineLathe>,
    generated: bool,
}

impl RoseEngineLatheRun {
    /// Create a new multi-pass rose engine lathe run
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes (typically 8-24)
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineLatheRun, RoseEngineConfig, CuttingBit, RosettePattern};
    /// 
    /// let mut config = RoseEngineConfig::new(20.0, 2.0);
    /// config.rosette = RosettePattern::MultiLobe { lobes: 12 };
    /// 
    /// let bit = CuttingBit::v_shaped(30.0, 0.5);
    /// let mut run = RoseEngineLatheRun::new(config, bit, 12).unwrap();
    /// run.generate();
    /// run.to_svg("guilloche_pattern.svg").unwrap();
    /// ```
    pub fn new(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
    ) -> Result<Self, SpirographError> {
        Self::new_with_center(config, cutting_bit, num_passes, 0.0, 0.0)
    }
    
    /// Create a new multi-pass rose engine lathe run with custom center position
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_with_center(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if num_passes == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_passes must be at least 1".to_string(),
            ));
        }
        
        if config.base_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "base_radius must be positive".to_string(),
            ));
        }
        
        Ok(RoseEngineLatheRun {
            base_config: config,
            cutting_bit,
            num_passes,
            center_x,
            center_y,
            passes: Vec::new(),
            generated: false,
        })
    }
    
    /// Generate all passes of the rose engine pattern
    ///
    /// This creates multiple lathe passes, each rotated by an equal angular increment
    /// to create a symmetric multi-pass guilloché pattern.
    pub fn generate(&mut self) {
        self.passes.clear();
        
        let rotation_step = 2.0 * PI / (self.num_passes as f64);
        
        for i in 0..self.num_passes {
            let rotation = (i as f64) * rotation_step;
            
            // Create a config for this pass with rotated start/end angles
            let mut pass_config = self.base_config.clone();
            pass_config.start_angle = rotation;
            pass_config.end_angle = rotation + 2.0 * PI;
            
            // Create and generate the lathe for this pass
            if let Ok(mut lathe) = RoseEngineLathe::new_with_center(
                pass_config,
                self.cutting_bit.clone(),
                self.center_x,
                self.center_y,
            ) {
                lathe.generate();
                self.passes.push(lathe);
            }
        }
        
        self.generated = true;
    }
    
    /// Export combined pattern to SVG format
    ///
    /// # Arguments
    /// * `filename` - Output SVG file path
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if !self.generated {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }
        
        use svg::node::element::{Path, path::Data};
        use svg::Document;
        
        // Collect all lines from all passes
        let mut all_lines = Vec::new();
        for pass in &self.passes {
            let rendered = pass.rendered_output();
            for line in &rendered.lines {
                all_lines.push(line.clone());
            }
        }
        
        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        
        for line in &all_lines {
            for point in line {
                min_x = min_x.min(point.x);
                max_x = max_x.max(point.x);
                min_y = min_y.min(point.y);
                max_y = max_y.max(point.y);
            }
        }
        
        let margin = 5.0;
        let width = max_x - min_x + 2.0 * margin;
        let height = max_y - min_y + 2.0 * margin;
        
        let mut document = Document::new()
            .set("width", format!("{}mm", width))
            .set("height", format!("{}mm", height))
            .set("viewBox", (
                min_x - margin,
                min_y - margin,
                width,
                height,
            ));
        
        // Add each line
        for line in all_lines.iter() {
            if line.is_empty() {
                continue;
            }
            
            let mut data = Data::new().move_to((line[0].x, line[0].y));
            
            for point in line.iter().skip(1) {
                data = data.line_to((point.x, point.y));
            }
            
            let path = Path::new()
                .set("d", data)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.05);
            
            document = document.add(path);
        }
        
        svg::save(filename, &document)
            .map_err(|e| SpirographError::ExportError(
                format!("Failed to save SVG file '{}': {}", filename, e)
            ))
    }
    
    /// Get the number of passes
    pub fn num_passes(&self) -> usize {
        self.num_passes
    }
    
    /// Get reference to individual passes
    pub fn passes(&self) -> &[RoseEngineLathe] {
        &self.passes
    }
}
