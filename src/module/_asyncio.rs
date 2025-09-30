use pyo3::prelude::*;

#[pymodule]
pub mod asyncio {
    use super::{super::*, *};

    #[pymodule_export]
    use _events::events;

    #[pymodule_export]
    use _runners::runners;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Arbitrary code to run at the module initialization
        Ok(())
    }
}
