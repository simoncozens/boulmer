use pyo3::prelude::*;

mod font;
mod layer;
// mod anchor;
mod component;
mod contour;
mod contourpoint;
mod glyph;
mod info;
// mod image;

#[pymodule]
fn boulmer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<font::_Font>()?;
    m.add_class::<glyph::_Glyph>()?;
    m.add_class::<layer::_Layer>()?;
    m.add_class::<contour::_Contour>()?;
    m.add_class::<contourpoint::_ContourPoint>()?;
    m.add_class::<component::_Component>()?;
    m.add_class::<info::_Info>()?;
    Ok(())
}
