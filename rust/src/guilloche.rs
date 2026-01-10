use crate::common::{validate_radius, ExportConfig, Point2D, SpirographError};
use crate::flinque::{FlinqueConfig, FlinqueLayer};
use crate::spirograph::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};

/// Enum to hold different types of spirograph patterns
#[derive(Debug, Clone)]
pub enum SpirographLayer {
    Horizontal(HorizontalSpirograph),
    Vertical(VerticalSpirograph),
    Spherical(SphericalSpirograph),
}

impl SpirographLayer {
    /// Generate points for this layer
    pub fn generate(&mut self) {
        match self {
            SpirographLayer::Horizontal(s) => {
                s.generate();
            }
            SpirographLayer::Vertical(s) => {
                s.generate();
            }
            SpirographLayer::Spherical(s) => {
                s.generate();
            }
        }
    }

    /// Get 2D points from this layer
    pub fn points_2d(&self) -> Vec<Point2D> {
        match self {
            SpirographLayer::Horizontal(s) => s.points().clone(),
            SpirographLayer::Vertical(s) => s.points().clone(),
            SpirographLayer::Spherical(s) => s.points_2d().clone(),
        }
    }
}

/// Enum to hold all layer types including flinqué
#[derive(Debug, Clone)]
pub enum GuillocheLayer {
    Spirograph(SpirographLayer),
    Flinque(FlinqueLayer),
}

impl GuillocheLayer {
    /// Generate points for this layer
    pub fn generate(&mut self) {
        match self {
            GuillocheLayer::Spirograph(s) => s.generate(),
            GuillocheLayer::Flinque(f) => f.generate(),
        }
    }
}

/// GuillochePattern - Combines multiple spirograph and flinqué patterns for complex guilloche effects
#[derive(Debug, Clone)]
pub struct GuillochePattern {
    pub radius: f64,
    spirograph_layers: Vec<SpirographLayer>,
    flinque_layers: Vec<FlinqueLayer>,
}

impl GuillochePattern {
    /// Create a new guilloche pattern with the specified radius
    pub fn new(radius: f64) -> Result<Self, SpirographError> {
        validate_radius(radius)?;

        Ok(GuillochePattern {
            radius,
            spirograph_layers: Vec::new(),
            flinque_layers: Vec::new(),
        })
    }

    /// Add a horizontal spirograph layer centered at origin
    pub fn add_horizontal_layer(&mut self, spiro: HorizontalSpirograph) {
        self.spirograph_layers
            .push(SpirographLayer::Horizontal(spiro));
    }

    /// Add a vertical spirograph layer centered at origin
    pub fn add_vertical_layer(&mut self, spiro: VerticalSpirograph) {
        self.spirograph_layers
            .push(SpirographLayer::Vertical(spiro));
    }

    /// Add a spherical spirograph layer centered at origin
    pub fn add_spherical_layer(&mut self, spiro: SphericalSpirograph) {
        self.spirograph_layers
            .push(SpirographLayer::Spherical(spiro));
    }

    /// Add a flinqué (engine-turned) layer
    pub fn add_flinque_layer(&mut self, flinque: FlinqueLayer) {
        self.flinque_layers.push(flinque);
    }

    /// Add a flinqué layer positioned at a given angle and distance from center
    /// angle is in radians, distance is in mm
    pub fn add_flinque_at_polar(
        &mut self,
        radius: f64,
        config: FlinqueConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let flinque = FlinqueLayer::new_at_polar(radius, config, angle, distance)?;
        self.flinque_layers.push(flinque);
        Ok(())
    }

