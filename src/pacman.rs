use crate::config::PacmanSection;
use crate::error::ArchisoError;
use async_process::{Command, Stdio};
use std::path::Path;

pub async fn install_official(pac: &PacmanSection, work_dir: &Path) -> Result<(), ArchisoError> {
    let rootfs = work_dir.join("airootfs");
    std::fs::create_dir_all(&rootfs)?;

    // Execute pacstrap inheriting stdio so you see stdout/stderr in real time
    let status = Command::new("pacstrap")
        .arg("-C")
        .arg("/etc/pacman.conf")
        .arg("-c")
        .arg(&rootfs)
        .args(&pac.packages)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await?;

    if !status.success() {
        return Err(ArchisoError::Process(format!(
            "pacstrap failed: {}",
            status
        )));
    }

    Ok(())
}
