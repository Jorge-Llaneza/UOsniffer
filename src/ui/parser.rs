use crate::commands::Command;

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
           "q" | "exit" => Command::ExitProgram,
           "ranking" => Command::CreateRanking,
           "commands" => Command::ShowAllCommands,
           _ => Command::UnmatchedCommand(String::from(line.trim())),
       }
    }
}