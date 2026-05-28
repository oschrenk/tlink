use anyhow::Result;
use std::process::Command;

pub struct TerminalAdapter {
    pub name: String,
}

pub fn from_name(name: &str) -> TerminalAdapter {
    TerminalAdapter { name: name.to_string() }
}

impl TerminalAdapter {
    pub fn focus(&self) -> Result<()> {
        match self.name.as_str() {
            "iTerm2" => applescript_activate("iTerm2"),
            "Terminal" | "Terminal.app" => applescript_activate("Terminal"),
            "Ghostty" => open_app("Ghostty"),
            "Kitty" => open_app("kitty"),
            "WezTerm" => open_app("WezTerm"),
            other => open_app(other),
        }
    }
}

fn applescript_activate(app_name: &str) -> Result<()> {
    let script = format!(r#"tell application "{app_name}" to activate"#);
    Command::new("osascript").args(["-e", &script]).status()?;
    Ok(())
}

fn open_app(app_name: &str) -> Result<()> {
    Command::new("open").args(["-a", app_name]).status()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name_stores_name() {
        let a = from_name("iTerm2");
        assert_eq!(a.name, "iTerm2");
    }

    #[test]
    fn test_from_name_unknown_does_not_panic() {
        let a = from_name("SomeFutureTerminal");
        assert_eq!(a.name, "SomeFutureTerminal");
    }

    #[test]
    fn test_terminal_app_alias_names() {
        let _ = from_name("Terminal");
        let _ = from_name("Terminal.app");
    }
}
