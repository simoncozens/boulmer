use norad::Ufo;
use pyo3::class::{PyMappingProtocol, PySequenceProtocol};
use pyo3::prelude::*;
use pyo3::types::PyUnicode;
use pyo3::types::{PyDict, PyType};

use std::path::Path;
use std::sync::Arc;

use crate::glyph::_Glyph;
use crate::info::_Info;
use crate::layer::_Layer;

use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::PyResult;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct _Font {
    pub font: Ufo,
}

impl From<Ufo> for _Font {
    fn from(font: Ufo) -> Self {
        Self { font }
    }
}

impl Into<Ufo> for _Font {
    fn into(self) -> Ufo {
        self.font
    }
}

trait MyToPyObject {
    fn to_object(&self, py: Python) -> PyObject;
}

impl MyToPyObject for plist::Value {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            plist::Value::String(s) => s.to_object(py),
            plist::Value::Boolean(s) => s.to_object(py),
            plist::Value::Data(s) => s.to_object(py),
            plist::Value::Real(s) => s.to_object(py),
            plist::Value::Integer(s) => s.as_signed().to_object(py),
            plist::Value::Uid(s) => s.get().to_object(py),
            plist::Value::Array(s) => s
                .iter()
                .map(|v| v.to_object(py))
                .collect::<Vec<PyObject>>()
                .to_object(py),
            plist::Value::Dictionary(s) => s.to_object(py),
            // XXX Date!
            _ => py.None(),
        }
    }
}

impl MyToPyObject for plist::Dictionary {
    fn to_object(&self, py: Python) -> PyObject {
        let d = PyDict::new(py);
        for (k, v) in self.iter() {
            d.set_item(k, v.to_object(py)).unwrap();
        }
        d.into()
    }
}

#[allow(non_snake_case)]
#[pymethods]
impl _Font {
    #[new]
    fn new() -> Self {
        Self { font: Ufo::new() }
    }

    #[classmethod]
    fn load(_cls: &PyType, path: &PyUnicode) -> PyResult<Self> {
        let s: String = path.extract()?;
        match Ufo::load(Path::new(&s)) {
            Ok(ufo) => Ok(Self { font: ufo }),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }

    fn save(&self, path: &PyUnicode) -> PyResult<()> {
        let s: String = path.extract()?;
        match self.font.save(s) {
            Ok(()) => Ok(()),
            Err(error) => Err(PyValueError::new_err(error.to_string())),
        }
    }

    #[getter]
    fn lib(&self) -> pyo3::Py<pyo3::PyAny> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match &self.font.lib {
            Some(lib) => lib.to_object(py),
            None => PyDict::new(py).into(),
        }
    }

    fn get_default_layer(&self) -> PyResult<_Layer> {
        match self.font.get_default_layer() {
            Some(l) => Ok(l.clone().into()), // Ideally not clone
            None => Err(PyValueError::new_err("No default layer found")),
        }
    }

    fn find_layer_by_name(&self, s: &str) -> PyResult<_Layer> {
        match self.font.find_layer(|layer| layer.name == s) {
            Some(l) => Ok(l.clone().into()), // Ideally not clone
            None => Err(PyKeyError::new_err("Layer not found")),
        }
    }
    fn layer_count(&self) -> usize {
        self.font.layers.len()
    }

    // addGlyph
    // appendGuideline
    // bounds / controlpointbounds in Python
    // data?!
    fn _features(&self) -> Option<&String> {
        self.font.features.as_ref()
    }

    fn get(&self, s: &str, default: PyObject) -> PyObject {
        let gil = Python::acquire_gil();
        let py = gil.python();
        match self.font.get_glyph(s) {
            Some(glyph) => _Glyph {
                glyph: Arc::clone(glyph),
            }
            .into_py(py),
            None => default,
        }
    }

    // glyphOrder in Python

    #[getter]
    fn groups(&self) -> pyo3::Py<pyo3::PyAny> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let d = PyDict::new(py);
        if let Some(groups) = &self.font.groups {
            for (k, v) in groups.iter() {
                let names: Vec<String> = v.iter().map(|n| n.clone().to_string()).collect();
                d.set_item(k, names.to_object(py)).unwrap();
            }
        }
        d.into()
    }

    #[getter]
    fn info(&self) -> Option<_Info> {
        match &self.font.font_info {
            Some(s) => Some(s.clone().into()), // Ideally not clone
            None => None,
        }
    }

    #[getter]
    fn kerning(&self) -> pyo3::Py<pyo3::PyAny> {
        /* sigh */
        let gil = Python::acquire_gil();
        let py = gil.python();
        let d = PyDict::new(py);
        if let Some(kerning) = &self.font.kerning {
            for (k, v) in kerning.iter() {
                let d2 = PyDict::new(py);
                for (k2, v2) in v.iter() {
                    d2.set_item(k2, v2.to_object(py)).unwrap();
                }
                d.set_item(k, d2).unwrap();
            }
        }
        d.into()
    }

    fn keys(&self) -> Vec<String> {
        self.font.iter_names().map(|n| n.to_string()).collect()
    }

    // layers in Python
    // newGlyph
    // newLayer
    // objectLib ???!!
    // open in Python

    // fn _renameGlyph(&mut self, old: &str, new: &str, overwrite: bool) -> PyResult<()> {
    //     println!("Before");
    //     for n in self.font.iter_names() {
    //         println!(". {:?}", n);
    //     }
    //     if self.font.get_glyph(new).is_some() && !overwrite {
    //         return Err(PyValueError::new_err("New glyph already exists"));
    //     }
    //     if let Some(g) = self.font.get_glyph_mut(old) {
    //         println!("Found a glyph");
    //         g.name = new.into(); // REALLY?
    //         println!("Its name is now {:?}", g.name)
    //     }
    //     println!("After");
    //     for n in self.font.iter_names() {
    //         println!(". {:?}", n);
    //     }

    //     Ok(())
    // }
    // fn _renameLayer(&mut self, old: String, new: String, overwrite: bool) -> PyResult<()> {
    //     // Check if new layer exists
    //     if self.font.find_layer(|layer| layer.name == new).is_some() && !overwrite {
    //         return Err(PyValueError::new_err("New layer already exists"));
    //     }
    //     for l in self.font.layers.iter_mut() {
    //         if l.name == old {
    //             l.name = new.clone()
    //         }
    //     }
    //     Ok(())
    // }
}

#[pyproto]
impl PyMappingProtocol for _Font {
    fn __getitem__(&self, s: &str) -> Option<_Glyph> {
        match self.font.get_glyph(s) {
            Some(glyph) => Some(_Glyph {
                glyph: Arc::clone(glyph), // Ideally not clone
            }),
            None => None,
        }
    }
}

#[pyproto]
impl PySequenceProtocol for _Font {
    fn __len__(&self) -> usize {
        self.font.glyph_count()
    }

    fn __contains__(&self, glyphname: &str) -> bool {
        match self.font.get_default_layer() {
            Some(l) => l.contains_glyph(glyphname),
            None => false,
        }
    }
}
