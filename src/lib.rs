use pyo3::prelude::*;

mod font;
// mod layer;
// mod anchor;
mod component;
// mod contour;
// mod contourpoint;
mod glyph;
// mod image;
// mod outline;

#[pymodule]
fn boulmer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<font::_Font>()?;
    m.add_class::<glyph::_Glyph>()?;
    m.add_class::<component::Component>()?;
    Ok(())
}
