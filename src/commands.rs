use crate::ranking::format;
use crate::store;

pub(crate) fn create_ranking(options: &CreateRankingOptions) -> Result<String, String> {

    match store::get_dataset() {
        Some(s) => Ok(format(s, options)),
        None => Err(String::from("Dataset not found, load pdfs in the data folder to create a ranking")), //TODO not very  user friendly, better upload tool pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn hardcoded_create_ranking() {
        println!("{}", create_ranking(&CreateRankingOptions::default()).unwrap());
    }
}

pub enum Command {
    ExitProgram,
    CreateRanking(CreateRankingOptions),
    ShowAllCommands,
    UnmatchedCommand(String),
}

pub struct CreateRankingOptions {
    min_exams_taken: u32
}

impl CreateRankingOptions {
    pub fn default() -> Self {
       Self {
           min_exams_taken: 0
       }
    }


    pub fn with_min_exams_taken(min: u32) -> Self {
        Self {
            min_exams_taken: min
        }
    }

    pub fn min_exams_taken(&self) -> u32 {
        self.min_exams_taken
    }
}

impl Command {

}