mod commands;
mod store;
mod ranking;
pub mod ui;

use std::io;
use std::io::Write;
use std::str::FromStr;

pub fn run_client(interactor: impl Interactor) -> io::Result<()> {
    interactor.show_initial_message()?;

    loop {

        let (command, options) = interactor.ask_command()?;

        match execute_command(&command, options
            .split(" ")
            .map(|s| s.to_string() )
            .collect()) {
            Ok(result) => println!("{}", result),
            Err(message) => {
                interactor.show_error_message(message)?;
                break;
            }
        }
    }
    Ok(())
}

pub trait Interactor {
    fn show_initial_message(&self) -> io::Result<()>;
    fn ask_command(&self) -> io::Result<(Command, String)>;
    fn show_command_result(&self, result: String) -> io::Result<()>;
    fn show_error_message(&self, message: String) -> io::Result<()>;
    fn show_fatal_error_message(&self, message: String) -> io::Result<()>;
}

fn execute_command(command: &Command, options: Vec<String>) -> Result<String, String> {
    match command {
        Command::ExitProgram => Err(String::from("Closing UOsniffer")),
        Command::CreateRanking => commands::create_ranking(options),
        Command::UnmatchedCommand(s) => Ok(format!(r#"Unknown command: "{}""#, s))
    }
}


pub enum Command {
    ExitProgram,
    CreateRanking,
    UnmatchedCommand(String),
}

impl FromStr for Command {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "exit" | "q" => Ok(Command::ExitProgram),
            "ranking" => Ok(Command::CreateRanking),
            s => Ok(Command::UnmatchedCommand(s.to_string())),
        }
    }
}