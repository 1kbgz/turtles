use crate::common::{ExportConfig, Point2D, SpirographError};
use crate::diamant::{DiamantConfig, DiamantLayer};
use crate::draperie::{DraperieConfig, DraperieLayer};
use crate::flinque::{FlinqueConfig, FlinqueLayer};
use crate::guilloche::GuillochePattern;
use crate::huiteight::{HuitEightConfig, HuitEightLayer};
use crate::limacon::{LimaconConfig, LimaconLayer};
use crate::paon::{PaonConfig, PaonLayer};
use crate::spirograph::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};

/// Watch dial circle configuration
#[derive(Debug, Clone)]
pub struct DialConfig {
    pub fill_color: String,
    pub stroke_color: String,
    pub stroke_width: f64,
}

impl Default for DialConfig {
    fn default() -> Self {
        DialConfig {
            fill_color: "#fafaf5".to_string(),
            stroke_color: "#2c2c2c".to_string(),
            stroke_width: 0.3,
        }
    }
}

/// Outer bezel ring configuration
#[derive(Debug, Clone)]
pub struct BezelConfig {
    pub radius_ratio: f64, // Multiplier of dial radius (e.g., 1.05 = 5% larger)
    pub stroke_color: String,
    pub stroke_width: f64,
}

impl Default for BezelConfig {
    fn default() -> Self {
        BezelConfig {
            radius_ratio: 1.05,
            stroke_color: "#1a1a1a".to_string(),
            stroke_width: 0.8,
        }
    }
}

/// Hole configuration (for center pinhole or other holes)
#[derive(Debug, Clone)]
pub struct HoleConfig {
    pub center_x: f64,
    pub center_y: f64,
    pub radius: f64,
    pub fill_color: String,
}

impl Default for HoleConfig {
    fn default() -> Self {
        HoleConfig {
            center_x: 0.0,
            center_y: 0.0,
            radius: 0.8,
            fill_color: "#1a1a1a".to_string(),
        }
    }
}

/// WatchFace - A high-level wrapper around GuillochePattern for creating watch dials
#[derive(Debug, Clone)]
pub struct WatchFace {
    pub guilloche: GuillochePattern,
    dial_config: Option<DialConfig>,
    bezel_config: Option<BezelConfig>,
    holes: Vec<HoleConfig>,
}

impl WatchFace {
    /// Create a new watch face with the specified radius
    pub fn new(radius: f64) -> Result<Self, SpirographError> {
        let guilloche = GuillochePattern::new(radius)?;
        Ok(WatchFace {
            guilloche,
            dial_config: None,
            bezel_config: None,
            holes: Vec::new(),
        })
    }

    /// Get the radius of the watch face
    pub fn radius(&self) -> f64 {
        self.guilloche.radius
    }

    /// Add the inner dial circle
    pub fn add_inner(&mut self) {
        self.add_inner_with_config(DialConfig::default());
    }

    /// Add the inner dial circle with custom configuration
    pub fn add_inner_with_config(&mut self, config: DialConfig) {
        self.dial_config = Some(config);
    }

    /// Add the outer bezel ring
    pub fn add_outer(&mut self) {
        self.add_outer_with_config(BezelConfig::default());
    }

    /// Add the outer bezel ring with custom configuration
    pub fn add_outer_with_config(&mut self, config: BezelConfig) {
        self.bezel_config = Some(config);
    }

    /// Add a center pinhole for watch hands (at origin with default size)
    pub fn add_center_hole(&mut self) {
        self.add_hole(HoleConfig::default());
    }

    /// Add a hole at a specific position
    pub fn add_hole(&mut self, config: HoleConfig) {
        self.holes.push(config);
    }

    /// Add a hole at a clock position
    pub fn add_hole_at_clock(&mut self, hour: u32, minute: u32, distance: f64, hole_radius: f64) {
        let (x, y) = crate::common::clock_to_cartesian(hour, minute, distance);
        self.holes.push(HoleConfig {
            center_x: x,
            center_y: y,
            radius: hole_radius,
            fill_color: "#1a1a1a".to_string(),
        });
    }

    /// Add a horizontal spirograph layer
    pub fn add_horizontal_layer(&mut self, spiro: HorizontalSpirograph) {
        self.guilloche.add_horizontal_layer(spiro);
    }

    /// Add a vertical spirograph layer
    pub fn add_vertical_layer(&mut self, spiro: VerticalSpirograph) {
        self.guilloche.add_vertical_layer(spiro);
    }

