// Spirograph and guilloche pattern generation modules
pub mod guilloche;
pub mod spirograph;

// Re-export main types for convenience
pub use guilloche::GuillochePattern;
pub use spirograph::{
    ExportConfig, HorizontalSpirograph, Point2D, Point3D, SphericalSpirograph, SpirographError,
    VerticalSpirograph,
};

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
