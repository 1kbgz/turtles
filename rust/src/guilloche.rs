use crate::clous_de_paris::{ClousDeParisConfig, ClousDeParisLayer};
use crate::common::{
    clip_lines_to_radius, validate_radius, ExportConfig, Point2D, Point3D, SpirographError,
};
use crate::cube::{CubeConfig, CubeLayer};
use crate::diamant::{DiamantConfig, DiamantLayer};
use crate::draperie::{DraperieConfig, DraperieLayer};
use crate::flinque::{FlinqueConfig, FlinqueLayer};
use crate::huiteight::{HuitEightConfig, HuitEightLayer};
use crate::limacon::LimaconLayer;
use crate::paon::{PaonConfig, PaonLayer};
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
    diamant_layers: Vec<DiamantLayer>,
    draperie_layers: Vec<DraperieLayer>,
    huiteight_layers: Vec<HuitEightLayer>,
    limacon_layers: Vec<LimaconLayer>,
    paon_layers: Vec<PaonLayer>,
    clous_de_paris_layers: Vec<ClousDeParisLayer>,
    cube_layers: Vec<CubeLayer>,
}

impl GuillochePattern {
    /// Create a new guilloche pattern with the specified radius
    pub fn new(radius: f64) -> Result<Self, SpirographError> {
        validate_radius(radius)?;

        Ok(GuillochePattern {
            radius,
            spirograph_layers: Vec::new(),
            flinque_layers: Vec::new(),
            diamant_layers: Vec::new(),
            draperie_layers: Vec::new(),
            huiteight_layers: Vec::new(),
            limacon_layers: Vec::new(),
            paon_layers: Vec::new(),
            clous_de_paris_layers: Vec::new(),
            cube_layers: Vec::new(),
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

    /// Add a diamant (diamond pattern) layer
    pub fn add_diamant_layer(&mut self, diamant: DiamantLayer) {
        self.diamant_layers.push(diamant);
    }

    /// Add a diamant layer positioned at a given angle and distance from center
    /// angle is in radians, distance is in mm
    pub fn add_diamant_at_polar(
        &mut self,
        config: DiamantConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let diamant = DiamantLayer::new_at_polar(config, angle, distance)?;
        self.diamant_layers.push(diamant);
        Ok(())
    }

    /// Add a diamant layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `config` - Diamant configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_diamant_at_clock(
        &mut self,
        config: DiamantConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let diamant = DiamantLayer::new_at_clock(config, hour, minute, distance)?;
        self.diamant_layers.push(diamant);
        Ok(())
    }

    /// Add a draperie (drapery pattern) layer
    pub fn add_draperie_layer(&mut self, draperie: DraperieLayer) {
        self.draperie_layers.push(draperie);
    }

    /// Add a huit-eight (figure-eight) pattern layer
    pub fn add_huiteight_layer(&mut self, huiteight: HuitEightLayer) {
        self.huiteight_layers.push(huiteight);
    }

    /// Add a huit-eight layer positioned at a given angle and distance from center
    pub fn add_huiteight_at_polar(
        &mut self,
        config: HuitEightConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let huiteight = HuitEightLayer::new_at_polar(config, angle, distance)?;
        self.huiteight_layers.push(huiteight);
        Ok(())
    }

    /// Add a huit-eight layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Huit-eight configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_huiteight_at_clock(
        &mut self,
        config: HuitEightConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let huiteight = HuitEightLayer::new_at_clock(config, hour, minute, distance)?;
        self.huiteight_layers.push(huiteight);
        Ok(())
    }

    /// Add a draperie layer positioned at a given angle and distance from center
    pub fn add_draperie_at_polar(
        &mut self,
        config: DraperieConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let draperie = DraperieLayer::new_at_polar(config, angle, distance)?;
        self.draperie_layers.push(draperie);
        Ok(())
    }

    /// Add a draperie layer positioned at a clock position
    pub fn add_draperie_at_clock(
        &mut self,
        config: DraperieConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let draperie = DraperieLayer::new_at_clock(config, hour, minute, distance)?;
        self.draperie_layers.push(draperie);
        Ok(())
    }

    /// Add a limaçon pattern layer
    pub fn add_limacon_layer(&mut self, limacon: LimaconLayer) {
        self.limacon_layers.push(limacon);
    }

    /// Add a limaçon layer positioned at a given angle and distance from center
    /// angle is in radians, distance is in mm
    pub fn add_limacon_at_polar(
        &mut self,
        config: crate::limacon::LimaconConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let limacon = LimaconLayer::new_at_polar(config, angle, distance)?;
        self.limacon_layers.push(limacon);
        Ok(())
    }

    /// Add a limaçon layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `config` - Limaçon configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_limacon_at_clock(
        &mut self,
        config: crate::limacon::LimaconConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let limacon = LimaconLayer::new_at_clock(config, hour, minute, distance)?;
        self.limacon_layers.push(limacon);
        Ok(())
    }

    /// Add a paon (peacock) pattern layer
    pub fn add_paon_layer(&mut self, paon: PaonLayer) {
        self.paon_layers.push(paon);
    }

