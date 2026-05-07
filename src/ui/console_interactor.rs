use std::io;
use std::io::Write;
use std::str::FromStr;
use crate::{Command, Interactor};

pub struct ConsoleInteractor {

}

const WELCOME_MESSAGE: &str = "Welcome to the note sniffer CLI
To see a list of the commands type commands
To get help about a command type help <Command>";
impl Interactor for ConsoleInteractor {
    fn show_initial_message(&self) -> io::Result<()> {
        println!("{}", WELCOME_MESSAGE);
        Ok(())
    }

    fn ask_command(&self) -> io::Result<(Command, String)> {
        print!(">>> ");
        io::stdout().flush()?;

        let mut incoming_command = String::new();
        io::stdin().read_line(&mut incoming_command)?;
        let options: String;
        let command = match incoming_command.trim().split_once(" ") {
            Some((command, o)) => {
                options = String::from(o);
                parse_command(command)
            },
            None => {
                options = String::new();
                parse_command(&incoming_command)
            },
        };
        Ok((command, options))
    }

    fn show_command_result(&self, result: String) -> io::Result<()> {
        println!("{}", result);
        Ok(())
    }

    fn show_error_message(&self, message: String) -> io::Result<()> {
        println!("{}", message);
        Ok(())
    }

    fn show_fatal_error_message(&self, message: String) -> io::Result<()> {
        println!("{}", message);
        Ok(())
    }
}

/// trims the string before conversion
fn parse_command(command: &str) -> Command {
    Command::from_str(command).unwrap()
}

