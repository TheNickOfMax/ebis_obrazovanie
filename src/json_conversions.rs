use json::JsonValue;

use crate::{
    diary_structs::{Discipline, Lesson},
    from_json::FromJson,
};

pub fn api_json_to_usable_vec(api_json: JsonValue) -> Vec<Discipline> {
    // leave only the disciplines
    let json_disciplines: Vec<JsonValue> =
        Vec::from_json_array(api_json["periodGradesTable"]["disciplines"].clone());

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

pub fn json_array_to_lesson_vec(json_array: &JsonValue) -> Vec<Lesson> {
    Vec::<JsonValue>::from_json_array(json_array.clone())
        .iter()
        .map(|lesson| json_value_to_lesson(lesson))
        .collect()
}

pub fn json_value_to_lesson(lesson: &JsonValue) -> Lesson {
    Lesson {
        lesson_id: lesson["lessonId"].as_str().unwrap_or_default().to_string(),
        date: lesson["date"].as_str().unwrap_or_default().to_string(),
        grades: json_array_to_grade_vec(&lesson["grades"].clone()),
    }
}

pub fn json_array_to_grade_vec(json_array: &JsonValue) -> Vec<Vec<String>> {
    Vec::<JsonValue>::from_json_array(json_array.clone())
        .iter()
        .map(|grade| json_value_to_grade(grade))
        .collect()
}

pub fn json_value_to_grade(grade: &JsonValue) -> Vec<String> {
    Vec::<JsonValue>::from_json_array(grade.clone())
        .iter()
        .map(|g| g.as_str().unwrap_or_default().to_string())
        .collect()
}