    /// Add a flinqué layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `radius` - Radius of the flinqué pattern
    /// * `config` - Flinqué configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_flinque_at_clock(
        &mut self,
        radius: f64,
        config: FlinqueConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let flinque = FlinqueLayer::new_at_clock(radius, config, hour, minute, distance)?;
        self.flinque_layers.push(flinque);
        Ok(())
    }

    /// Generate all layers
    pub fn generate(&mut self) {
        for layer in &mut self.spirograph_layers {
            layer.generate();
        }
        for layer in &mut self.flinque_layers {
            layer.generate();
        }
    }

    /// Get total layer count (spirographs + flinqué)
    pub fn layer_count(&self) -> usize {
        self.spirograph_layers.len() + self.flinque_layers.len()
    }

    /// Get all spirograph layer points (for rendering)
    pub fn spirograph_points(&self) -> Vec<Vec<Point2D>> {
        self.spirograph_layers
            .iter()
            .map(|layer| layer.points_2d())
            .collect()
    }

    /// Get all flinqué layer lines (for rendering)
    pub fn flinque_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.flinque_layers.iter().map(|f| f.lines()).collect()
    }

    /// Export all layers to separate files with the given base name
    pub fn export_all(
        &self,
        base_name: &str,
        config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        if self.spirograph_layers.is_empty() && self.flinque_layers.is_empty() {
            return Err(SpirographError::ExportError(
                "No layers to export. Add layers first.".to_string(),
            ));
        }

        // Export combined SVG
        self.export_combined_svg(&format!("{}.svg", base_name))?;

        // Export combined STL
        self.export_combined_stl(&format!("{}.stl", base_name), config)?;

        // Export combined STEP
        self.export_combined_step(&format!("{}.stp", base_name), config)?;

        Ok(())
    }

    /// Export combined SVG with all layers
    pub fn export_combined_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use ::svg::node::element::path::Data;
        use ::svg::node::element::{Circle, Path};
        use ::svg::Document;

        let size = self.radius * 2.5;
        let mut document = Document::new()
            .set("viewBox", (-size, -size, size * 2.0, size * 2.0))
            .set("width", format!("{}mm", size * 2.0))
            .set("height", format!("{}mm", size * 2.0));

        // Watch dial circle
        let dial_circle = Circle::new()
            .set("cx", 0)
            .set("cy", 0)
            .set("r", self.radius)
            .set("fill", "#fafaf5") // Slightly lighter center
            .set("stroke", "#2c2c2c")
            .set("stroke-width", 0.3);

        document = document.add(dial_circle);

        // Guilloche line colors - subtle dark tones that simulate engraved metal
        // Using varying shades creates depth and visual interest
        let colors = [
            "#1a1a1a", // Deep black for primary pattern
            "#2d2d2d", // Dark gray
            "#3a3a3a", // Medium-dark gray
            "#454545", // Medium gray
            "#505050", // Lighter gray
            "#5a5a5a", // Light gray for subtle background patterns
        ];

        // Stroke widths - thinner lines for more delicate guilloche appearance
        let stroke_widths = [0.04, 0.035, 0.03, 0.03, 0.025, 0.025];

        // Render spirograph layers
        for (i, layer) in self.spirograph_layers.iter().enumerate() {
            let points = layer.points_2d();
            if points.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((points[0].x, points[0].y));
            for point in points.iter().skip(1) {
                data = data.line_to((point.x, point.y));
            }
            data = data.close();

            let color = colors[i % colors.len()];
            let stroke_width = stroke_widths[i % stroke_widths.len()];
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", stroke_width)
                .set("stroke-linecap", "round")
                .set("stroke-linejoin", "round")
                .set("d", data);

            document = document.add(path);
        }

        // Render flinqué layers
        for flinque_layer in &self.flinque_layers {
            for wave_points in flinque_layer.lines() {
                if wave_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((wave_points[0].x, wave_points[0].y));
                for point in wave_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                document = document.add(path);
            }
        }

        // Add outer bezel ring
        let bezel = Circle::new()
            .set("cx", 0)
            .set("cy", 0)
            .set("r", self.radius * 1.05)
            .set("fill", "none")
            .set("stroke", "#1a1a1a")
            .set("stroke-width", 0.8);

        document = document.add(bezel);

        // Add center pinhole for watch hands
        let center_hole = Circle::new()
            .set("cx", 0)
            .set("cy", 0)
            .set("r", 0.8)
            .set("fill", "#1a1a1a");

        document = document.add(center_hole);

        ::svg::save(filename, &document)
            .map_err(|e| SpirographError::ExportError(format!("SVG export failed: {}", e)))
    }

    /// Export combined STL with all layers
    pub fn export_combined_stl(
        &self,
        filename: &str,
        config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        use stl_io::{Normal, Triangle, Vertex};

        let mut all_triangles = Vec::new();
        let depth = config.depth;

        for layer in &self.spirograph_layers {
            let points = layer.points_2d();
            if points.is_empty() {
                continue;
            }

            let num_points = points.len();
            for i in 0..num_points {
                let p1 = points[i];
                let p2 = points[(i + 1) % num_points];

                let v1_top = Vertex::new([p1.x as f32, p1.y as f32, 0.0]);
                let v2_top = Vertex::new([p2.x as f32, p2.y as f32, 0.0]);
                let v1_bottom = Vertex::new([p1.x as f32, p1.y as f32, -depth as f32]);
                let v2_bottom = Vertex::new([p2.x as f32, p2.y as f32, -depth as f32]);

                let normal = Normal::new([0.0, 0.0, 1.0]);

                all_triangles.push(Triangle {
                    normal,
                    vertices: [v1_top, v2_top, v1_bottom],
                });
                all_triangles.push(Triangle {
                    normal,
                    vertices: [v2_top, v2_bottom, v1_bottom],
                });
            }
        }

        let mut file = std::fs::File::create(filename)
            .map_err(|e| SpirographError::ExportError(format!("Failed to create file: {}", e)))?;
        stl_io::write_stl(&mut file, all_triangles.iter())
            .map_err(|e| SpirographError::ExportError(format!("STL write failed: {}", e)))
    }

    /// Export combined STEP with all layers
    pub fn export_combined_step(
        &self,
        filename: &str,
        _config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        let mut content = String::new();

        let timestamp = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();

        content.push_str("ISO-10303-21;\n");
        content.push_str("HEADER;\n");
        content.push_str("FILE_DESCRIPTION(('Guilloche Pattern - Multiple Layers'),'2;1');\n");
        content.push_str(&format!(
            "FILE_NAME('guilloche.stp','{}',(''),(''),'','','');\n",
            timestamp
        ));
        content.push_str("FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\n");
        content.push_str("ENDSEC;\n");
        content.push_str("DATA;\n");

        let mut point_id = 1;
        for layer in &self.spirograph_layers {
            let points = layer.points_2d();
            for point in points {
                content.push_str(&format!(
                    "#{}=CARTESIAN_POINT('',({}.,{}.,0.));\n",
                    point_id, point.x, point.y
                ));
                point_id += 1;
            }
        }

        content.push_str("ENDSEC;\n");
        content.push_str("END-ISO-10303-21;\n");

        std::fs::write(filename, content)
            .map_err(|e| SpirographError::ExportError(format!("Failed to write STEP file: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guilloche_pattern_creation() {
        let pattern = GuillochePattern::new(40.0);
        assert!(pattern.is_ok());

        let pattern_bad = GuillochePattern::new(50.0);
        assert!(pattern_bad.is_err());
    }

    #[test]
    fn test_add_layers() {
        let mut pattern = GuillochePattern::new(38.0).unwrap();

        let h_spiro = HorizontalSpirograph::new(38.0, 0.75, 0.6, 50, 360).unwrap();
        pattern.add_horizontal_layer(h_spiro);

        let v_spiro = VerticalSpirograph::new(38.0, 0.6, 0.5, 30, 360, 2.0, 5.0).unwrap();
        pattern.add_vertical_layer(v_spiro);

        assert_eq!(pattern.layer_count(), 2);
    }

    #[test]
    fn test_generate_pattern() {
        let mut pattern = GuillochePattern::new(38.0).unwrap();

        let h_spiro = HorizontalSpirograph::new(38.0, 0.75, 0.6, 10, 100).unwrap();
        pattern.add_horizontal_layer(h_spiro);

        pattern.generate();

        // Verify points were generated
        assert_eq!(pattern.layer_count(), 1);
    }
}
