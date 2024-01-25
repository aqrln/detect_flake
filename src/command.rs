use anyhow::{anyhow, Result};

pub struct ParsedCommand(Vec<String>);

impl ParsedCommand {
    pub fn parse(cmd: &str) -> Result<Self> {
        shlex::split(cmd)
            .map(Self)
            .ok_or_else(|| anyhow!("failed to parse the command: '{cmd}'"))
    }

    pub fn program(&self) -> &str {
        &self.0[0]
    }

    pub fn args(&self) -> &[String] {
        &self.0[1..]
    }

    pub fn to_parts_owned(&self) -> (String, Vec<String>) {
        (self.program().to_owned(), self.args().to_owned())
    }
}
