mod commands;
mod store;
mod ranking;

use std::io::Write;
use std::str::FromStr;

const WELCOME_MESSAGE: &str = "Welcome to the note sniffer CLI
To see a list of the commands type commands
To get help about a command type help <Command>";
pub fn run_client() {
    println!("{}", WELCOME_MESSAGE);

    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();

        let mut incoming_command = String::new();
        std::io::stdin().read_line(&mut incoming_command).unwrap();
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

        match execute_command(&command, options
            .split(" ")
            .map(|s| s.to_string() )
            .collect()) {
            Ok(result) => println!("{}", result),
            Err(message) => {
                println!("{}", message);
                break;
            }
        }
    }
}

/// trims the string before conversion
fn parse_command(command: &str) -> Command {
    Command::from_str(command).unwrap()
}

fn execute_command(command: &Command, options: Vec<String>) -> Result<String, String> {
    match command {
        Command::ExitProgram => Err(String::from("Closing UOsniffer")),
        Command::CreateRanking => commands::create_ranking(options),
        Command::UnmatchedCommand(s) => Ok(format!(r#"Unknown command: "{}""#, s))
    }
}


enum Command {
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