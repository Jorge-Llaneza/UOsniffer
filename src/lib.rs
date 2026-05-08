mod commands;
mod store;
mod ranking;
pub mod ui;

use std::io;

pub fn run_client(interactor: impl Interactor) -> io::Result<()> {
    interactor.show_initial_message()?;

    loop {

        let command = interactor.ask_command()?;

        match execute_command(&command) {
            Ok(result) => interactor.show_command_result(result)?,
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
    fn ask_command(&self) -> io::Result<Command>;
    fn show_command_result(&self, result: String) -> io::Result<()>;
    fn show_error_message(&self, message: String) -> io::Result<()>;
    fn show_fatal_error_message(&self, message: String) -> io::Result<()>;
}

fn execute_command(command: &Command) -> Result<String, String> {
    match command {
        Command::ExitProgram => Err(String::from("Closing UOsniffer")),
        Command::CreateRanking => commands::create_ranking(),
        Command::UnmatchedCommand(s) => Ok(format!(r#"Unknown command: "{}""#, s))
    }
}


pub enum Command {
    ExitProgram,
    CreateRanking,
    UnmatchedCommand(String),
}