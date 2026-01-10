use crate::common::{ExportConfig, Point2D, SpirographError};

use super::config::RoseEngineConfig;
use super::cutting_bit::CuttingBit;

/// Represents a tool path segment (arc or line)
#[derive(Debug, Clone)]
pub struct Arc {
    pub start: Point2D,
    pub end: Point2D,
    pub center: Point2D,
    pub radius: f64,
}

/// Output from tool path generation
#[derive(Debug, Clone)]
pub struct ToolPathOutput {
    /// Center line of the tool path
    pub center_line: Vec<Point2D>,
    
    /// Actual cut edges considering bit shape
    /// Each inner Vec represents one edge of the cut
    pub cut_edges: Vec<Vec<Point2D>>,
    
    /// Arc segments (for CNC toolpaths)
    pub arcs: Vec<Arc>,
}

/// Output from rendered visualization
#[derive(Debug, Clone)]
pub struct RenderedOutput {
    /// Lines to draw for visualization
    pub lines: Vec<Vec<Point2D>>,
    
    /// Depth at each point (optional)
    pub depth_map: Vec<f64>,
    
    /// Shading/intensity values (optional)
    pub shading: Vec<f64>,
}

/// Rose Engine Lathe - generates guilloché patterns
/// 
/// A rose engine lathe simulates the mechanical process of cutting
/// decorative guilloché patterns into metal or other materials.
/// 
/// # Example
/// ```
/// use turtles::rose_engine::{RoseEngineLathe, RoseEngineConfig, RosettePattern, CuttingBit};
/// 
/// let config = RoseEngineConfig::new(
///     RosettePattern::MultiLobe { lobes: 12 },
///     2.0,
///     20.0,
///     1000
/// );
/// let bit = CuttingBit::v_shaped(30.0, 1.0);
/// 
/// let mut lathe = RoseEngineLathe::new(config, bit).unwrap();
/// lathe.generate();
/// ```
#[derive(Debug, Clone)]
pub struct RoseEngineLathe {
    /// Configuration for the rose engine
    pub config: RoseEngineConfig,
    
    /// Cutting bit/tool
    pub cutting_bit: CuttingBit,
    
    /// Generated tool path (center line)
    tool_path: Vec<Point2D>,
    
    /// Cut geometry (actual edges)
    cut_geometry: Vec<Vec<Point2D>>,
    
    /// Rendered visualization
    rendered_lines: Vec<Vec<Point2D>>,
}

