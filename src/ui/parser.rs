use crate::commands::{Command, CreateRankingOptions};

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
           "ranking" => Command::CreateRanking(CreateRankingOptions::with_min_exams_taken(3)),
           "commands" => Command::ShowAllCommands,
           _ => Command::UnmatchedCommand(String::from(line.trim())),
       }
    }
}