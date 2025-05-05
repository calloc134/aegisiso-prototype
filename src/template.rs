use crate::config::IsoSection;
use askama::Template;

#[derive(Template)]
#[template(
    source = r#"
set default=0
set timeout=5
menuentry 'Arch Linux Live { version }' {
    linux /vmlinuz-linux archisobasedir=arch archisolabel={ name }
    initrd /initramfs-linux.img
}
"#,
    ext = "txt"
)]
#[allow(dead_code)]
struct GrubTemplate<'a> {
    name: &'a str,
    version: &'a str,
}

/// Render GRUB configuration from IsoSection
pub fn render_grub(iso: &IsoSection) -> Result<String, crate::error::ArchisoError> {
    GrubTemplate {
        name: &iso.name,
        version: &iso.version,
    }
    .render()
    .map_err(crate::error::ArchisoError::Template)
}
