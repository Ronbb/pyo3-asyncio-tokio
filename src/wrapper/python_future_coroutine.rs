use std::{
    pin::Pin,
    task::{Context, Poll, Waker},
};

use pyo3::{exceptions::PyStopIteration, prelude::*};

enum PythonFutureState {
    Pending {
        future: Pin<Box<dyn Future<Output = PyResult<Py<PyAny>>> + Send + Sync>>,
        waker: Option<Waker>,
    },
    Ready(Py<PyAny>),
    Err(Py<PyAny>),
    Closed,
}

/// A Python coroutine that wraps a Rust future.
#[pyclass]
pub struct PythonFutureCoroutine {
    state: PythonFutureState,
}

impl PythonFutureCoroutine {
    pub fn new(future: Pin<Box<dyn Future<Output = PyResult<Py<PyAny>>> + Send + Sync>>) -> Self {
        Self {
            state: PythonFutureState::Pending {
                future,
                waker: None,
            },
        }
    }
}

#[pymethods]
impl PythonFutureCoroutine {
    fn send(&mut self, py: Python<'_>, _value: &Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
        self.__next__(py)
    }

    fn throw(&mut self, py: Python<'_>, exc: Py<PyAny>) -> PyResult<Py<PyAny>> {
        match &mut self.state {
            PythonFutureState::Closed => Err(PyErr::new::<pyo3::exceptions::PySystemError, _>(
                "Coroutine is closed",
            )),
            _ => {
                self.state = PythonFutureState::Err(exc.clone_ref(py));
                Err(PyErr::from_value(exc.into_bound(py)))
            }
        }
    }

    fn close(&mut self) {
        self.state = PythonFutureState::Closed;
    }

    fn __await__(this: Py<Self>) -> Py<Self> {
        this
    }

    fn __next__(&mut self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        match &mut self.state {
            PythonFutureState::Pending { future, waker } => {
                let waker = waker.get_or_insert_with(|| Waker::noop().clone());
                let mut cx = Context::from_waker(waker);
                let result = future.as_mut().poll(&mut cx);
                match result {
                    Poll::Ready(Ok(value)) => {
                        self.state = PythonFutureState::Ready(value.clone_ref(py));
                        Ok(value)
                    }
                    Poll::Ready(Err(e)) => {
                        self.state =
                            PythonFutureState::Err(e.clone_ref(py).into_value(py).into_any());
                        Err(e)
                    }
                    Poll::Pending => Ok(py.None()),
                }
            }
            PythonFutureState::Ready(value) => Err(PyStopIteration::new_err(value.clone_ref(py))),
            PythonFutureState::Err(e) => Err(PyErr::from_value(e.clone_ref(py).into_bound(py))),
            PythonFutureState::Closed => Err(PyErr::new::<pyo3::exceptions::PySystemError, _>(
                "Coroutine is closed",
            )),
        }
    }
}