impl RoseEngineLathe {
    /// Create a new rose engine lathe
    /// 
    /// # Arguments
    /// * `config` - Rose engine configuration
    /// * `cutting_bit` - Cutting bit/tool configuration
    pub fn new(config: RoseEngineConfig, cutting_bit: CuttingBit) -> Result<Self, SpirographError> {
        // Validate parameters
        if config.base_radius <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "base_radius must be positive".to_string(),
            ));
        }
        
        if config.amplitude < 0.0 {
            return Err(SpirographError::InvalidParameter(
                "amplitude must be non-negative".to_string(),
            ));
        }
        
        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }
        
        Ok(RoseEngineLathe {
            config,
            cutting_bit,
            tool_path: Vec::new(),
            cut_geometry: Vec::new(),
            rendered_lines: Vec::new(),
        })
    }
    
    /// Generate the complete rose engine pattern
    /// 
    /// This generates:
    /// - Tool path (center line of tool movement)
    /// - Cut geometry (actual material removal)
    /// - Rendered output (visualization)
    pub fn generate(&mut self) {
        self.generate_tool_path();
        self.generate_cut_geometry();
        self.generate_rendered_output();
    }
    
    /// Generate the tool path (center line of the cutting bit)
    /// 
    /// This simulates the path the center of the cutting bit follows
    /// as it traces the rosette pattern while the spindle rotates.
    pub fn generate_tool_path(&mut self) {
        self.tool_path.clear();
        
        let angle_step = (self.config.end_angle - self.config.start_angle) 
            / (self.config.resolution as f64);
        
        for i in 0..=self.config.resolution {
            let theta = self.config.start_angle + (i as f64) * angle_step;
            
            // Calculate radial displacement from rosette pattern
            let displacement = self.config.rosette.calculate_displacement(
                theta + self.config.phase,
                self.config.amplitude,
            );
            
            // Total radius = base radius + rosette displacement
            let r = self.config.base_radius + displacement;
            
            // Convert to Cartesian coordinates
            let x = r * theta.cos() + self.config.center_x;
            let y = r * theta.sin() + self.config.center_y;
            
            self.tool_path.push(Point2D::new(x, y));
        }
    }
    
    /// Generate cut geometry considering the cutting bit shape
    /// 
    /// This calculates the actual edges of the material removal
    /// by considering the cutting bit's cross-sectional profile.
    pub fn generate_cut_geometry(&mut self) {
        self.cut_geometry.clear();
        
        if self.tool_path.is_empty() {
            return;
        }
        
        // For simplicity, we'll generate two edge lines (left and right of center path)
        // In a full implementation, this would consider the bit shape more carefully
        
        let mut left_edge = Vec::new();
        let mut right_edge = Vec::new();
        
        let half_width = self.cutting_bit.width / 2.0;
        
        for i in 0..self.tool_path.len() {
            let point = self.tool_path[i];
            
            // Calculate perpendicular direction for this point
            let (dx, dy) = if i < self.tool_path.len() - 1 {
                // Use direction to next point
                let next = self.tool_path[i + 1];
                (next.x - point.x, next.y - point.y)
            } else if i > 0 {
                // Use direction from previous point
                let prev = self.tool_path[i - 1];
                (point.x - prev.x, point.y - prev.y)
            } else {
                (1.0, 0.0) // Default direction
            };
            
            // Normalize and rotate 90 degrees for perpendicular
            let len = (dx * dx + dy * dy).sqrt();
            if len > 0.0 {
                let perp_x = -dy / len;
                let perp_y = dx / len;
                
                // Create left and right edge points
                left_edge.push(Point2D::new(
                    point.x + perp_x * half_width,
                    point.y + perp_y * half_width,
                ));
                right_edge.push(Point2D::new(
                    point.x - perp_x * half_width,
                    point.y - perp_y * half_width,
                ));
            } else {
                left_edge.push(point);
                right_edge.push(point);
            }
        }
        
        self.cut_geometry.push(left_edge);
        self.cut_geometry.push(right_edge);
    }
    
    /// Generate rendered output for visualization
    /// 
    /// This creates the visual representation of the cut pattern,
    /// showing what it would look like when cut into material.
    pub fn generate_rendered_output(&mut self) {
        self.rendered_lines.clear();
        
        // For rendering, we'll use the cut geometry
        // In a more advanced implementation, this could include shading,
        // depth visualization, etc.
        
        if !self.cut_geometry.is_empty() {
            self.rendered_lines = self.cut_geometry.clone();
        } else if !self.tool_path.is_empty() {
            // Fallback to tool path if cut geometry not available
            self.rendered_lines.push(self.tool_path.clone());
        }
    }
    
    /// Get the tool path output
    pub fn tool_path(&self) -> ToolPathOutput {
        ToolPathOutput {
            center_line: self.tool_path.clone(),
            cut_edges: self.cut_geometry.clone(),
            arcs: Vec::new(), // Arc generation could be added later
        }
    }
    
    /// Get the rendered output
    pub fn rendered_output(&self) -> RenderedOutput {
        RenderedOutput {
            lines: self.rendered_lines.clone(),
            depth_map: Vec::new(),
            shading: Vec::new(),
        }
    }
    
    /// Get the generated lines (for backward compatibility with other patterns)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.rendered_lines
    }
    
    /// Export the pattern to SVG format
    /// 
    /// # Arguments
    /// * `filename` - Path to the output SVG file
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;
        
        let mut document = Document::new()
            .set("width", "400")
            .set("height", "400")
            .set("viewBox", "-50 -50 100 100");
        
        // Draw each line in the rendered output
        for line in &self.rendered_lines {
            if line.is_empty() {
                continue;
            }
            
            let mut data = Data::new().move_to((line[0].x, line[0].y));
            
            for point in line.iter().skip(1) {
                data = data.line_to((point.x, point.y));
            }
            
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", "0.1")
                .set("d", data);
            
            document = document.add(path);
        }
        
        svg::save(filename, &document)
            .map_err(|e| SpirographError::ExportError(e.to_string()))
    }
    
    /// Export the pattern to STL format
    /// 
    /// # Arguments
    /// * `filename` - Path to the output STL file
    /// * `config` - Export configuration (depth, base thickness, etc.)
    pub fn to_stl(&self, _filename: &str, _config: &ExportConfig) -> Result<(), SpirographError> {
        // For now, return an error indicating STL export is not fully implemented
        // Full implementation would create a 3D mesh from the cut geometry
        Err(SpirographError::ExportError(
            "STL export for rose engine not yet implemented".to_string(),
        ))
    }
    
    /// Export the pattern to STEP format
    /// 
    /// # Arguments
    /// * `filename` - Path to the output STEP file
    /// * `config` - Export configuration
    pub fn to_step(&self, _filename: &str, _config: &ExportConfig) -> Result<(), SpirographError> {
        // For now, return an error indicating STEP export is not fully implemented
        Err(SpirographError::ExportError(
            "STEP export for rose engine not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rose_engine::{RosettePattern, RoseEngineConfig};
    
    #[test]
    fn test_new_lathe() {
        let config = RoseEngineConfig::default();
        let bit = CuttingBit::default();
        let lathe = RoseEngineLathe::new(config, bit);
        assert!(lathe.is_ok());
    }
    
    #[test]
    fn test_invalid_parameters() {
        let mut config = RoseEngineConfig::default();
        config.base_radius = -1.0;
        let bit = CuttingBit::default();
        let lathe = RoseEngineLathe::new(config, bit);
        assert!(lathe.is_err());
    }
    
    #[test]
    fn test_generate_tool_path() {
        let config = RoseEngineConfig::new(
            RosettePattern::MultiLobe { lobes: 6 },
            1.0,
            10.0,
            100,
        );
        let bit = CuttingBit::default();
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();
        
        lathe.generate_tool_path();
        assert_eq!(lathe.tool_path.len(), 101); // 0..=100
    }
    
    #[test]
    fn test_generate_full_pattern() {
        let config = RoseEngineConfig::flinque(12, 15.0);
        let bit = CuttingBit::v_shaped(30.0, 0.5);
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();
        
        lathe.generate();
        assert!(!lathe.tool_path.is_empty());
        assert!(!lathe.rendered_lines.is_empty());
    }
    
    #[test]
    fn test_tool_path_output() {
        let config = RoseEngineConfig::default();
        let bit = CuttingBit::default();
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();
        
        lathe.generate();
        let output = lathe.tool_path();
        assert!(!output.center_line.is_empty());
    }
}
