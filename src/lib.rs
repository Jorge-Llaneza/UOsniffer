mod commands;
mod store;
mod ranking;
pub mod ui;

use std::io;
use commands::Command;

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

const COMMAND_LIST: &str = "exit (q): exit program
ranking: create ranking";
fn execute_command(command: &Command) -> Result<String, String> {
    match command {
        Command::ExitProgram => Err(String::from("Closing UOsniffer")),
        Command::ShowAllCommands => Ok(String::from(COMMAND_LIST)),
        Command::CreateRanking(options) => commands::create_ranking(op),
        Command::UnmatchedCommand(s) => Ok(format!(r#"Unknown command: "{}""#, s))
    }
}


