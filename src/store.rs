use std::path::{Path, PathBuf};

pub struct StudentDataSet {
    exam_marks: Vec<ExamMarks>,
    students: Vec<Student>,
}
struct ExamMarks {
    exam_name: String,
    marks: Vec<Mark>,
}
struct Mark {
    /// score in range [0.0 , 1.0]
    normalized_score: f64,
    student: Student,
}
struct Student {
    name: Vec<String>,
    surname: Vec<String>
}

pub(crate) fn get_dataset() -> Option<StudentDataSet> {
    todo!()
}

fn read_pdf(path: &Path) -> Result<ExamMarks, ReadPdfError> {
    let pdf_text = match pdf_extract::extract_text(path) {
        Ok(s) => s,
        Err(_) => return Err(ReadPdfError::FileNotFound)
    };

    let exam_name = path.file_name().unwrap().to_str().unwrap().to_string();

    let marks = match extract_marks(&pdf_text) {
        Ok(m) => m,
        Err(()) => return Err(ReadPdfError::NoNotesInFile),
    };

    Ok(ExamMarks {
        exam_name,
        marks
    })
}

fn extract_marks(text: &str) -> Result<Vec<Mark>, ()> {
    let mut marks: Vec<Mark> = Vec::new();

    for row in text.lines() {
        if let Some(mark) = extract_mark(&row) {
            marks.push(mark);
        }
    }

    if marks.is_empty() {
        Err(())
    } else {
        Ok(marks)
    }
}
fn extract_mark(pdf_text_row: &str) -> Option<Mark> {
    let student = match find_student(pdf_text_row) {
        Ok(s) => s,
        None => return None,
    };

    let mark = find_mark(pdf_text_row) {
        Ok(f) => f,
        None => return None
    };
}

fn find_mark(pdf_row: &str) -> Option<Mark> {
    for word in pdf_row.split_whitespace() {

    }
}

fn find_student(pdf_row: &str) -> Option<Student> {
    let split = match pdf_row.split_once(","){
        Some(s) => s,
        None => return None
    };
    let surname = split.0.split_whitespace()
        .map(|s| s.trim().to_lowercase().to_string())
        .collect::<Vec<String>>();

    let mut name = Vec::new();

    if split.1.is_empty() {
        return None;
    }
    for possible_name in split.1.split_whitespace() {
        if possible_name.chars().all(char::is_alphanumeric) {
            name.push(possible_name.trim().to_lowercase().to_string());
        } else {
            break;
        }
    }

    if name.is_empty() {
        return None;
    }
    if surname.is_empty() {
        return None;
    }

    Some(Student {
        name,
        surname
    })
}

enum ReadPdfError {
    FileNotFound,
    NoNotesInFile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_valid_mark_test() {

    }

    #[test]
    fn extract_pdf_marks() {
        let pdf = pdf_extract::extract_text(Path::new("test/assets/notes.pdf")).unwrap();

        for row in pdf.lines() {
            extract_mark(&row);
        }
    }
}