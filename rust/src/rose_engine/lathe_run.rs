use crate::common::{Point2D, SpirographError};
use crate::rose_engine::{CuttingBit, RoseEngineConfig, RoseEngineLathe};
use std::f64::consts::PI;

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
    /// Number of segments per pass (creates gaps for classical guilloché appearance)
    pub segments_per_pass: usize,
    /// Center position of the pattern (x, y)
    pub center_x: f64,
    pub center_y: f64,

    // Generated data
    passes: Vec<RoseEngineLathe>,
    segmented_lines: Vec<Vec<Point2D>>,
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
        // Default to 24 segments per pass for classical guilloché appearance
        Self::new_with_segments(config, cutting_bit, num_passes, 24, 0.0, 0.0)
    }

    /// Create a new multi-pass rose engine lathe run with custom segmentation
    ///
    /// # Arguments
    /// * `config` - Base rose engine configuration for each pass
    /// * `cutting_bit` - Cutting bit configuration
    /// * `num_passes` - Number of rotational passes
    /// * `segments_per_pass` - Number of arc segments per pass (creates gaps between segments)
    /// * `center_x` - X coordinate of center
    /// * `center_y` - Y coordinate of center
    pub fn new_with_segments(
        config: RoseEngineConfig,
        cutting_bit: CuttingBit,
        num_passes: usize,
        segments_per_pass: usize,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if num_passes == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_passes must be at least 1".to_string(),
            ));
        }

        if segments_per_pass == 0 {
            return Err(SpirographError::InvalidParameter(
                "segments_per_pass must be at least 1".to_string(),
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
            segments_per_pass,
            center_x,
            center_y,
            passes: Vec::new(),
            segmented_lines: Vec::new(),
            generated: false,
        })
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
        Self::new_with_segments(config, cutting_bit, num_passes, 24, center_x, center_y)
    }

    /// Generate all passes of the rose engine pattern
    ///
    /// This creates multiple lathe passes, each rotated by an equal angular increment.
    /// Each pass is segmented into multiple arcs with gaps to create the characteristic
    /// guilloché mesh appearance.
    ///
    /// For patterns like diamant (sinusoidal with frequency=1), rotating the phase
    /// rotates the entire circle around the center, creating the overlapping circles
    /// pattern. For multi-lobe patterns, rotating the phase rotates the pattern itself.
    pub fn generate(&mut self) {
        self.passes.clear();
        self.segmented_lines.clear();

        let rotation_step = 2.0 * PI / (self.num_passes as f64);

        for i in 0..self.num_passes {
            let rotation = (i as f64) * rotation_step;

            // Create a config for this pass with rotated phase
            // Rotating the phase (not start/end angles) rotates the entire pattern
            // around the center. For a sinusoidal pattern with frequency=1, this
            // rotates the offset circle around the origin, creating the diamant pattern.
            let mut pass_config = self.base_config.clone();
            pass_config.phase = self.base_config.phase + rotation;

            // Create and generate the lathe for this pass
            if let Ok(mut lathe) = RoseEngineLathe::new_with_center(
                pass_config,
                self.cutting_bit.clone(),
                self.center_x,
                self.center_y,
            ) {
                lathe.generate();

                // Get the complete circular path from this pass
                let rendered = lathe.rendered_output();
                if !rendered.lines.is_empty() && !rendered.lines[0].is_empty() {
                    let complete_path = &rendered.lines[0];

                    // Segment this path into multiple arcs with gaps
                    self.segment_path(complete_path);
                }

                self.passes.push(lathe);
            }
        }

        self.generated = true;
    }

    /// Segment a complete circular path into multiple arcs with gaps
    fn segment_path(&mut self, path: &[Point2D]) {
        if path.is_empty() || self.segments_per_pass == 0 {
            return;
        }

        // Special case: segments_per_pass=1 means draw the complete path without gaps
        if self.segments_per_pass == 1 {
            self.segmented_lines.push(path.to_vec());
            return;
        }

        let total_points = path.len();

        // Calculate points per segment
        // Each segment takes up a fraction of the circle with a gap
        // For visual effect: 70% drawing, 30% gap
        let draw_ratio = 0.7;
        let points_per_cycle = total_points / self.segments_per_pass;
        let draw_points = (points_per_cycle as f64 * draw_ratio) as usize;

        for seg_idx in 0..self.segments_per_pass {
            let start_idx = seg_idx * points_per_cycle;
            let end_idx = (start_idx + draw_points).min(total_points);

            if start_idx < total_points && end_idx > start_idx {
                let segment: Vec<Point2D> = path[start_idx..end_idx].to_vec();
                if !segment.is_empty() {
                    self.segmented_lines.push(segment);
                }
            }
        }
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

        use svg::node::element::{path::Data, Path};
        use svg::Document;

        // Use segmented lines instead of complete passes
        let all_lines = &self.segmented_lines;

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for line in all_lines {
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

        // Add each segmented line
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

        svg::save(filename, &document).map_err(|e| {
            SpirographError::ExportError(format!("Failed to save SVG file '{}': {}", filename, e))
        })
    }

    /// Get the number of passes
    pub fn num_passes(&self) -> usize {
        self.num_passes
    }

    /// Get reference to individual passes
    pub fn passes(&self) -> &[RoseEngineLathe] {
        &self.passes
    }

    /// Get reference to the segmented lines (the generated pattern curves)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.segmented_lines
    }
}
