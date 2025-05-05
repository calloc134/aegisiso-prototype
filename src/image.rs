use crate::error::ArchisoError;
use std::path::Path;
use std::process::{Command, Stdio};

/// Create SquashFS image from root directory
pub fn squash(root: &Path, out: &Path) -> Result<(), ArchisoError> {
    let status = Command::new("mksquashfs")
        .arg(root)
        .arg(out)
        .arg("-noappend")
        .arg("-comp")
        .arg("xz")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        return Err(ArchisoError::Process(format!(
            "mksquashfs failed: {}",
            status
        )));
    }
    Ok(())
}

/// Create ISO from work directory
pub fn make_iso(src: &Path, iso: &Path, volid: &str) -> Result<(), ArchisoError> {
    let status = Command::new("xorriso")
        .args(&["-as", "mkisofs", "-o"])
        .arg(iso)
        .args(&["-J", "-R", "-V", volid])
        .arg(src)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        return Err(ArchisoError::Process(format!("xorriso failed: {}", status)));
    }
    Ok(())
}
