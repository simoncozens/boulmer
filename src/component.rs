use pyo3::prelude::*;

#[pyclass(subclass)]
#[derive(Clone, Debug)]
pub struct _Component {
    pub component: norad::Component,
}

impl From<norad::Component> for _Component {
    fn from(component: norad::Component) -> Self {
        Self { component }
    }
}
