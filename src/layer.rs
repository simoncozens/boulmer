// use crate::component::Component;

use crate::glyph::_Glyph;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;
use pyo3::PyMappingProtocol;
use pyo3::PySequenceProtocol;
use std::sync::Arc;

#[pyclass]
pub struct _GlyphIterator {
    contents: Vec<_Glyph>,
    index: usize,
}

#[pyproto]
impl PyIterProtocol for _GlyphIterator {
    fn __iter__(slf: PyRef<'p, Self>) -> PyRef<'p, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<_Glyph> {
        let index = slf.index;
        slf.index += 1;
        match slf.contents.get(index) {
            Some(x) => Some(x.clone()), // XXX unfortunate
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

#[pyproto]
impl PyIterProtocol for _Layer {
    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<_GlyphIterator>> {
        let iter = _GlyphIterator {
            contents: slf
                .layer
                .iter_contents()
                .map(|glyph| _Glyph { glyph })
                .collect(),
            index: 0,
        };
        Py::new(slf.py(), iter)
    }
}

#[pyproto]
impl PyMappingProtocol for _Layer {
    fn __getitem__(&self, s: &str) -> Option<_Glyph> {
        match self.layer.get_glyph(s) {
            Some(glyph) => Some(_Glyph {
                glyph: Arc::clone(glyph), // Ideally not clone
            }),
            None => None,
        }
    }
}
#[pyproto]
impl PySequenceProtocol for _Layer {
    fn __len__(&self) -> usize {
        self.layer.iter_contents().count()
    }

    fn __contains__(&self, glyphname: &str) -> bool {
        self.layer.contains_glyph(glyphname)
    }
}

#[pymethods]
impl _Layer {
    // fn addGlyph(&self, g: Arc<_Glyph>) {
    //     if !self.layer.contains_glyph(&g.glyph.name) {
    //         self.layer.insert_glyph(g.glyph)
    //     }
    // }
}