    /// Add a spherical spirograph layer
    pub fn add_spherical_layer(&mut self, spiro: SphericalSpirograph) {
        self.guilloche.add_spherical_layer(spiro);
    }

    /// Add a flinqué layer
    pub fn add_flinque_layer(&mut self, flinque: FlinqueLayer) {
        self.guilloche.add_flinque_layer(flinque);
    }

    /// Add a flinqué layer at a clock position
    pub fn add_flinque_at_clock(
        &mut self,
        radius: f64,
        config: FlinqueConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_flinque_at_clock(radius, config, hour, minute, distance)
    }

    /// Add a diamant (diamond pattern) layer
    pub fn add_diamant_layer(&mut self, diamant: DiamantLayer) {
        self.guilloche.add_diamant_layer(diamant);
    }

    /// Add a diamant layer at a clock position
    pub fn add_diamant_at_clock(
        &mut self,
        config: DiamantConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_diamant_at_clock(config, hour, minute, distance)
    }

    /// Add a draperie (drapery pattern) layer
    pub fn add_draperie_layer(&mut self, draperie: DraperieLayer) {
        self.guilloche.add_draperie_layer(draperie);
    }

    /// Add a draperie layer at a clock position
    pub fn add_draperie_at_clock(
        &mut self,
        config: DraperieConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_draperie_at_clock(config, hour, minute, distance)
    }

    /// Add a huit-eight (figure-eight) pattern layer
    pub fn add_huiteight_layer(&mut self, huiteight: HuitEightLayer) {
        self.guilloche.add_huiteight_layer(huiteight);
    }

    /// Add a huit-eight layer at a clock position
    pub fn add_huiteight_at_clock(
        &mut self,
        config: HuitEightConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_huiteight_at_clock(config, hour, minute, distance)
    }

    /// Add a limaçon pattern layer
    pub fn add_limacon_layer(&mut self, limacon: LimaconLayer) {
        self.guilloche.add_limacon_layer(limacon);
    }

    /// Add a limaçon layer at a clock position
    pub fn add_limacon_at_clock(
        &mut self,
        config: LimaconConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_limacon_at_clock(config, hour, minute, distance)
    }

    /// Add a paon (peacock pattern) layer
    pub fn add_paon_layer(&mut self, paon: PaonLayer) {
        self.guilloche.add_paon_layer(paon);
    }

    /// Add a paon layer at a clock position
    pub fn add_paon_at_clock(
        &mut self,
        config: PaonConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        self.guilloche
            .add_paon_at_clock(config, hour, minute, distance)
    }

    /// Generate all layers
    pub fn generate(&mut self) {
        self.guilloche.generate();
    }

    /// Get total layer count
    pub fn layer_count(&self) -> usize {
        self.guilloche.layer_count()
    }

    /// Export to SVG
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use ::svg::node::element::path::Data;
        use ::svg::node::element::{Circle, Path};
        use ::svg::Document;

        let radius = self.guilloche.radius;
        let size = radius * 2.5;
        let mut document = Document::new()
            .set("viewBox", (-size, -size, size * 2.0, size * 2.0))
            .set("width", format!("{}mm", size * 2.0))
            .set("height", format!("{}mm", size * 2.0));

        // Add inner dial circle if configured
        if let Some(ref dial) = self.dial_config {
            let dial_circle = Circle::new()
                .set("cx", 0)
                .set("cy", 0)
                .set("r", radius)
                .set("fill", dial.fill_color.as_str())
                .set("stroke", dial.stroke_color.as_str())
                .set("stroke-width", dial.stroke_width);
            document = document.add(dial_circle);
        }

        // Clip all pattern content to the dial circle
        {
            use ::svg::node::element::{ClipPath, Group};

            let clip_circle = Circle::new().set("cx", 0).set("cy", 0).set("r", radius);
            let clip = ClipPath::new().set("id", "dial-clip").add(clip_circle);
            document = document.add(clip);
        }

        // Guilloche line colors
        let colors = [
            "#1a1a1a", "#2d2d2d", "#3a3a3a", "#454545", "#505050", "#5a5a5a",
        ];
        let stroke_widths = [0.04, 0.035, 0.03, 0.03, 0.025, 0.025];

        // All pattern content goes inside a clipped group
        let mut pattern_group = {
            use ::svg::node::element::Group;
            Group::new().set("clip-path", "url(#dial-clip)")
        };

        // Render spirograph layers from guilloche
        for (i, points) in self.get_spirograph_points().iter().enumerate() {
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

            pattern_group = pattern_group.add(path);
        }

