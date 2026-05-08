use std::str::FromStr;
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
        let command = first_word(line);
       match command {
           "q" | "exit" => Command::ExitProgram,
           "ranking" => self.parse_ranking_commands(line),
           "commands" => Command::ShowAllCommands,
           _ => Command::UnmatchedCommand(String::from(command)),
       }
    }

    fn parse_ranking_commands(&self, line: &str) -> Command {
        let parts = line.split(' ');
        let mut min_exams_taken = 0;
        for option in parts {
            if option.starts_with("--min-exams-taken=") {
                if let Some(parts) = option.split_once('=') {
                    if let Ok(number) = u32::from_str(parts.1.trim()){
                        min_exams_taken = number;
                    }
                }
            }
        }
        Command::CreateRanking(CreateRankingOptions::with_min_exams_taken(min_exams_taken))
    }
}

fn first_word(line: &str) -> &str {
    if let Some(parts) = line.trim().split_once(' ') {
        parts.0.trim()
    } else {
        line.trim()
    }
}