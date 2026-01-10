use pyo3::prelude::*;

mod guilloche_bindings;
mod rose_engine_bindings;
mod spirograph_bindings;
mod watch_face_bindings;

pub use guilloche_bindings::{FlinqueLayer, GuillochePattern};
pub use rose_engine_bindings::{CuttingBit, RoseEngineConfig, RoseEngineLathe, RoseEngineLatheRun, RosettePattern};
pub use spirograph_bindings::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};
pub use watch_face_bindings::WatchFace;

#[pymodule]
fn turtles(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Spirograph classes
    m.add_class::<HorizontalSpirograph>().unwrap();
    m.add_class::<VerticalSpirograph>().unwrap();
    m.add_class::<SphericalSpirograph>().unwrap();

    // Guilloche pattern
    m.add_class::<GuillochePattern>().unwrap();

    // Flinqué (engine-turned) layer
    m.add_class::<FlinqueLayer>().unwrap();

    // Watch face
    m.add_class::<WatchFace>().unwrap();
    
    // Rose engine classes
    m.add_class::<RoseEngineLathe>().unwrap();
    m.add_class::<RoseEngineLatheRun>().unwrap();
    m.add_class::<RoseEngineConfig>().unwrap();
    m.add_class::<CuttingBit>().unwrap();
    m.add_class::<RosettePattern>().unwrap();

    Ok(())
}
