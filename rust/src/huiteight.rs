use std::f64::consts::PI;

use crate::common::{clock_to_cartesian, polar_to_cartesian, Point2D, SpirographError};

/// Configuration for the Huit-Eight (Figure-Eight) guilloché pattern
///
/// The huit-eight pattern is formed by drawing figure-eight curves (lemniscates
/// of Bernoulli) that pass through the centre, rotated around the centre at
/// different angles.  The overlapping lemniscates create an intricate woven mesh.
///
/// ## Lemniscate of Bernoulli
///
/// The parametric form used is:
///
///   x(t) = a · cos(t) / (1 + sin²(t))
///   y(t) = a · sin(t) · cos(t) / (1 + sin²(t))
///
/// where `a` is `scale` (the half-width of the figure-eight).  The curve:
///   - passes through the origin at t = π/2, 3π/2
///   - extends to ±a along the x-axis at t = 0, π
///   - has maximum y-extent ±a/(2√2) ≈ ±0.354a
///
/// Each lemniscate is rotated by 2π·i/N around the origin so that N curves
/// tile the full circle.
#[derive(Debug, Clone)]
pub struct HuitEightConfig {
    /// Number of figure-eight curves to draw (more = denser mesh)
    pub num_curves: usize,
    /// Scale (half-width) of each figure-eight
    pub scale: f64,
    /// Resolution – number of points per curve
    pub resolution: usize,
    /// Number of clusters to group curves into (0 = uniform distribution)
    ///
    /// When non-zero, the `num_curves` are divided among `num_clusters`
    /// evenly-spaced groups.  Within each group the curves are tightly
    /// packed, producing the characteristic "petalled" look seen on
    /// real engine-turned dials.
    pub num_clusters: usize,
    /// Angular spread **per cluster** in radians.
    ///
    /// Controls how tightly each bundle of curves is packed.  Smaller
    /// values produce denser petals with wider gaps between them.
    /// A value of 0 means "auto" – half of the sector allocated to
    /// each cluster (π / num_clusters).
    pub cluster_spread: f64,
}

impl Default for HuitEightConfig {
    fn default() -> Self {
        HuitEightConfig {
            num_curves: 72,
            scale: 20.0,
            resolution: 360,
            num_clusters: 0,
            cluster_spread: 0.0,
        }
    }
}

impl HuitEightConfig {
    /// Create a new huit-eight configuration
    ///
    /// # Arguments
    /// * `num_curves` - Number of figure-eight curves to draw around the centre
    /// * `scale` - Half-width of each lemniscate
    pub fn new(num_curves: usize, scale: f64) -> Self {
        HuitEightConfig {
            num_curves,
            scale,
            resolution: 360,
            num_clusters: 0,
            cluster_spread: 0.0,
        }
    }

    /// Set the resolution (points per curve)
    pub fn with_resolution(mut self, resolution: usize) -> Self {
        self.resolution = resolution;
        self
    }

    /// Set clustering parameters
    ///
    /// When `num_clusters > 0`, curves are grouped into that many bundles,
    /// spaced evenly around the origin.  `spread` (radians) controls the
    /// angular width of each bundle.  Use 0.0 for automatic spread.
    pub fn with_clusters(mut self, num_clusters: usize, spread: f64) -> Self {
        self.num_clusters = num_clusters;
        self.cluster_spread = spread;
        self
    }
}

/// A Huit-Eight (Figure-Eight) pattern layer
///
/// Creates the huit-eight guilloché effect by drawing lemniscate curves
/// (figure-eights) that pass through the centre and are rotated at equal
/// angular intervals.  The overlapping curves create a dense woven mesh
/// pattern reminiscent of traditional engine-turned watch dials.
#[derive(Debug, Clone)]
pub struct HuitEightLayer {
    pub config: HuitEightConfig,
    pub center_x: f64,
    pub center_y: f64,
    curves: Vec<Vec<Point2D>>,
}

impl HuitEightLayer {
    /// Create a new huit-eight layer centred at origin
    pub fn new(config: HuitEightConfig) -> Result<Self, SpirographError> {
        Self::new_with_center(config, 0.0, 0.0)
    }

    /// Create a new huit-eight layer with a custom centre point
    pub fn new_with_center(
        config: HuitEightConfig,
        center_x: f64,
        center_y: f64,
    ) -> Result<Self, SpirographError> {
        if config.scale <= 0.0 {
            return Err(SpirographError::InvalidParameter(
                "scale must be positive".to_string(),
            ));
        }

        if config.num_curves == 0 {
            return Err(SpirographError::InvalidParameter(
                "num_curves must be at least 1".to_string(),
            ));
        }

        if config.resolution < 10 {
            return Err(SpirographError::InvalidParameter(
                "resolution must be at least 10".to_string(),
            ));
        }

        Ok(HuitEightLayer {
            config,
            center_x,
            center_y,
            curves: Vec::new(),
        })
    }

