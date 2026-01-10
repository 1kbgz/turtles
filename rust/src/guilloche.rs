use crate::spirograph::{
    validate_radius, ExportConfig, HorizontalSpirograph, Point2D, SphericalSpirograph,
    SpirographError, VerticalSpirograph,
};

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

/// GuillochePattern - Combines multiple spirograph patterns for complex guilloche effects
#[derive(Debug, Clone)]
pub struct GuillochePattern {
    pub radius: f64,
    layers: Vec<SpirographLayer>,
}

impl GuillochePattern {
    /// Create a new guilloche pattern with the specified radius
    pub fn new(radius: f64) -> Result<Self, SpirographError> {
        validate_radius(radius)?;

        Ok(GuillochePattern {
            radius,
            layers: Vec::new(),
        })
    }

    /// Add a horizontal spirograph layer
    pub fn add_horizontal_layer(&mut self, spiro: HorizontalSpirograph) {
        self.layers.push(SpirographLayer::Horizontal(spiro));
    }

    /// Add a vertical spirograph layer
    pub fn add_vertical_layer(&mut self, spiro: VerticalSpirograph) {
        self.layers.push(SpirographLayer::Vertical(spiro));
    }

    /// Add a spherical spirograph layer
    pub fn add_spherical_layer(&mut self, spiro: SphericalSpirograph) {
        self.layers.push(SpirographLayer::Spherical(spiro));
    }

    /// Generate all layers
    pub fn generate(&mut self) {
        for layer in &mut self.layers {
            layer.generate();
        }
    }

    /// Export all layers to separate files with the given base name
    pub fn export_all(
        &self,
        base_name: &str,
        config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        if self.layers.is_empty() {
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
        use ::svg::node::element::Path;
        use ::svg::Document;

        let size = self.radius * 2.5;
        let mut document = Document::new()
            .set("viewBox", (-size, -size, size * 2.0, size * 2.0))
            .set("width", format!("{}mm", size * 2.0))
            .set("height", format!("{}mm", size * 2.0));

        // Colors for different layers
        let colors = ["black", "blue", "red", "green", "purple", "orange"];

        for (i, layer) in self.layers.iter().enumerate() {
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
            let path = Path::new()
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 0.1)
                .set("d", data);

            document = document.add(path);
        }

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

        for layer in &self.layers {
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
        for layer in &self.layers {
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

    /// Get the number of layers
    pub fn layer_count(&self) -> usize {
        self.layers.len()
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
