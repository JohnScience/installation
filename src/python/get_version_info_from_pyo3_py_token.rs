use pyo3::{prelude::*, types::PyTuple, PyDowncastError};
use thiserror::Error;

use super::PythonVersionInfo;

#[derive(Error, Debug)]
pub enum Error<'py> {
    #[error(
        "Py error when calling `installation::python::get_version_info_from_pyo3_py_token`: {0}"
    )]
    PyError(#[from] PyErr),
    #[error("Python version info returned not a tuple: {0}")]
    VersionInfoReturnedNotATuple(PyDowncastError<'py>),
    // Can be there more errors?
}

pub fn get_version_info_from_pyo3_py_token<'py>(
    py: &'py Python,
) -> Result<PythonVersionInfo, Error<'py>> {
    let sys = py.import("sys")?;
    let version_info = sys
        .getattr("version_info")?
        .downcast::<PyTuple>()
        .map_err(Error::VersionInfoReturnedNotATuple)?;
    Ok::<_, PyErr>(PythonVersionInfo {
        major: version_info.get_item(0)?.extract::<u8>()?,
        minor: version_info.get_item(1)?.extract::<u8>()?,
        micro: version_info.get_item(2)?.extract::<u8>()?,
        releaselevel: version_info.get_item(3)?.extract::<String>()?,
        serial: version_info.get_item(4)?.extract::<u64>()?,
    })
    .map_err(Error::from)
}