    /// Add a paon layer positioned at a given angle and distance from center
    pub fn add_paon_at_polar(
        &mut self,
        config: PaonConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let paon = PaonLayer::new_at_polar(config, angle, distance)?;
        self.paon_layers.push(paon);
        Ok(())
    }

    /// Add a paon layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Paon configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_paon_at_clock(
        &mut self,
        config: PaonConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let paon = PaonLayer::new_at_clock(config, hour, minute, distance)?;
        self.paon_layers.push(paon);
        Ok(())
    }

    /// Add a clous de Paris (hobnail) pattern layer
    pub fn add_clous_de_paris_layer(&mut self, cdp: ClousDeParisLayer) {
        self.clous_de_paris_layers.push(cdp);
    }

    /// Add a clous de Paris layer positioned at a given angle and distance from center
    pub fn add_clous_de_paris_at_polar(
        &mut self,
        config: ClousDeParisConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let cdp = ClousDeParisLayer::new_at_polar(config, angle, distance)?;
        self.clous_de_paris_layers.push(cdp);
        Ok(())
    }

    /// Add a clous de Paris layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Clous de Paris configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_clous_de_paris_at_clock(
        &mut self,
        config: ClousDeParisConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let cdp = ClousDeParisLayer::new_at_clock(config, hour, minute, distance)?;
        self.clous_de_paris_layers.push(cdp);
        Ok(())
    }

    /// Add a cube (tumbling blocks) pattern layer
    pub fn add_cube_layer(&mut self, cube: CubeLayer) {
        self.cube_layers.push(cube);
    }

    /// Add a cube layer positioned at a given angle and distance from center
    pub fn add_cube_at_polar(
        &mut self,
        config: CubeConfig,
        angle: f64,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let cube = CubeLayer::new_at_polar(config, angle, distance)?;
        self.cube_layers.push(cube);
        Ok(())
    }

    /// Add a cube layer positioned at a clock position
    ///
    /// # Arguments
    /// * `config` - Cube configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from center of watch face
    pub fn add_cube_at_clock(
        &mut self,
        config: CubeConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<(), SpirographError> {
        let cube = CubeLayer::new_at_clock(config, hour, minute, distance)?;
        self.cube_layers.push(cube);
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
        for layer in &mut self.diamant_layers {
            layer.generate();
        }
        for layer in &mut self.draperie_layers {
            layer.generate();
        }
        for layer in &mut self.huiteight_layers {
            layer.generate();
        }
        for layer in &mut self.limacon_layers {
            layer.generate();
        }
        for layer in &mut self.paon_layers {
            layer.generate();
        }
        for layer in &mut self.clous_de_paris_layers {
            layer.generate();
        }
        for layer in &mut self.cube_layers {
            layer.generate();
        }
    }

    /// Get total layer count (spirographs + flinqué + diamant + limaçon)
    pub fn layer_count(&self) -> usize {
        self.spirograph_layers.len()
            + self.flinque_layers.len()
            + self.diamant_layers.len()
            + self.draperie_layers.len()
            + self.huiteight_layers.len()
            + self.limacon_layers.len()
            + self.paon_layers.len()
            + self.clous_de_paris_layers.len()
            + self.cube_layers.len()
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

    /// Get all diamant layer lines (for rendering)
    pub fn diamant_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.diamant_layers.iter().map(|d| d.lines()).collect()
    }

    /// Get all draperie layer lines (for rendering)
    pub fn draperie_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.draperie_layers.iter().map(|d| d.lines()).collect()
    }

    /// Get all huit-eight layer lines (for rendering)
    pub fn huiteight_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.huiteight_layers.iter().map(|h| h.lines()).collect()
    }

    /// Get all limaçon layer lines (for rendering)
    pub fn limacon_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.limacon_layers.iter().map(|l| l.lines()).collect()
    }

    /// Get all paon layer lines (for rendering)
    pub fn paon_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.paon_layers.iter().map(|p| p.lines()).collect()
    }

    /// Get all clous de Paris layer lines (for rendering)
    pub fn clous_de_paris_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.clous_de_paris_layers
            .iter()
            .map(|c| c.lines())
            .collect()
    }

    /// Get all cube layer lines (for rendering)
    pub fn cube_lines(&self) -> Vec<&Vec<Vec<Point2D>>> {
        self.cube_layers.iter().map(|c| c.lines()).collect()
    }

    /// Collect every layer's geometry as a flat list of polylines.
    ///
    /// Used by the combined exporters so that they cannot silently omit a
    /// layer family - the previous STL/STEP exporters iterated only
    /// `spirograph_layers`, so a flinqué- or diamant-only pattern produced a
    /// valid but completely empty file.
    pub fn all_paths(&self) -> Vec<Vec<Point2D>> {
        let mut paths: Vec<Vec<Point2D>> = Vec::new();
        for layer in &self.spirograph_layers {
            paths.push(layer.points_2d());
        }
        for group in [
            self.flinque_lines(),
            self.diamant_lines(),
            self.draperie_lines(),
            self.huiteight_lines(),
            self.limacon_lines(),
            self.paon_lines(),
            self.clous_de_paris_lines(),
            self.cube_lines(),
        ] {
            for layer_lines in group {
                for line in layer_lines {
                    paths.push(line.clone());
                }
            }
        }
        paths
    }

    /// Every layer's geometry, clipped to the dial boundary.
    ///
    /// The manufacturing exporters must use this rather than [`Self::all_paths`].
    /// An SVG `clip-path` only hides ink in a viewer; STL and STEP carry raw
    /// coordinates, so an overflowing pattern was previously *machined* outside
    /// the dial it belongs to (measured: a 42 mm mesh on a 38 mm dial).
    pub fn clipped_paths(&self) -> Vec<Vec<Point2D>> {
        clip_lines_to_radius(&self.all_paths(), self.radius)
    }

    /// Export all layers to separate files with the given base name
    pub fn export_all(
        &self,
        base_name: &str,
        config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        if self.spirograph_layers.is_empty()
            && self.flinque_layers.is_empty()
            && self.diamant_layers.is_empty()
            && self.draperie_layers.is_empty()
            && self.huiteight_layers.is_empty()
            && self.limacon_layers.is_empty()
            && self.paon_layers.is_empty()
            && self.clous_de_paris_layers.is_empty()
            && self.cube_layers.is_empty()
        {
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
        use ::svg::node::element::{Circle, ClipPath, Group, Path};
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

        // Everything the pattern draws lives in a clipped group. The combined
        // exporter previously applied no clipping whatsoever, so a layer that
        // overflowed its dial (a limaçon reaching 42mm on a 38mm face) drew
        // straight over the bezel. `watch_face.rs` already did this; the two
        // exporters simply disagreed.
        let clip_circle = Circle::new()
            .set("cx", 0)
            .set("cy", 0)
            .set("r", self.radius);
        document = document.add(
            ClipPath::new()
                .set("id", "guilloche-dial-clip")
                .add(clip_circle),
        );
        let mut pattern_group = Group::new().set("clip-path", "url(#guilloche-dial-clip)");

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
            // Do NOT close the path. A spirograph only returns to its start when
            // `rotations` is an exact multiple of the curve's closure period; for
            // any other value `close()` draws a straight chord from the last point
            // back to the first, straight across the pattern. The standalone
            // exporter in `spirograph.rs` already deliberately omits this.

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

                pattern_group = pattern_group.add(path);
            }
        }

        // Render diamant layers
        for diamant_layer in &self.diamant_layers {
            for circle_points in diamant_layer.lines() {
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

        // Render huit-eight layers
        for huiteight_layer in &self.huiteight_layers {
            for curve_points in huiteight_layer.lines() {
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

        // Render draperie layers
        for draperie_layer in &self.draperie_layers {
            for ring_points in draperie_layer.lines() {
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

        // Render paon layers
        for paon_layer in &self.paon_layers {
            for line_points in paon_layer.lines() {
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

        // Render limaçon layers
        for limacon_layer in &self.limacon_layers {
            for curve_points in limacon_layer.lines() {
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

        // Render clous de Paris layers
        for clous_layer in &self.clous_de_paris_layers {
            for line_points in clous_layer.lines() {
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

        // Render cube layers
        for cube_layer in &self.cube_layers {
            for line_points in cube_layer.lines() {
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
    ///
    /// NOTE: the emitted mesh is still a zero-thickness ribbon with placeholder
    /// normals - see TODO.md. This function's contract fixed here is *coverage*:
    /// it previously iterated only `spirograph_layers`, so a flinqué- or
    /// diamant-only pattern silently produced an empty (but valid) STL.
    pub fn export_combined_stl(
        &self,
        filename: &str,
        config: &ExportConfig,
    ) -> Result<(), SpirographError> {
        use stl_io::{Normal, Triangle, Vertex};

        let mut all_triangles = Vec::new();
        let depth = config.depth;

        let mut extrude = |points: &[Point2D], closed: bool| {
            if points.len() < 2 {
                return;
            }
            let num_points = points.len();
            let segments = if closed { num_points } else { num_points - 1 };
            for i in 0..segments {
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
        };

        // Clipping splits a stroke wherever it leaves the dial, so every piece
        // is an open run - closing any of them would bridge the gap with a
        // chord straight across the face.
        for path in self.clipped_paths() {
            extrude(&path, false);
        }

        if all_triangles.is_empty() {
            return Err(SpirographError::ExportError(
                "No geometry to export. Ensure the pattern was generated and \
                 contains at least one path of two or more points."
                    .to_string(),
            ));
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
        let paths: Vec<Vec<Point3D>> = self
            .clipped_paths()
            .into_iter()
            .map(|path| {
                path.into_iter()
                    .map(|p| Point3D::new(p.x, p.y, 0.0))
                    .collect()
            })
            .collect();

        crate::step::write_step_polylines(filename, "Guilloche Pattern - Multiple Layers", &paths)
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
