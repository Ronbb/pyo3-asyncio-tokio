use pyo3::prelude::*;

#[pymodule]
pub mod events {
    use super::*;

    #[pyfunction]
    pub fn new_event_loop(py: Python<'_>) -> PyResult<Py<EventLoop>> {
        todo!();
    }

    #[pyfunction]
    pub fn set_event_loop(py: Python<'_>, loop_: Py<EventLoop>) -> PyResult<()> {
        todo!();

        Ok(())
    }

    #[pyclass]
    pub struct EventLoop {
        debug: Option<bool>,
    }

    impl EventLoop {
        pub fn set_debug(&mut self, debug: bool) {
            self.debug = Some(debug);
        }
    }
}
