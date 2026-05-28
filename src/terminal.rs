pub struct TerminalAdapter { pub name: String }
pub fn from_name(name: &str) -> TerminalAdapter {
    TerminalAdapter { name: name.to_string() }
}
impl TerminalAdapter {
    pub fn focus(&self) -> anyhow::Result<()> { Ok(()) }
}
