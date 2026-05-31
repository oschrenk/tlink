use std::path::PathBuf;

/// Embed the notification logo at compile time.
const LOGO_BYTES: &[u8] = include_bytes!("../../assets/notification-logo.png");

/// Returns the path to the notification logo, extracting it to disk
/// on first access. The icon lives at `~/.tlink/notification-logo.png`.
pub fn ensure_icon() -> std::io::Result<PathBuf> {
    let tlink_dir = dirs::home_dir()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "home dir not found"))?
        .join(".tlink");

    // Create ~/.tlink if it doesn't exist.
    std::fs::create_dir_all(&tlink_dir)?;

    let icon_path = tlink_dir.join("notification-logo.png");

    // Write the icon only if it doesn't already exist (idempotent).
    if !icon_path.exists() {
        std::fs::write(&icon_path, LOGO_BYTES)?;
    }

    Ok(icon_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logo_bytes_is_not_empty() {
        assert!(!LOGO_BYTES.is_empty());
    }

    #[test]
    fn logo_bytes_looks_like_png() {
        assert_eq!(&LOGO_BYTES[..4], b"\x89PNG");
    }

    #[test]
    fn ensure_icon_creates_file_in_tlink_dir() {
        let path = ensure_icon().expect("should create icon");
        assert!(path.exists());
        assert!(path.ends_with(".tlink/notification-logo.png"));
    }
}
