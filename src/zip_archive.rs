use std::path::Path;

use crate::error::Result;

pub fn extract<P: AsRef<Path>>(from: &[u8], to: P) -> Result<()> {
    let cursor = std::io::Cursor::new(from);
    let mut archive = zip::ZipArchive::new(cursor)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let path = file.sanitized_name();
        if file.name().ends_with('/') {
            std::fs::create_dir_all(to.as_ref().join(path))?;
        } else {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(to.as_ref().join(parent))?;
            }

            let mut out = std::fs::File::create(to.as_ref().join(path))?;
            std::io::copy(&mut file, &mut out)?;
        }
    }

    Ok(())
}
