use crate::Command;
use crate::Command::{ExitProgram, UnmatchedCommand};

pub struct ConsoleCommandParser {
    _sealed: ()
}

impl ConsoleCommandParser {
    pub fn new() -> Self {
        ConsoleCommandParser{
            _sealed: ()
        }
    }

    pub fn parse_line(&self, line: &str) -> Command {
       match line.trim() {
           "q" | "exit" => ExitProgram,
           _ => UnmatchedCommand(String::from(line.trim())),
       }
    }
}