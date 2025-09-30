use pyo3::prelude::*;

#[pymodule]
pub mod runners {
    use super::{super::*, *};

    #[pyfunction]
    #[pyo3(signature = (coro, *, debug=None, loop_factory=None))]
    pub fn run(
        py: Python<'_>,
        coro: Bound<'_, PyAny>,
        debug: Option<bool>,
        loop_factory: Option<Bound<'_, PyAny>>,
    ) -> PyResult<Py<PyAny>> {
        Ok(py.None())
    }

    #[derive(PartialEq)]
    enum RunnerState {
        Created,
        Initialized,
        Closed,
    }

    #[pyclass]
    pub struct Runner {
        state: RunnerState,
        debug: Option<bool>,
        loop_factory: Option<Py<PyAny>>,
        r#loop: Option<Py<events::EventLoop>>,
        set_event_loop: bool,
        context: Option<Py<PyAny>>,
        interrupt_count: usize,
    }

    impl Runner {
        fn lazy_init(&mut self, py: Python<'_>) -> PyResult<()> {
            match &mut self.state {
                RunnerState::Created => {
                    let r#loop = match &self.loop_factory {
                        Some(loop_factory) => {
                            let r#loop = loop_factory.bind(py).call0()?.downcast_into()?;
                            self.r#loop = Some(r#loop.clone().unbind());
                            r#loop
                        }
                        None => {
                            let r#loop = events::new_event_loop(py)?.into_bound(py);
                            self.r#loop = Some(r#loop.clone().unbind());
                            if !self.set_event_loop {
                                events::set_event_loop(py, r#loop.clone().unbind())?;
                                self.set_event_loop = true;
                            }

                            r#loop
                        }
                    };

                    if let Some(debug) = self.debug {
                        r#loop.borrow_mut().set_debug(debug);
                    }

                    let contextvars = py.import("contextvars")?;

                    self.context = Some(contextvars.getattr("copy_context")?.call0()?.unbind());

                    self.state = RunnerState::Initialized;
                    Ok(())
                }
                RunnerState::Closed => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Runner is closed",
                )),
                RunnerState::Initialized => Ok(()),
            }
        }
    }

    #[pymethods]
    impl Runner {
        #[pyo3(signature = (*, debug=None, loop_factory=None))]
        fn __init__(this: Bound<'_, Self>, debug: Option<bool>, loop_factory: Option<Py<PyAny>>) {
            let mut this = this.borrow_mut();
            this.state = RunnerState::Created;
            this.debug = debug;
            this.loop_factory = loop_factory;
            this.r#loop = None;
            this.context = None;
            this.interrupt_count = 0;
            this.set_event_loop = false;
        }

        fn __enter__(this: Bound<'_, Self>) -> PyResult<Bound<'_, Self>> {
            this.borrow_mut().lazy_init(this.py())?;
            Ok(this)
        }
    }
}
