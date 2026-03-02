// Common types shared across modules
pub mod common;
// Diamant (diamond) pattern generation
pub mod diamant;
// Draperie (drapery) pattern generation
pub mod draperie;
// Flinque (engine-turned) pattern generation
pub mod flinque;
// Spirograph and guilloche pattern generation modules
pub mod guilloche;
// Huit-Eight (Figure-Eight) pattern generation
pub mod huiteight;
// Limaçon pattern generation
pub mod limacon;
// Clous de Paris (Hobnail) pattern generation
pub mod clous_de_paris;
// Paon (Peacock) pattern generation
pub mod paon;
pub mod spirograph;
// Rose engine lathe module
pub mod rose_engine;
// Watch face wrapper
pub mod watch_face;

// Re-export main types for convenience
pub use clous_de_paris::{ClousDeParisConfig, ClousDeParisLayer};
pub use common::{
    clock_to_cartesian, polar_to_cartesian, validate_radius, ExportConfig, Point2D, Point3D,
    SpirographError,
};
pub use diamant::{DiamantConfig, DiamantLayer};
pub use draperie::{DraperieConfig, DraperieLayer};
pub use flinque::{FlinqueConfig, FlinqueLayer};
pub use guilloche::GuillochePattern;
pub use huiteight::{HuitEightConfig, HuitEightLayer};
pub use limacon::{LimaconConfig, LimaconLayer};
pub use paon::{paon_wave_fn, PaonConfig, PaonLayer};
pub use rose_engine::{
    Arc, BitShape, CuttingBit, RenderedOutput, RoseEngineConfig, RoseEngineLathe,
    RoseEngineLatheRun, RosettePattern, ToolPathOutput,
};
pub use spirograph::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};
pub use watch_face::{BezelConfig, DialConfig, HoleConfig, WatchFace};

/**********************************/
// #[cfg(test)]
// mod example_tests {
//     use super::*;

//     #[test]
//     fn test_new() {
//         let e = Example::new(String::from("test"));
//         assert_eq!(e.stuff, String::from("test"));
//     }

//     #[test]
//     fn test_clone_and_eq() {
//         let e = Example::new(String::from("test"));
//         assert_eq!(e, e.clone());
//     }

//     #[test]
//     fn test_debug() {
//         let e = Example::new(String::from("test"));
//         assert_eq!(format!("{e:?}"), "Example { stuff: \"test\" }");
//     }
// }
