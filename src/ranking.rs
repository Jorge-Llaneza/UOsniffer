use crate::store::{ExamMarks, Mark, Student, StudentDataSet};

#[derive(Debug)]
struct StudentRanked {
    student: Student,
    average_mark: f64,
    exams_taken: i32,
}

pub(crate) fn format(dataset: StudentDataSet) -> String {
    let students = dataset.students();
    let marks = dataset.exam_marks();

    let mut ranking = Vec::<StudentRanked>::new();

    for student in students {
        ranking.push(create_stats(student, &marks));
    }

    // TODO implement advanced filtering
    ranking = ranking.into_iter()
        .filter(|student| student.exams_taken > 0)
        .collect();

    sort(&mut ranking);

    format_ranking(ranking)
}


fn format_ranking(ranking: Vec<StudentRanked>) -> String {
    let mut counter = 1;
    let mut ranking_string = String::with_capacity(ranking.len() * 30);

    for student in ranking {
        ranking_string.push_str(&format!("{}- ", counter));
        counter += 1;

        ranking_string = push_name(ranking_string, student.student);
        ranking_string = push_mark(ranking_string, student.average_mark);
        ranking_string = push_exams_taken(ranking_string, student.exams_taken);

        ranking_string.push('\n');
    }

    ranking_string
}

fn push_name(mut ranking: String, student: Student) -> String {
    for name in student.name {
        ranking.push_str(format!("{} ", name).as_str());
    }
    for surname in student.surname {
        ranking.push_str(format!("{} ", surname).as_str());
    }
    ranking
}

fn push_mark(mut ranking: String, mark: f64) -> String {
    ranking.push_str(format!("Average: {:.2} ", mark).as_str());
    ranking
}

fn push_exams_taken(mut ranking: String, exams_taken: i32) -> String {
    ranking.push_str(format!("Exams_taken: {}", exams_taken).as_str());
    ranking
}

fn create_stats(student: Student, marks: &Vec<ExamMarks>) -> StudentRanked {
    let mut exams_taken = 0;
    let mut numerical_marks = Vec::new();

    for exam in marks {
        for mark in &exam.marks {
            if mark.student == student {
                exams_taken += 1;
                numerical_marks.push(mark.normalized_score);
            }
        }
    }

    StudentRanked {
        student,
        average_mark: calculate_average(numerical_marks),
        exams_taken,
    }
}

/// sorts the ranking by average mark in-place
fn sort(ranking: &mut Vec<StudentRanked>) {
    ranking.sort_by(|a, b| b.average_mark.partial_cmp(&a.average_mark).unwrap())}

fn calculate_average(marks: Vec<f64>) -> f64 {
    if marks.is_empty() {
        return 0.0;
    }
    let sum: f64 = marks.iter().sum();

    sum / marks.len() as f64
}
