// use crate::component::Component;

use crate::glyph::_Glyph;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;
use std::sync::Arc;

#[pyclass(unsendable)]
struct GlyphIterator {
    inner: Box<dyn Iterator<Item = Arc<norad::Glyph>>>,
}

#[pyproto]
impl PyIterProtocol for GlyphIterator {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<_Glyph> {
        match slf.inner.next() {
            Some(g) => Some(_Glyph { glyph: g }),
            None => None,
        }
    }
}

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct _Layer {
    pub layer: norad::Layer,
}

impl From<norad::Layer> for _Layer {
    fn from(layer: norad::Layer) -> Self {
        Self { layer }
    }
}

// #[pyproto]
// impl PyIterProtocol for _Layer {
//     fn __iter__(slf: PyRef<Self>) -> PyResult<Py<GlyphIterator>> {
//         let iter = GlyphIterator {
//             inner: Box::new(slf.layer.iter_contents())
//         };
//         Py::new(slf.py(), iter)
//     }
// }
