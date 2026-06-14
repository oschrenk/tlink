mod wizard;

use anyhow::{bail, Result};

/// Arguments passed from the CLI for the setup command.
#[derive(Debug, Default)]
pub struct SetupArgs {
    /// Terminal emulator (e.g. "iTerm2", "Ghostty")
    pub terminal: Option<String>,
    /// Skip all prompts (non-interactive mode)
    pub yes: bool,
    /// Enable anonymous telemetry
    pub telemetry: bool,
    /// Disable anonymous telemetry
    pub no_telemetry: bool,
}

/// Entry point for `tlink setup`.
///
/// If any non-interactive flag is provided (`--terminal` or `--yes`), runs
/// without the TUI wizard. Otherwise launches the interactive wizard.
pub fn run(args: SetupArgs) -> Result<()> {
    let is_non_interactive = args.terminal.is_some() || args.yes;

    if is_non_interactive {
        run_non_interactive(&args)
    } else {
        run_interactive()
    }
}

/// Non-interactive setup: uses CLI args to skip the wizard.
fn run_non_interactive(args: &SetupArgs) -> Result<()> {
    // Resolve the terminal name
    let terminal_name = resolve_terminal(args)?;

    // Handle telemetry preference
    apply_telemetry(args);

    // Save the terminal config
    let config = crate::config::Config {
        terminal: Some(terminal_name.clone()),
        ..crate::config::load().unwrap_or_default()
    };
    crate::config::save(&config)?;

    // Compile and register the URI scheme handler
    if let Err(e) = crate::bundle::create() {
        bail!("Bundle creation failed: {e:#}");
    }

    // Verify the scheme was registered
    let success = wizard::verify_scheme();
    if success {
        println!("Setup complete! Terminal: {terminal_name}");
        println!("Run `tlink status` to verify, or `open tmux://session` to test.");
    } else {
        println!(
            "Setup complete (handler compiled), but tmux:// scheme verification was inconclusive."
        );
        println!("Terminal: {terminal_name}");
        println!("The handler may activate after relogin.");
        println!("Run `tlink status` to verify, or `open tmux://session` to test.");
    }

    Ok(())
}

/// Interactive setup: launches the TUI wizard.
fn run_interactive() -> Result<()> {
    match wizard::run_wizard()? {
        Some(terminal) => {
            println!("Setup complete! Terminal: {terminal}");
            println!("Run `tlink status` to verify, or `open tmux://session` to test.");
        }
        None => {
            println!("Setup cancelled.");
        }
    }
    Ok(())
}

/// Resolve the terminal name from CLI args.
fn resolve_terminal(args: &SetupArgs) -> Result<String> {
    match &args.terminal {
        Some(name) => {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                bail!("--terminal requires a non-empty value");
            }
            Ok(trimmed.to_string())
        }
        None => {
            // --yes without --terminal: try to auto-detect
            let detected = wizard::detect_terminals();
            match detected.len() {
                0 => bail!(
                    "No terminal emulators found in /Applications.\n\
                     Specify one with: --terminal <NAME>\n\
                     Known terminals: {}",
                    wizard::KNOWN_TERMINALS.join(", ")
                ),
                1 => {
                    let name = detected.into_iter().next().unwrap();
                    println!("Auto-detected terminal: {name}");
                    Ok(name)
                }
                _ => bail!(
                    "Multiple terminal emulators found: {}\n\
                     Specify one with: --terminal <NAME>",
                    detected.join(", ")
                ),
            }
        }
    }
}

/// Apply telemetry preference from CLI args.
/// Saves the decision to config so it persists.
fn apply_telemetry(args: &SetupArgs) {
    match (args.telemetry, args.no_telemetry) {
        (true, false) => {
            let mut cfg = crate::config::load().unwrap_or_default();
            cfg.telemetry_enabled = Some(true);
            let _ = crate::config::save(&cfg);
        }
        (false, true) => {
            let mut cfg = crate::config::load().unwrap_or_default();
            cfg.telemetry_enabled = Some(false);
            let _ = crate::config::save(&cfg);
        }
        (false, false) => {
            // Neither flag set: default to disabled for non-interactive mode
            let mut cfg = crate::config::load().unwrap_or_default();
            if cfg.telemetry_enabled.is_none() {
                cfg.telemetry_enabled = Some(false);
                let _ = crate::config::save(&cfg);
            }
        }
        (true, true) => unreachable!("clap enforces conflicts_with"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_terminal_from_arg() {
        let args = SetupArgs {
            terminal: Some("Ghostty".into()),
            ..Default::default()
        };
        assert_eq!(resolve_terminal(&args).unwrap(), "Ghostty");
    }

    #[test]
    fn test_resolve_terminal_trims() {
        let args = SetupArgs {
            terminal: Some("  iTerm2  ".into()),
            ..Default::default()
        };
        assert_eq!(resolve_terminal(&args).unwrap(), "iTerm2");
    }

    #[test]
    fn test_resolve_terminal_empty_rejected() {
        let args = SetupArgs {
            terminal: Some("   ".into()),
            ..Default::default()
        };
        assert!(resolve_terminal(&args).is_err());
    }

    #[test]
    fn test_is_non_interactive_with_terminal() {
        let args = SetupArgs {
            terminal: Some("Ghostty".into()),
            ..Default::default()
        };
        assert!(args.terminal.is_some() || args.yes);
    }

    #[test]
    fn test_is_non_interactive_with_yes() {
        let args = SetupArgs {
            yes: true,
            ..Default::default()
        };
        assert!(args.terminal.is_some() || args.yes);
    }
}
