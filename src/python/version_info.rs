/// The result of [sys.version_info()] call.
/// 
/// Do not confuse with [pyo3::PythonVersionInfo].
/// 
/// [sys.version_info()]: https://docs.python.org/3/library/sys.html#sys.version_info
#[derive(Debug)]
pub struct PythonVersionInfo {
    pub major: u8,
    pub minor: u8,
    pub micro: u8,
    pub releaselevel: String,
    pub serial: u64,
}
