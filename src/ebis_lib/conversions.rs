use crate::ebis_lib::diary::{Discipline, Lesson};

use json::JsonValue;

pub fn api_json_to_ebis_structs(api_json: JsonValue) -> Vec<Discipline> {
    // leave only the disciplines
    let json_disciplines: Vec<&JsonValue> = api_json["periodGradesTable"]["disciplines"]
        .members()
        .collect();

    let disciplines: Vec<Discipline> = json_disciplines
        .iter()
        .map(|dis| {
            // the first array of "grades" that really should be called lessons
            let lessons_json = dis["grades"].clone();
            let lessons: Vec<Lesson> = json_array_to_lesson_vec(&lessons_json);

            Discipline {
                name: dis["name"].as_str().unwrap_or_default().to_string(),
                total_grade: dis["totalGrade"].as_str().unwrap_or_default().to_string(),
                lessons,
            }
        })
        .collect();

    disciplines
}

fn json_array_to_lesson_vec(json_array: &JsonValue) -> Vec<Lesson> {
    json_array
        .clone()
        .members()
        .map(|lesson| json_value_to_lesson(lesson))
        .collect()
}

fn json_value_to_lesson(lesson: &JsonValue) -> Lesson {
    Lesson {
        lesson_id: lesson["lessonId"].as_str().unwrap_or_default().to_string(),
        date: lesson["date"].as_str().unwrap_or_default().to_string(),
        grades: json_array_to_grade_vec(&lesson["grades"].clone())
            .into_iter()
            .flatten()
            .collect(),
    }
}

fn json_array_to_grade_vec(json_array: &JsonValue) -> Vec<Vec<String>> {
    json_array
        .clone()
        .members()
        .map(|grade| json_value_to_grade(grade))
        .collect()
}

fn json_value_to_grade(grade: &JsonValue) -> Vec<String> {
    grade
        .clone()
        .members()
        .map(|g| g.as_str().unwrap_or_default().to_string())
        .collect()
}
