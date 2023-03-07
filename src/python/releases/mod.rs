mod artifact;
mod get_installation_url;

pub use artifact::{ArtifactKind, SupportedWindowsArch::{Arm64, X64, X86}};
use get_installation_url::get_installation_url;


pub struct GenRelease<const SUPPORTS_WINDOWS_ARM64: bool> {
    major: u8,
    minor: u8,
    micro: u8,
}

/// A common denominator for all releases.
///
/// All releases are assumed to support Windows with amd64 and x86 architectures.
pub struct CommonDenRelease {
    major: u8,
    minor: u8,
    micro: u8,
    supports_windows_arm64: bool,
}

impl CommonDenRelease {
    pub const fn const_from<const SUPPORTS_WINDOWS_ARM64: bool>(
        release: &GenRelease<SUPPORTS_WINDOWS_ARM64>,
    ) -> Self {
        Self {
            major: release.major,
            minor: release.minor,
            micro: release.micro,
            supports_windows_arm64: SUPPORTS_WINDOWS_ARM64,
        }
    }
}

impl<const SUPPORTS_WINDOWS_ARM64: bool> GenRelease<SUPPORTS_WINDOWS_ARM64> {
    /// Returns a URL to the installer for the given artifact kind.
    fn get_artifact_url(&self, artifact_kind: ArtifactKind) -> String {
        get_installation_url(self.major, self.minor, self.micro, artifact_kind)
    }

}

impl GenRelease</*SUPPORTS_WINDOWS_ARM64*/ true> {
    fn get_windows_arm64_installer_url(&self) -> String {
        self.get_artifact_url( ArtifactKind::WindowsInstaller(Arm64))
    }

    fn get_windows_arm64_embeddable_zip_package_url(&self) -> String {
        self.get_artifact_url(ArtifactKind::WindowsEmbeddableZipPackage(Arm64))
    }
}

pub mod constants {
    use super::GenRelease;
    const SUPPORTS_WINDOWS_ARM64: bool = true;
    const DOES_NOT_SUPPORT_WINDOWS_ARM64: bool = false;

    pub const PYTHON_3_10_10: GenRelease<SUPPORTS_WINDOWS_ARM64> = GenRelease {
        major: 3,
        minor: 10,
        micro: 10,
    };
}
