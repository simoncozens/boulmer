use crate::contourpoint::_ContourPoint;
use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
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
