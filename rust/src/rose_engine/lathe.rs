use crate::common::{ExportConfig, Point2D, SpirographError};
use crate::rose_engine::config::RoseEngineConfig;
use crate::rose_engine::cutting_bit::CuttingBit;
use std::f64::consts::PI;

/// Arc segment for tool path representation
#[derive(Debug, Clone)]
pub struct Arc {
    /// Center point of the arc
    pub center: Point2D,
    /// Radius of the arc
    pub radius: f64,
    /// Start angle in radians
    pub start_angle: f64,
    /// End angle in radians
    pub end_angle: f64,
}

/// Output structure for tool paths and cut geometry
#[derive(Debug, Clone)]
pub struct ToolPathOutput {
    /// Center line of the tool path
    pub center_line: Vec<Point2D>,
    /// Actual cut edges (considering bit shape) - left and right edges
    pub cut_edges: Vec<Vec<Point2D>>,
    /// Optional arc segments for CNC machining
    pub arcs: Vec<Arc>,
}

/// Output structure for rendered visualization
#[derive(Debug, Clone)]
pub struct RenderedOutput {
    /// Lines to draw for visualization
    pub lines: Vec<Vec<Point2D>>,
    /// Depth at each point (optional, for 3D rendering)
    pub depth_map: Vec<f64>,
    /// Shading/intensity at each point (optional, 0.0 to 1.0)
    pub shading: Vec<f64>,
}

/// Main rose engine lathe implementation
#[derive(Debug, Clone)]
pub struct RoseEngineLathe {
    /// Configuration for the rose engine
    pub config: RoseEngineConfig,
    /// Cutting bit configuration
    pub cutting_bit: CuttingBit,
    /// Center position of the lathe (x, y)
    pub center_x: f64,
    pub center_y: f64,

    // Generated data
    tool_path: Vec<Point2D>,
    cut_geometry: ToolPathOutput,
    rendered: RenderedOutput,
    generated: bool,
}

impl RoseEngineLathe {
    /// Create a new rose engine lathe
    ///
    /// # Arguments
    /// * `config` - Rose engine configuration
    /// * `cutting_bit` - Cutting bit configuration
    ///
    /// # Example
    /// ```
    /// use turtles::rose_engine::{RoseEngineLathe, RoseEngineConfig, CuttingBit, RosettePattern};
    ///
    /// let mut config = RoseEngineConfig::new(20.0, 2.0);
    /// config.rosette = RosettePattern::MultiLobe { lobes: 12 };
    ///
    /// let bit = CuttingBit::v_shaped(30.0, 1.0);
    /// let lathe = RoseEngineLathe::new(config, bit).unwrap();
    /// ```
    pub fn new(config: RoseEngineConfig, cutting_bit: CuttingBit) -> Result<Self, SpirographError> {
        Self::new_with_center(config, cutting_bit, 0.0, 0.0)
    }

