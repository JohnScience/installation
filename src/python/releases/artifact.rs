pub enum SupportedWindowsArch {
    X86,
    // x86_64 aka amd64
    X64,
    Arm64,
}

pub enum ArtifactKind {
    WindowsInstaller(SupportedWindowsArch),
    WindowsEmbeddableZipPackage(SupportedWindowsArch),
    MacOS64BitUniversal2Installer,
}
