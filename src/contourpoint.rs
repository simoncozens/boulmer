use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyo3::PyResult;


#[pyclass(subclass)]
#[derive(Clone,Debug)]
pub struct _ContourPoint {
    pub pt: norad::ContourPoint,
}

impl From<norad::ContourPoint> for _ContourPoint {
    fn from(pt: norad::ContourPoint) -> Self {
        Self{pt}
    }
}
