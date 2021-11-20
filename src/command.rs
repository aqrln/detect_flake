pub struct CommandParser {
    command_parts: Vec<String>,
}

impl CommandParser {
    pub fn from_command(cmd: &str) -> Self {
        Self {
            command_parts: cmd.split(" ").map(|s| s.to_owned()).collect(),
        }
    }

    pub fn program(&self) -> &str {
        &self.command_parts[0]
    }

    pub fn args(&self) -> &[String] {
        &self.command_parts[1..]
    }

    pub fn to_parts_owned(&self) -> (String, Vec<String>) {
        (self.program().to_owned(), self.args().to_owned())
    }
}