    /// Create a huit-eight layer positioned at a given angle and distance from origin
    pub fn new_at_polar(
        config: HuitEightConfig,
        angle: f64,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = polar_to_cartesian(angle, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Create a huit-eight layer positioned at a clock position (like hour hand)
    ///
    /// # Arguments
    /// * `config` - Huit-eight configuration
    /// * `hour` - Hour position (1-12, where 12 is at top)
    /// * `minute` - Minute position (0-59)
    /// * `distance` - Distance from centre of watch face
    pub fn new_at_clock(
        config: HuitEightConfig,
        hour: u32,
        minute: u32,
        distance: f64,
    ) -> Result<Self, SpirographError> {
        let (center_x, center_y) = clock_to_cartesian(hour, minute, distance);
        Self::new_with_center(config, center_x, center_y)
    }

    /// Generate the huit-eight pattern
    ///
    /// Each curve is a lemniscate of Bernoulli rotated by an angle
    /// determined by dividing the full rotation among all curves.
    /// The parametric form is:
    ///
    ///   x(t) = a cos(t) / (1 + sin²(t))
    ///   y(t) = a sin(t) cos(t) / (1 + sin²(t))
    ///
    /// rotated by the per-curve rotation angle.
    pub fn generate(&mut self) {
        self.curves.clear();

        let a = self.config.scale;
        let n = self.config.num_curves;

        // Build the list of rotation angles.
        let rotations: Vec<f64> = if self.config.num_clusters > 0 && self.config.num_clusters < n {
            let nc = self.config.num_clusters;
            let curves_per_cluster = n / nc;
            let remainder = n % nc;
            let sector = 2.0 * PI / (nc as f64);
            let spread = if self.config.cluster_spread > 0.0 {
                self.config.cluster_spread
            } else {
                sector * 0.5 // auto: half the sector width
            };

            let mut rots = Vec::with_capacity(n);
            for k in 0..nc {
                let cluster_center = (k as f64) * sector;
                let count = curves_per_cluster + if k < remainder { 1 } else { 0 };
                for c in 0..count {
                    let t = if count > 1 {
                        (c as f64) / ((count - 1) as f64) - 0.5 // −0.5 .. +0.5
                    } else {
                        0.0
                    };
                    rots.push(cluster_center + t * spread);
                }
            }
            rots
        } else {
            // Uniform distribution
            let angle_step = 2.0 * PI / (n as f64);
            (0..n).map(|i| (i as f64) * angle_step).collect()
        };

        for rotation in &rotations {
            let cos_rot = rotation.cos();
            let sin_rot = rotation.sin();

            let mut curve_points = Vec::with_capacity(self.config.resolution + 1);

            for j in 0..=self.config.resolution {
                let t = (j as f64) / (self.config.resolution as f64);
                let angle = 2.0 * PI * t;

                // Lemniscate of Bernoulli parametric form
                let sin_a = angle.sin();
                let cos_a = angle.cos();
                let denom = 1.0 + sin_a * sin_a;

                let lx = a * cos_a / denom;
                let ly = a * sin_a * cos_a / denom;

                // Rotate by the per-curve rotation angle
                let x = self.center_x + lx * cos_rot - ly * sin_rot;
                let y = self.center_y + lx * sin_rot + ly * cos_rot;

                curve_points.push(Point2D::new(x, y));
            }

            self.curves.push(curve_points);
        }
    }

    /// Get the generated curves as a vector of point vectors
    pub fn curves(&self) -> &Vec<Vec<Point2D>> {
        &self.curves
    }

    /// Get all lines for rendering (alias for curves)
    pub fn lines(&self) -> &Vec<Vec<Point2D>> {
        &self.curves
    }

    /// Export the pattern to SVG format
    pub fn to_svg(&self, filename: &str) -> Result<(), SpirographError> {
        use svg::node::element::{path::Data, Path};
        use svg::Document;

        if self.curves.is_empty() {
            return Err(SpirographError::ExportError(
                "Pattern not generated. Call generate() first.".to_string(),
            ));
        }

        // Find bounds
        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for curve in &self.curves {
            for point in curve {
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

        for curve in &self.curves {
            if curve.is_empty() {
                continue;
            }

            let mut data = Data::new().move_to((curve[0].x, curve[0].y));
            for point in curve.iter().skip(1) {
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
            .map_err(|e| SpirographError::ExportError(format!("Failed to save SVG: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huiteight_config_default() {
        let config = HuitEightConfig::default();
        assert_eq!(config.num_curves, 72);
        assert_eq!(config.scale, 20.0);
        assert_eq!(config.resolution, 360);
    }

    #[test]
    fn test_huiteight_config_new() {
        let config = HuitEightConfig::new(48, 15.0);
        assert_eq!(config.num_curves, 48);
        assert_eq!(config.scale, 15.0);
    }

    #[test]
    fn test_huiteight_layer_creation() {
        let config = HuitEightConfig::default();
        let layer = HuitEightLayer::new(config);
        assert!(layer.is_ok());
    }

    #[test]
    fn test_huiteight_layer_invalid_scale() {
        let config = HuitEightConfig::new(48, -10.0);
        let layer = HuitEightLayer::new(config);
        assert!(layer.is_err());
    }

    #[test]
    fn test_huiteight_layer_generate() {
        let config = HuitEightConfig::new(12, 10.0).with_resolution(36);
        let mut layer = HuitEightLayer::new(config).unwrap();
        layer.generate();

        assert_eq!(layer.curves().len(), 12);
        assert_eq!(layer.curves()[0].len(), 37); // resolution + 1
    }

    #[test]
    fn test_huiteight_passes_through_origin() {
        let config = HuitEightConfig::new(4, 10.0).with_resolution(360);
        let mut layer = HuitEightLayer::new(config).unwrap();
        layer.generate();

        // Each figure-eight should pass through or very close to the origin
        for curve in layer.curves() {
            let min_dist = curve
                .iter()
                .map(|p| (p.x * p.x + p.y * p.y).sqrt())
                .fold(f64::INFINITY, f64::min);

            assert!(
                min_dist < 0.5,
                "Figure-eight should pass through origin, min_dist = {}",
                min_dist
            );
        }
    }

    #[test]
    fn test_huiteight_matches_rose_engine() {
        use crate::rose_engine::RoseEngineLatheRun;

        let num_curves = 12;
        let scale = 10.0;
        let resolution = 360;

        // Create mathematical HuitEightLayer
        let config = HuitEightConfig::new(num_curves, scale).with_resolution(resolution);
        let mut huiteight = HuitEightLayer::new(config).unwrap();
        huiteight.generate();

        // Create equivalent rose engine huiteight
        let mut rose_run =
            RoseEngineLatheRun::new_huiteight(num_curves, scale, resolution, 0.0, 0.0, 0, 0.0)
                .unwrap();
        rose_run.generate();

        let he_lines = huiteight.lines();
        let rose_lines = rose_run.lines();

        assert_eq!(
            he_lines.len(),
            rose_lines.len(),
            "HuitEightLayer and RoseEngineLatheRun should have same number of curves"
        );

        for (i, (h_curve, r_curve)) in he_lines.iter().zip(rose_lines.iter()).enumerate() {
            assert_eq!(
                h_curve.len(),
                r_curve.len(),
                "Curve {} should have same number of points",
                i
            );

            for (j, (h_pt, r_pt)) in h_curve.iter().zip(r_curve.iter()).enumerate() {
                let dist = ((h_pt.x - r_pt.x).powi(2) + (h_pt.y - r_pt.y).powi(2)).sqrt();
                assert!(
                    dist < 1e-10,
                    "Point {},{} differs: huiteight=({}, {}), rose=({}, {}), dist={}",
                    i,
                    j,
                    h_pt.x,
                    h_pt.y,
                    r_pt.x,
                    r_pt.y,
                    dist
                );
            }
        }
    }

    #[test]
    fn test_huiteight_clustered_matches_rose_engine() {
        use crate::rose_engine::RoseEngineLatheRun;

        let num_curves = 48;
        let scale = 10.0;
        let resolution = 360;
        let num_clusters = 8;
        let cluster_spread = 0.3;

        let config = HuitEightConfig::new(num_curves, scale)
            .with_resolution(resolution)
            .with_clusters(num_clusters, cluster_spread);
        let mut huiteight = HuitEightLayer::new(config).unwrap();
        huiteight.generate();

        let mut rose_run = RoseEngineLatheRun::new_huiteight(
            num_curves,
            scale,
            resolution,
            0.0,
            0.0,
            num_clusters,
            cluster_spread,
        )
        .unwrap();
        rose_run.generate();

        let he_lines = huiteight.lines();
        let rose_lines = rose_run.lines();

        assert_eq!(he_lines.len(), rose_lines.len());

        for (i, (h_curve, r_curve)) in he_lines.iter().zip(rose_lines.iter()).enumerate() {
            assert_eq!(h_curve.len(), r_curve.len());
            for (j, (h_pt, r_pt)) in h_curve.iter().zip(r_curve.iter()).enumerate() {
                let dist = ((h_pt.x - r_pt.x).powi(2) + (h_pt.y - r_pt.y).powi(2)).sqrt();
                assert!(
                    dist < 1e-10,
                    "Clustered point {},{} differs: dist={}",
                    i,
                    j,
                    dist
                );
            }
        }
    }
}
