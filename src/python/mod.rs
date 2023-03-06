mod get_version_from_cli;
pub use get_version_from_cli::get_version_from_cli;
#[cfg(feature = "pyo3")]
mod get_version_info_via_pyo3;
#[cfg(feature = "pyo3")]
mod get_version_info_from_pyo3_py_token;
#[cfg(feature = "pyo3")]
mod version_info;
#[cfg(feature = "pyo3")]
pub use {
    get_version_info_via_pyo3::get_version_info_via_pyo3,
    get_version_info_from_pyo3_py_token::get_version_info_from_pyo3_py_token,
    version_info::PythonVersionInfo,
};

// const KNOWN_IMPLEMENTATIONS: &[&str] = &[
//     // https://github.com/python/cpython
//     "CPython",
//     // https://www.jython.org/
//     "Jython",
//     // https://ironpython.net/
//     "IronPython",
//     // https://www.pypy.org/
//     "PyPy",
//     // https://github.com/stackless-dev/stackless/wiki/
//     "Stackless",
// ];
