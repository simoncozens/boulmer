use std::sync::Arc;
use norad::Ufo;
use std::path::Path;
use pyo3::class::{PySequenceProtocol,PyMappingProtocol};
use pyo3::types::PyType;
use pyo3::prelude::*;
use pyo3::types::PyUnicode;

use crate::glyph::Glyph;

use pyo3::PyResult;
use pyo3::exceptions::PyValueError;

#[pyclass(subclass)]
#[derive(Clone,Debug)]
pub struct Font {
    pub font: Ufo,
}

impl From<Ufo> for Font {
    fn from(font: Ufo) -> Self {
        Self{font}
    }
}

impl Into<Ufo> for Font {
    fn into(self) -> Ufo {
        self.font
    }
}

#[pymethods]
impl Font {
    #[new]
    fn new() -> Self {
        Self {font: Ufo::new() }
    }

    #[classmethod]
    fn load(_cls: &PyType, path: &PyUnicode) -> PyResult<Self> {
    	let s: String = path.extract()?;
    	match Ufo::load(Path::new(&s)) {
    		Ok(ufo) => Ok(Self { font: ufo }),
    		Err(error) => Err(PyValueError::new_err(error.to_string()))
    	}
    }


}

#[pyproto]
impl PyMappingProtocol for Font {
    fn __getitem__(&self, s: &str) -> Option<Glyph> {
    	match self.font.get_glyph(s) {
				Some(glyph) => Some(Glyph { glyph: Arc::clone(glyph) }),
    		None => None
    	}
    }
}

#[pyproto]
impl PySequenceProtocol for Font {

    fn __len__(&self) -> PyResult<usize> {
      Ok(self.font.glyph_count())
    }
}
