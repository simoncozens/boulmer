
use crate::component::Component;
use std::sync::Arc;
use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyo3::PyResult;


#[pyclass(subclass)]
#[derive(Clone,Debug)]
pub struct Glyph {
    pub glyph: Arc<norad::Glyph>,
}

impl From<Arc<norad::Glyph>> for Glyph {
    fn from(glyph: Arc<norad::Glyph>) -> Self {
        Self{glyph}
    }
}


#[pymethods]
impl Glyph {
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

    fn components(&self) -> Vec<Component> {
        match &self.glyph.outline {
            Some(outline) => outline.components.iter().map(|c| Component::from(c)).collect(),
            None => Vec::<Component>::new()
        }

    }
}

#[pyproto]
impl PySequenceProtocol for Glyph {
    fn __len__(&self) -> PyResult<usize> {
      match &self.glyph.outline {
        Some(outline) => Ok(outline.contours.len()),
        None => Ok(0)
      }
    }
}
