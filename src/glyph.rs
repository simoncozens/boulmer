use crate::contour::_Contour;
use crate::component::_Component;
use pyo3::types::PyDict;
// use crate::component::Component;
use std::sync::Arc;
use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyo3::PyResult;


#[pyclass(subclass)]
#[derive(Clone,Debug)]
pub struct _Glyph {
    pub glyph: Arc<norad::Glyph>,
}

impl From<Arc<norad::Glyph>> for _Glyph {
    fn from(glyph: Arc<norad::Glyph>) -> Self {
        Self{glyph}
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

#[pymethods]
impl _Glyph {
    // #[new]
    // fn new() -> Self {
    //     Self {glyph: norad::Glyph::new() }
    // }

//     fn __getitem__(&self, s: &str) -> Option<usize> {
//     	match self.font.get_glyph(s) {
// 				Some(glyph) => Some(1),
//     		None => None
//     	}
//     }

    // fn anchors
    // fn appendAnchor
    // fn appendContour
    // fn appendGuideline
    // fn clear(&mut self) {
    //     self.clearAnchors();
    //     self.clearComponents();
    //     self.clearContours();
    //     self.clearGuidelines();
    // }

    // fn clearAnchors(&mut self) {
    //   if let Some(a) = &self.glyph.anchors {
    //     a.clear()
    //   }
    // }
    // fn clearComponents(&self) {
    //   if let Some(o) = &self.glyph.outline {
    //     o.components.clear()
    //   }
    // }
    // fn clearContours(&self) {
    //   if let Some(o) = &self.glyph.outline {
    //     o.contours.clear()
    //   }
    // }
    // fn clearGuidelines(&self) {
    //   if let Some(g) = &self.glyph.guidelines {
    //     g.clear()
    //   }
    // }

    fn components(&self) -> Vec<_Component> {
        match &self.glyph.outline {
            Some(outline) => outline.components.iter().map(|c| _Component::from(c.clone())).collect(),
            None => Vec::<_Component>::new()
        }
    }

    fn contours(&self) -> Vec<_Contour> {
        match &self.glyph.outline {
            Some(outline) => outline.contours.iter().map(|c| _Contour::from(c.clone())).collect(),
            None => Vec::<_Contour>::new()
        }
    }

    // fn copy


    // fn draw XXX Implement this in Python wrapper

    #[allow(non_snake_case)]
    fn drawPoints(&self, pen: PyObject) {
      if let Some(o) = &self.glyph.outline {
        let gil = Python::acquire_gil();
        let py = gil.python();

        for c in &o.contours {
            if let Err(e) = pen.call_method0(py, "beginPath") {
                e.restore(py);
                return
            }
            for p in &c.points {
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
        for c in &o.components {
            let transform = (c.transform.x_scale, c.transform.xy_scale, c.transform.yx_scale, c.transform.y_scale, c.transform.x_offset, c.transform.y_offset);
            pen.call_method1(py, "addComponent", (c.base.to_object(py),transform.to_object(py))).unwrap();

        }
      }
    }

    fn width(&self) -> Option<f32> {
        self.glyph.advance_width()
    }

}

#[pyproto]
impl PySequenceProtocol for _Glyph {
    fn __len__(&self) -> PyResult<usize> {
      match &self.glyph.outline {
        Some(outline) => Ok(outline.contours.len()),
        None => Ok(0)
      }
    }
}
