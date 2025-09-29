use pyo3::prelude::*;

use super::*;

#[pymodule]
pub mod asyncio {
    use super::*;

    #[pymodule_export]
    use events::module as events;

    #[pymodule_export]
    use runners::module as runners;

    #[pymodule_init]
    fn init(_m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Arbitrary code to run at the module initialization
        Ok(())
    }
}
