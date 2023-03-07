use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error when calling `installation::python::get_version_from_cli`: {0}")]
    IoError(#[from] std::io::Error),
    // Can be there more errors?
}

/// A newtype wrapper around [`std::process::Output`] to to implement
/// parsing and so on.
#[derive(Debug)]
pub struct VersionOutput(pub std::process::Output);

pub fn get_version_from_cli() -> Result<VersionOutput, Error> {
    std::process::Command::new("python")
        .arg("--version")
        .output()
        .map(VersionOutput)
        .map_err(Error::from)
}
