use super::adapter::{NotificationAdapter, NotificationRequest};
use super::utils::applescript_escape;
use anyhow::Result;

pub struct OsascriptAdapter;

impl OsascriptAdapter {
    pub fn build_script(&self, req: &NotificationRequest) -> String {
        // `with icon` requires macOS 11+ (Big Sur) and a path to an image file.
        // When the icon path is empty (fallback), we omit the clause.
        let icon_clause = if req.icon_path.is_empty() {
            String::new()
        } else {
            format!(
                " with icon POSIX file \"{}\"",
                applescript_escape(&req.icon_path),
            )
        };
        format!(
            "display notification \"{}\" with title \"{}\" subtitle \"{} @ {}\"{} sound name \"Glass\"\n\
             open location \"{}\"",
            applescript_escape(&req.message),
            applescript_escape(&req.title),
            applescript_escape(&req.session),
            applescript_escape(&req.location),
            icon_clause,
            applescript_escape(&req.deeplink),
        )
    }
}

impl NotificationAdapter for OsascriptAdapter {
    fn name(&self) -> &str {
        "osascript"
    }

    fn notify(&self, req: &NotificationRequest) -> Result<()> {
        let script = self.build_script(req);
        std::process::Command::new("osascript")
            .args(["-e", &script])
            .status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> NotificationRequest {
        NotificationRequest {
            title: "Title".into(),
            message: "Msg".into(),
            location: "s > w > 0".into(),
            deeplink: "tmux://s/w/0".into(),
            session: "s".into(),
            icon_path: "/tmp/tlink-logo.png".into(),
        }
    }

    #[test]
    fn build_script_contains_display_notification() {
        let s = OsascriptAdapter.build_script(&req());
        assert!(s.contains("display notification"));
    }

    #[test]
    fn build_script_contains_title_and_message() {
        let s = OsascriptAdapter.build_script(&req());
        assert!(s.contains("\"Msg\""));
        assert!(s.contains("\"Title\""));
    }

    #[test]
    fn build_script_subtitle_includes_session() {
        let s = OsascriptAdapter.build_script(&req());
        // subtitle is "s @ s > w > 0" (session @ location)
        assert!(s.contains("s @ s > w > 0"));
    }

    #[test]
    fn build_script_contains_open_location_for_deeplink() {
        let s = OsascriptAdapter.build_script(&req());
        assert!(s.contains("open location"));
        assert!(s.contains("tmux://s/w/0"));
    }

    #[test]
    fn build_script_includes_icon_when_path_set() {
        let s = OsascriptAdapter.build_script(&req());
        assert!(s.contains("with icon"));
        assert!(s.contains("POSIX file"));
        assert!(s.contains("/tmp/tlink-logo.png"));
    }

    #[test]
    fn build_script_omits_icon_when_path_empty() {
        let r = NotificationRequest {
            icon_path: String::new(),
            ..req()
        };
        let s = OsascriptAdapter.build_script(&r);
        assert!(!s.contains("with icon"));
    }

    #[test]
    fn build_script_escapes_special_chars() {
        let r = NotificationRequest {
            title: r#"Say "hi""#.into(),
            message: r"foo\bar".into(),
            location: "s > w > 0".into(),
            deeplink: "tmux://s/w/0".into(),
            session: "s".into(),
            icon_path: "/tmp/tlink-logo.png".into(),
        };
        let s = OsascriptAdapter.build_script(&r);
        assert!(s.contains(r#"\"hi\""#));
        assert!(s.contains(r"foo\\bar"));
    }

    #[test]
    fn name_returns_osascript() {
        assert_eq!(OsascriptAdapter.name(), "osascript");
    }
}
