use crate::ranking::format;
use crate::store;

pub(crate) fn create_ranking(options: Vec<String>) -> Result<String, String> {
    match store::get_dataset() {
        Some(s) => Ok(format(s)),
        None => Err(String::from("Dataset not found, load pdfs in the data folder to create a ranking")), //TODO not very  user friendly, better upload tool pending
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn hardcoded_create_ranking() {
        println!("{}", create_ranking().unwrap());
    }
}