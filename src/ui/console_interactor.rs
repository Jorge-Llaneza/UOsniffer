use std::io;
use std::io::Write;
use crate::{Command, Interactor};
use crate::ui::parser::ConsoleCommandParser;

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

    fn ask_command(&self) -> io::Result<Command> {
        print!(">>> ");
        io::stdout().flush()?;

        let mut incoming_command = String::new();
        io::stdin().read_line(&mut incoming_command)?;

        let parser = ConsoleCommandParser::new();
        Ok(parser.parse_line(&incoming_command))
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


