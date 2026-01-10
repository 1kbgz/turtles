use pyo3::prelude::*;

mod spirograph_bindings;
mod guilloche_bindings;

pub use spirograph_bindings::{HorizontalSpirograph, VerticalSpirograph, SphericalSpirograph};
pub use guilloche_bindings::{GuillochePattern, FlinqueLayer};


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

    Ok(())
}
