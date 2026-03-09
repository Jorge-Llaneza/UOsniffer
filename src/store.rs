use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub struct StudentDataSet {
     exam_marks: Vec<ExamMarks>,
    students: Vec<Student>,
}

impl StudentDataSet {
    /// Creates a deep clone of the dataset's students
    pub fn students(&self) -> Vec<Student> {
        self.students.clone()
    }

    /// Creates a deep clone of the dataset's exams
    pub fn exam_marks(&self) -> Vec<ExamMarks> {
        self.exam_marks.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ExamMarks {
    exam_name: String,
    pub(super) marks: Vec<Mark>,
}
#[derive(Debug, Clone)]
pub(super) struct Mark {
    /// score in range [0.0 , 1.0]
    pub(super) normalized_score: f64,
    pub(super) student: Student,
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Student {
    pub(super) name: Vec<String>,
    pub(super) surname: Vec<String>
}

pub(crate) fn get_dataset() -> Option<StudentDataSet> {
    let data_path = PathBuf::new()
        .join("data");

    let mut exam_marks = Vec::<ExamMarks>::new();

    for entry in std::fs::read_dir(data_path).unwrap() {
        if let Ok(entry) = entry {
            match read_pdf(&entry.path()) {
                Ok(markset) => exam_marks.push(markset),
                Err(_) => (),
            }
        }
    }

    let students = match collect_students(&exam_marks) {
        Some(s) => s,
        None => return None,
    };

    if exam_marks.len() > 0 {
        Some(StudentDataSet {
            exam_marks,
            students,
        })
    } else {
        None
    }
}

fn collect_students(exams: &Vec<ExamMarks>) -> Option<Vec<Student>> {
    let mut seen_students = Vec::<Student>::new();

    if exams.is_empty() {
        return None;
    }

    for exam in exams {
        for mark in &exam.marks {
            if seen_students.contains(&mark.student) {
                continue;
            } else {
                seen_students.push(mark.student.clone());
            }
        }
    }

    if seen_students.is_empty() {
        return None;
    }

    Some(seen_students)
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
        Some(s) => s,
        None => return None,
    };

    let mark = match find_mark(pdf_text_row) {
        Some(f) => f,
        None => return None
    };

    Some(
        Mark {
            normalized_score: mark,
            student
        }
    )
}

fn find_mark(pdf_row: &str) -> Option<f64> {
    for word in pdf_row.split_whitespace() {
        if let Ok(mark) = f64::from_str(&word.replace(",", ".")) {
            return Some(mark)
        }
    }
    None
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
    use approx::assert_relative_eq;

    #[test]
    fn extract_valid_mark_test() {

    }

    #[test]
    fn extract_pdf_marks() {
        let pdf = pdf_extract::extract_text(Path::new("test/assets/notes.pdf")).unwrap();

        match extract_marks(&pdf) {
            Ok(m) => {
                assert_eq!(m.len(), 5);
                assert_relative_eq!(m[0].normalized_score, 2.6);
                assert_relative_eq!(m[1].normalized_score, 7.9);
                assert_relative_eq!(m[2].normalized_score, 9.0);

                assert_eq!(m[3].student.name[0], "nícolo");
                assert_eq!(m[2].student.surname[0], "ertenestez");
                assert_eq!(m[2].student.surname[1], "ponta");
                assert_eq!(m[2].student.name[0], "carlos");
                assert_eq!(m[2].student.name[1], "papero");
            }
            Err(()) => panic!()
        }
    }
}