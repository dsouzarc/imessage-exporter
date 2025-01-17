use std::{
    path::Path,
    process::{Command, Stdio},
};

/// Convert a HEIC image file to a JPEG
///
/// This uses the MacOS builtin `sips` program
/// Docs: <https://www.unix.com/man-page/osx/1/sips/> (or `man sips`)
pub fn heic_to_jpeg(from: &Path, to: &Path) -> Option<()> {
    // Get the path we want to copy from
    let from_path = from.to_str()?;

    // Get the path we want to write to
    let to_path = to.to_str()?;

    // Build the comment
    match Command::new("sips")
        .args(&vec!["-s", "format", "jpeg", from_path, "-o", to_path])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .spawn()
    {
        Ok(mut sips) => match sips.wait() {
            Ok(_) => Some(()),
            Err(why) => {
                eprintln!("Conversion failed: {why}");
                None
            }
        },
        Err(why) => {
            eprintln!("Conversion failed: {why}");
            None
        }
    }
}
