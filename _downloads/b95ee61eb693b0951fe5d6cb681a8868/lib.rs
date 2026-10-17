use pyo3::prelude::*;

mod errors;
mod diamant_bindings;
mod draperie_bindings;
mod clous_de_paris_bindings;
mod cube_bindings;
mod guilloche_bindings;
mod huiteight_bindings;
mod limacon_bindings;
mod paon_bindings;
mod rose_engine_bindings;
mod spirograph_bindings;
mod watch_face_bindings;

pub use clous_de_paris_bindings::ClousDeParisLayer;
pub use cube_bindings::CubeLayer;
pub use diamant_bindings::DiamantLayer;
pub use draperie_bindings::DraperieLayer;
pub use guilloche_bindings::{FlinqueLayer, GuillochePattern};
pub use huiteight_bindings::HuitEightLayer;
pub use limacon_bindings::LimaconLayer;
pub use paon_bindings::PaonLayer;
pub use rose_engine_bindings::{CuttingBit, RoseEngineConfig, RoseEngineLathe, RoseEngineLatheRun, RosettePattern};
pub use spirograph_bindings::{HorizontalSpirograph, SphericalSpirograph, VerticalSpirograph};
pub use watch_face_bindings::WatchFace;

#[pymodule]
fn turtles(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    // Spirograph classes
    m.add_class::<HorizontalSpirograph>()?;
    m.add_class::<VerticalSpirograph>()?;
    m.add_class::<SphericalSpirograph>()?;

    // Guilloche pattern
    m.add_class::<GuillochePattern>()?;

    // Flinqué (engine-turned) layer
    m.add_class::<FlinqueLayer>()?;

    // Diamant (diamond) pattern layer
    m.add_class::<DiamantLayer>()?;

    // Huit-Eight (figure-eight) pattern layer
    m.add_class::<HuitEightLayer>()?;

    // Draperie (drapery) pattern layer
    m.add_class::<DraperieLayer>()?;

    // Paon (peacock) pattern layer
    m.add_class::<PaonLayer>()?;

    // Clous de Paris (hobnail) pattern layer
    m.add_class::<ClousDeParisLayer>()?;

    // Cube (tumbling blocks) pattern layer
    m.add_class::<CubeLayer>()?;

    // Limaçon pattern layer
    m.add_class::<LimaconLayer>()?;

    // Watch face
    m.add_class::<WatchFace>()?;

    // Rose engine classes
    m.add_class::<RoseEngineLathe>()?;
    m.add_class::<RoseEngineLatheRun>()?;
    m.add_class::<RoseEngineConfig>()?;
    m.add_class::<CuttingBit>()?;
    m.add_class::<RosettePattern>()?;

    Ok(())
}