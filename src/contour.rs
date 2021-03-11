use crate::contourpoint::_ContourPoint;
use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::PyResult;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct _Contour {
    pub contour: norad::Contour,
}

impl From<norad::Contour> for _Contour {
    fn from(contour: norad::Contour) -> Self {
        Self { contour }
    }
}

trait ToString {
    fn to_string(&self) -> Option<String>;
}
impl ToString for norad::glyph::PointType {
    fn to_string(&self) -> Option<String> {
        match self {
            norad::PointType::Move => Some("move".to_string()),
            norad::PointType::Line => Some("line".to_string()),
            norad::PointType::OffCurve => None,
            norad::PointType::Curve => Some("curve".to_string()),
            norad::PointType::QCurve => Some("qcurve".to_string()),
        }
    }
}
#[pyproto]
impl PySequenceProtocol for _Contour {
    fn __len__(&self) -> usize {
        self.contour.points.len()
    }

    fn __getitem__(&self, i: isize) -> PyResult<_ContourPoint> {
        let mut u: usize = i as usize;
        if i < 0 {
            u += self.contour.points.len()
        }
        if u > self.contour.points.len() {
            return Err(PyIndexError::new_err("list index out of range"));
        }
        Ok(self.contour.points[u].clone().into())
    }
}

#[pymethods]
impl _Contour {
    // bounds/controlPointBounds in Python

    // draw in Python

    #[allow(non_snake_case)]
    fn drawPoints(&self, pen: PyObject) {
        let gil = Python::acquire_gil();
        let py = gil.python();

        if let Err(e) = pen.call_method0(py, "beginPath") {
            e.restore(py);
            return;
        }
        for p in &self.contour.points {
            let coord = (p.x, p.y).to_object(py);
            let d = PyDict::new(py);
            d.set_item("segmentType", p.typ.to_string()).unwrap();
            d.set_item("smooth", Some(p.smooth)).unwrap();
            d.set_item("name", p.name.as_ref()).unwrap();
            // d.set_item("identifier", p.identifier.as_ref()).unwrap();
            pen.call_method(py, "addPoint", (coord,), Some(d)).unwrap();
        }
        pen.call_method0(py, "endPath").unwrap();
    }

    // getBounds/getControlBounds in Python

    fn identifier(&self) -> Option<String> {
        match self.contour.identifier() {
            Some(s) => Some(s.as_str().to_string()),
            None => None,
        }
    }

    // insert xxx
    // move xxx
    fn open(&self) -> bool {
        if self.contour.points.is_empty() {
            return true;
        }
        self.contour.points[0].typ == norad::PointType::Move
    }

    fn points(&self) -> Vec<_ContourPoint> {
        self.contour
            .points
            .iter()
            .map(|pt| _ContourPoint { pt: pt.clone() }) // Ideally not
            .collect()
    }
}
