use super::adapter::{NotificationAdapter, NotificationRequest};
use super::utils::sh_quote;
use anyhow::Result;

pub struct DunstifyAdapter;

impl DunstifyAdapter {
    pub fn build_script(&self, req: &NotificationRequest) -> String {
        let appname = format!("tlink ({})", req.session);
        let icon = if req.icon_path.is_empty() {
            "utilities-terminal"
        } else {
            req.icon_path.as_str()
        };
        format!(
            "ACTION=$(dunstify {t} {m} --action='default,Go there' \
                --urgency=normal --icon={i} --appname={a}); \
             [ \"$ACTION\" = \"default\" ] && tlink open {dl}",
            t = sh_quote(&req.title),
            m = sh_quote(&req.message),
            a = sh_quote(&appname),
            i = sh_quote(icon),
            dl = sh_quote(&req.deeplink),
        )
    }
}

impl NotificationAdapter for DunstifyAdapter {
    fn name(&self) -> &str {
        "dunstify"
    }

    fn notify(&self, req: &NotificationRequest) -> Result<()> {
        let cmd = self.build_script(req);
        std::process::Command::new("sh")
            .args(["-c", &cmd])
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> NotificationRequest {
        NotificationRequest {
            title: "Claude".into(),
            message: "Done".into(),
            location: "s > w > 0".into(),
            deeplink: "tmux://s/w/0".into(),
            session: "mysession".into(),
            icon_path: "/tmp/tlink-logo.png".into(),
        }
    }

    #[test]
    fn build_script_calls_dunstify() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("dunstify"));
    }

    #[test]
    fn build_script_has_click_action() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("tlink open"));
        assert!(s.contains("\"default\""));
    }

    #[test]
    fn build_script_includes_title_and_message() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("'Claude'"));
        assert!(s.contains("'Done'"));
    }

    #[test]
    fn build_script_includes_session_in_appname() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("tlink (mysession)"));
        assert!(s.contains("--appname="));
    }

    #[test]
    fn build_script_uses_tlink_icon_when_path_set() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("/tmp/tlink-logo.png"));
        assert!(!s.contains("utilities-terminal"));
    }

    #[test]
    fn build_script_falls_back_to_generic_icon_when_path_empty() {
        let r = NotificationRequest {
            icon_path: String::new(),
            ..req()
        };
        let s = DunstifyAdapter.build_script(&r);
        assert!(s.contains("utilities-terminal"));
        assert!(!s.contains("/tmp/tlink-logo.png"));
    }

    #[test]
    fn build_script_deeplink_passed() {
        let s = DunstifyAdapter.build_script(&req());
        assert!(s.contains("tmux://s/w/0"));
    }

    #[test]
    fn name_returns_dunstify() {
        assert_eq!(DunstifyAdapter.name(), "dunstify");
    }
}