    /// Create a new rose engine lathe with custom center position
    ///
    /// # Arguments
    /// * `config` - Rose engine configuration
    /// * `cutting_bit` - Cutting bit configuration
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_with_center(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
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
            center_x,
            center_y,
            tool_path: Vec::new(),
            cut_geometry: ToolPathOutput {
                center_line: Vec::new(),
                cut_edges: Vec::new(),
                arcs: Vec::new(),
            },
            rendered: RenderedOutput {
                lines: Vec::new(),
                depth_map: Vec::new(),
                shading: Vec::new(),
            },
            generated: false,
        })
    }

    /// Generate the rose engine pattern
    /// This creates the tool path, cut geometry, and rendered output
    pub fn generate(&mut self) {
        self.generate_tool_path();
        self.generate_cut_geometry();
        self.generate_rendered_output();
        self.generated = true;
    }

    /// Generate the tool path (center line that the cutting bit follows)
    fn generate_tool_path(&mut self) {
        self.tool_path.clear();

        let angle_step =
            (self.config.end_angle - self.config.start_angle) / (self.config.resolution as f64);

        for i in 0..=self.config.resolution {
            let angle = self.config.start_angle + (i as f64) * angle_step;
            let radius = self.config.radius_at_angle(angle);

            let x = self.center_x + radius * angle.cos();
            let y = self.center_y + radius * angle.sin();

            self.tool_path.push(Point2D::new(x, y));
        }
    }

    /// Generate cut geometry considering the bit shape
    fn generate_cut_geometry(&mut self) {
        self.cut_geometry.center_line = self.tool_path.clone();
        self.cut_geometry.cut_edges.clear();
        self.cut_geometry.arcs.clear();

        if self.tool_path.len() < 2 {
            return;
        }

        let half_width = self.cutting_bit.width / 2.0;
        let mut left_edge = Vec::new();
        let mut right_edge = Vec::new();

        // Calculate edges offset by bit width perpendicular to path
        for i in 0..self.tool_path.len() {
            let angle = if i == 0 {
                // Use angle to next point
                let dx = self.tool_path[i + 1].x - self.tool_path[i].x;
                let dy = self.tool_path[i + 1].y - self.tool_path[i].y;
                dy.atan2(dx)
            } else if i == self.tool_path.len() - 1 {
                // Use angle from previous point
                let dx = self.tool_path[i].x - self.tool_path[i - 1].x;
                let dy = self.tool_path[i].y - self.tool_path[i - 1].y;
                dy.atan2(dx)
            } else {
                // Use average of unit vectors to handle angle wraparound correctly
                let dx1 = self.tool_path[i].x - self.tool_path[i - 1].x;
                let dy1 = self.tool_path[i].y - self.tool_path[i - 1].y;
                let dx2 = self.tool_path[i + 1].x - self.tool_path[i].x;
                let dy2 = self.tool_path[i + 1].y - self.tool_path[i].y;

                // Normalize to unit vectors
                let len1 = (dx1 * dx1 + dy1 * dy1).sqrt();
                let len2 = (dx2 * dx2 + dy2 * dy2).sqrt();

                if len1 > 0.0 && len2 > 0.0 {
                    let ux1 = dx1 / len1;
                    let uy1 = dy1 / len1;
                    let ux2 = dx2 / len2;
                    let uy2 = dy2 / len2;

                    // Average unit vectors
                    let avg_ux = (ux1 + ux2) / 2.0;
                    let avg_uy = (uy1 + uy2) / 2.0;
                    avg_uy.atan2(avg_ux)
                } else {
                    dy1.atan2(dx1)
                }
            };

            let perp_angle = angle + PI / 2.0;
            let offset_x = half_width * perp_angle.cos();
            let offset_y = half_width * perp_angle.sin();

            left_edge.push(Point2D::new(
                self.tool_path[i].x - offset_x,
                self.tool_path[i].y - offset_y,
            ));

            right_edge.push(Point2D::new(
                self.tool_path[i].x + offset_x,
                self.tool_path[i].y + offset_y,
            ));
        }

        self.cut_geometry.cut_edges.push(left_edge);
        self.cut_geometry.cut_edges.push(right_edge);
    }

    /// Generate rendered output for visualization
    fn generate_rendered_output(&mut self) {
        self.rendered.lines.clear();
        self.rendered.depth_map.clear();
        self.rendered.shading.clear();

        // Add center line as primary rendering line
        self.rendered.lines.push(self.tool_path.clone());

        // Add cut edges
        for edge in &self.cut_geometry.cut_edges {
            self.rendered.lines.push(edge.clone());
        }

        // Calculate depth and shading if depth modulation is enabled
        if self.config.depth_modulation {
            let angle_step =
                (self.config.end_angle - self.config.start_angle) / (self.config.resolution as f64);

            for i in 0..=self.config.resolution {
                let angle = self.config.start_angle + (i as f64) * angle_step;
                let depth = self.config.depth_at_angle(angle, self.cutting_bit.depth);
                self.rendered.depth_map.push(depth);

                // Simple shading based on depth (deeper = darker)
                // Avoid division by zero when depth is 0
                let shading = if self.cutting_bit.depth > 0.0 {
                    1.0 - (depth / (self.cutting_bit.depth * 2.0)).min(1.0)
                } else {
                    0.5 // Default shading for zero-depth bits
                };
                self.rendered.shading.push(shading);
            }
        }
    }

    /// Get the generated tool path
    pub fn tool_path(&self) -> &ToolPathOutput {
        &self.cut_geometry
    }

    /// Get the rendered output
    pub fn rendered_output(&self) -> &RenderedOutput {
        &self.rendered
    }

    /// Export to SVG format
    ///
    /// # Arguments
    /// * `filename` - Output SVG file path
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        if !self.generated {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        use svg::node::element::{path::Data, Path};
        use svg::Document;

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for line in &self.rendered.lines {
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
            .set("viewBox", (min_x - margin, min_y - margin, width, height));

        // Add each line
        for (idx, line) in self.rendered.lines.iter().enumerate() {
            if line.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((line[0].x, line[0].y));

            for point in line.iter().skip(1) {
                data = data.line_to((point.x, point.y));
            }

            let stroke_width = if idx == 0 { 0.1 } else { 0.05 };
            let path = Path::new()
                .set("d", data)
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", stroke_width);

            document = document.add(path);
        }

        svg::save(filename, &document).map_err(|e| {
            SpirographError::ExportError(format!("Failed to save SVG file '{}': {}", filename, e))
        })
    }

    /// Export to STL format
    ///
    /// # Arguments
    /// * `filename` - Output STL file path
    /// * `config` - Export configuration (depth, base thickness, etc.)
    pub fn to_stl(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        if !self.generated {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // For STL export, we need to create triangular mesh
        use stl_io::{Normal, Triangle, Vertex};

        let mut triangles = Vec::new();
        let depth = config.depth;
        let num_points = self.tool_path.len();

        // For each line segment in the path, create a rectangular groove
        for i in 0..num_points {
            if i >= num_points - 1 {
                break;
            }

            let p1 = self.tool_path[i];
            let p2 = self.tool_path[i + 1];

            // Create vertices for the groove
            let v1_top = Vertex::new([p1.x as f32, p1.y as f32, 0.0]);
            let v2_top = Vertex::new([p2.x as f32, p2.y as f32, 0.0]);
            let v1_bottom = Vertex::new([p1.x as f32, p1.y as f32, -depth as f32]);
            let v2_bottom = Vertex::new([p2.x as f32, p2.y as f32, -depth as f32]);

            // Create triangles for the groove sides
            let normal = Normal::new([0.0, 0.0, 1.0]);

            // Two triangles per segment
            triangles.push(Triangle {
                normal,
                vertices: [v1_top, v2_top, v1_bottom],
            });
            triangles.push(Triangle {
                normal,
                vertices: [v2_top, v2_bottom, v1_bottom],
            });
        }

        let mut file = std::fs::File::create(filename)
            .map_err(|e| SpirographError::ExportError(e.to_string()))?;
        stl_io::write_stl(&mut file, triangles.iter())
            .map_err(|e| SpirographError::ExportError(e.to_string()))
    }

    /// Export to STEP format
    ///
    /// # Arguments
    /// * `filename` - Output STEP file path
    /// * `config` - Export configuration
    pub fn to_step(&self, _filename: &str, _config: &ExportConfig) -> Result<(), SpirographError> {
        if !self.generated {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // STEP export would require a STEP library
        // This is a placeholder for now
        Err(SpirographError::ExportError(
            "STEP export not yet implemented".to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rose_engine::rosette::RosettePattern;

    #[test]
    fn test_rose_engine_creation() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let lathe = RoseEngineLathe::new(config, bit);
        assert!(lathe.is_ok());
    }

    #[test]
    fn test_rose_engine_invalid_params() {
        let mut config = RoseEngineConfig::new(20.0, 2.0);
        config.base_radius = -1.0;
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let lathe = RoseEngineLathe::new(config, bit);
        assert!(lathe.is_err());
    }

    #[test]
    fn test_rose_engine_generate() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();

        lathe.generate();
        assert!(lathe.generated);
        assert!(!lathe.tool_path.is_empty());
        assert!(!lathe.cut_geometry.center_line.is_empty());
    }

    #[test]
    fn test_tool_path_output() {
        let mut config = RoseEngineConfig::new(20.0, 2.0);
        config.rosette = RosettePattern::MultiLobe { lobes: 6 };
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();

        lathe.generate();
        let output = lathe.tool_path();

        assert_eq!(output.center_line.len(), lathe.tool_path.len());
        assert!(!output.cut_edges.is_empty());
    }

    #[test]
    fn test_rendered_output() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let mut lathe = RoseEngineLathe::new(config, bit).unwrap();

        lathe.generate();
        let rendered = lathe.rendered_output();

        assert!(!rendered.lines.is_empty());
    }

    #[test]
    fn test_svg_export_without_generate() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let lathe = RoseEngineLathe::new(config, bit).unwrap();

        let result = lathe.to_svg("/tmp/test.svg");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_center() {
        let config = RoseEngineConfig::new(20.0, 2.0);
        let bit = CuttingBit::v_shaped(60.0, 1.0);
        let lathe = RoseEngineLathe::new_with_center(config, bit, 10.0, 5.0).unwrap();

        assert_eq!(lathe.center_x, 10.0);
        assert_eq!(lathe.center_y, 5.0);
    }
}