        // Render flinqué layers from guilloche
        for wave_lines in self.get_flinque_lines() {
            for wave_points in wave_lines {
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

                pattern_group = pattern_group.add(path);
            }
        }

        // Render diamant layers from guilloche
        for circle_lines in self.get_diamant_lines() {
            for circle_points in circle_lines {
                if circle_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((circle_points[0].x, circle_points[0].y));
                for point in circle_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                pattern_group = pattern_group.add(path);
            }
        }

        // Render draperie layers from guilloche
        for ring_lines in self.get_draperie_lines() {
            for ring_points in ring_lines {
                if ring_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((ring_points[0].x, ring_points[0].y));
                for point in ring_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                pattern_group = pattern_group.add(path);
            }
        }

        // Render huiteight layers from guilloche
        for curve_lines in self.get_huiteight_lines() {
            for curve_points in curve_lines {
                if curve_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((curve_points[0].x, curve_points[0].y));
                for point in curve_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                pattern_group = pattern_group.add(path);
            }
        }

        // Render limaçon layers from guilloche
        for curve_lines in self.get_limacon_lines() {
            for curve_points in curve_lines {
                if curve_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((curve_points[0].x, curve_points[0].y));
                for point in curve_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                pattern_group = pattern_group.add(path);
            }
        }

        // Render paon layers from guilloche
        for line_set in self.get_paon_lines() {
            for line_points in line_set {
                if line_points.is_empty() {
                    continue;
                }

                let mut data = Data::new().move_to((line_points[0].x, line_points[0].y));
                for point in line_points.iter().skip(1) {
                    data = data.line_to((point.x, point.y));
                }

                let path = Path::new()
                    .set("fill", "none")
                    .set("stroke", "#1a1a1a")
                    .set("stroke-width", 0.03)
                    .set("stroke-linecap", "round")
                    .set("stroke-linejoin", "round")
                    .set("d", data);

                pattern_group = pattern_group.add(path);
            }
        }

        document = document.add(pattern_group);

        // Add outer bezel ring if configured
        if let Some(ref bezel) = self.bezel_config {
            let bezel_circle = Circle::new()
                .set("cx", 0)
                .set("cy", 0)
                .set("r", radius * bezel.radius_ratio)
                .set("fill", "none")
                .set("stroke", bezel.stroke_color.as_str())
                .set("stroke-width", bezel.stroke_width);
            document = document.add(bezel_circle);
        }

        // Add all holes
        for hole in &self.holes {
            let hole_circle = Circle::new()
                .set("cx", hole.center_x)
                .set("cy", hole.center_y)
                .set("r", hole.radius)
                .set("fill", hole.fill_color.as_str());
            document = document.add(hole_circle);
        }

        ::svg::save(filename, &document)
            .map_err(|e| SpirographError::ExportError(format!("SVG export failed: {}", e)))
    }

    /// Export to STL
    pub fn to_stl(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        self.guilloche.export_combined_stl(filename, config)
    }

    /// Export to STEP
    pub fn to_step(&self, filename: &str, config: &ExportConfig) -> Result<(), SpirographError> {
        self.guilloche.export_combined_step(filename, config)
    }

    // Helper methods to access guilloche data for rendering
    fn get_spirograph_points(&self) -> Vec<Vec<Point2D>> {
        self.guilloche.spirograph_points()
    }

    fn get_flinque_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.flinque_lines()
    }

    fn get_diamant_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.diamant_lines()
    }

    fn get_draperie_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.draperie_lines()
    }

    fn get_huiteight_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.huiteight_lines()
    }

    fn get_limacon_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.limacon_lines()
    }

    fn get_paon_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.guilloche.paon_lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watch_face_creation() {
        let face = WatchFace::new(40.0);
        assert!(face.is_ok());

        let face_bad = WatchFace::new(50.0);
        assert!(face_bad.is_err());
    }

    #[test]
    fn test_add_inner_outer_hole() {
        let mut face = WatchFace::new(38.0).unwrap();
        face.add_inner();
        face.add_outer();
        face.add_center_hole();

        assert!(face.dial_config.is_some());
        assert!(face.bezel_config.is_some());
        assert_eq!(face.holes.len(), 1);
    }

    #[test]
    fn test_add_hole_at_clock() {
        let mut face = WatchFace::new(38.0).unwrap();
        face.add_hole_at_clock(3, 0, 15.0, 1.0);

        assert_eq!(face.holes.len(), 1);
        // At 3 o'clock, x should be positive
        assert!(face.holes[0].center_x > 0.0);
    }
}
