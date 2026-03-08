use crate::store;
use crate::store::StudentDataSet;

pub(crate) fn create_ranking() -> Result<String, String> {
    let marks = match store::get_dataset() {
        Some(s) => s,
        None => return Err(String::from("Dataset not found, load data to calculate a ranking")),
    };
    return Ok("lusa 1, bizu 2".to_string());
}