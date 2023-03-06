use thiserror::Error;
use pyo3::prelude::*;

use super::PythonVersionInfo;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Py error when calling `installation::python::get_version_info_from_pyo3_py_token`: {0}")]
    PyError(#[from] PyErr),
    #[error("Python version info returned not a tuple")]
    VersionInfoReturnedNotATuple,
    // Can be there more errors?
}

impl<'py> From<super::get_version_info_from_pyo3_py_token::Error<'py>> for Error {
    fn from(err: super::get_version_info_from_pyo3_py_token::Error<'py>) -> Self {
        match err {
            super::get_version_info_from_pyo3_py_token::Error::PyError(err) => Self::PyError(err),
            super::get_version_info_from_pyo3_py_token::Error::VersionInfoReturnedNotATuple(_err) => Self::VersionInfoReturnedNotATuple,
        }
    }
}

pub fn get_version_info_via_pyo3() -> Result<PythonVersionInfo, Error> {
    Python::with_gil(|py| {
        super::get_version_info_from_pyo3_py_token::get_version_info_from_pyo3_py_token(&py).map_err(Error::from)
    })
}
