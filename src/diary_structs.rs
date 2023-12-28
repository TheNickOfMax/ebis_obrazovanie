#[derive(Clone, Debug)]
pub struct Discipline {
    pub name: String,
    pub total_grade: String,
    pub lessons: Vec<Lesson>,
}

#[derive(Clone, Debug)]
pub struct Lesson {
    pub lesson_id: String,
    pub date: String,
    pub grades: Vec<Vec<String>>,
}
