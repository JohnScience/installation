use super::{ArtifactKind, artifact::SupportedWindowsArch};

pub trait SupportedWindowsArchExt {
    fn get_opt_target_infix(&self) -> &'static str;
}

impl SupportedWindowsArchExt for SupportedWindowsArch {
    fn get_opt_target_infix(&self) -> &'static str {
        match self {
            SupportedWindowsArch::X86 => "",
            SupportedWindowsArch::X64 => "-amd64",
            SupportedWindowsArch::Arm64 => "-arm64",
        }
    }
}

/// # Arguments
/// 
/// For Windows, this is an architecture like "-amd64" or "", which implies x86.
/// For MacOS, this is a "-macos11"
pub(super) fn get_installation_url(
    major: u8,
    minor: u8,
    micro: u8,
    artifact_kind: ArtifactKind,
) -> String {
    let (opt_target_infix, opt_embed_infix, ext) = match artifact_kind {
        ArtifactKind::WindowsInstaller(arch) => {
            let opt_target_infix = arch.get_opt_target_infix();
            (opt_target_infix, "", "exe")
        },
        ArtifactKind::WindowsEmbeddableZipPackage(arch) => {
            let opt_target_infix = arch.get_opt_target_infix();
            (opt_target_infix, "-embed", "zip")
        },
        ArtifactKind::MacOS64BitUniversal2Installer => ("-macos11", "", "pkg"),
    };
    format!("https://www.python.org/ftp/python/{major}.{minor}.{micro}/python-{major}.{minor}.{micro}{opt_embed_infix}{opt_target_infix}.{ext}")
}
