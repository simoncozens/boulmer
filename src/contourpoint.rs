use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyo3::PyResult;


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

#[pymethods]
impl _ContourPoint {
	#[getter]
	fn x(&self) -> f32 { self.pt.x }
	#[getter]
	fn y(&self) -> f32 { self.pt.y }
	#[getter]
	fn smooth(&self) -> bool { self.pt.smooth }

	#[getter]
	fn name(&self) -> Option<String> { self.pt.name.clone() }

	#[allow(non_snake_case)]
	#[getter]
	fn segmentType(&self) -> Option<String> { self.pt.typ.to_string() }
	#[getter]
	fn identifier(&self) -> Option<String> {
		match self.pt.identifier() {
			Some(s) => Some(s.as_str().into()),
			None => None
		}
	}
}
