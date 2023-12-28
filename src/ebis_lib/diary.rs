use std::{fmt::Debug, str::FromStr};

#[derive(Clone, Debug)]
pub struct Discipline {
    pub name: String,
    pub total_grade: String,
    pub lessons: Vec<Lesson>,
}

impl Discipline {
    /// Returns all grades of this [`Discipline`] as [`Vec<T>`]`
    pub fn to_grades<T>(&self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.lessons
            .iter()
            .flat_map(|l| l.to_grades::<T>())
            .collect()
    }

    /// Returns the estimate grade of this [`Discipline`].
    pub fn estimate_grade(&self) -> f32 {
        let f_grades: Vec<f32> = self.to_grades();

        let sum: f32 = f_grades.iter().sum();

        if sum > 0.0 {
            return sum / f_grades.len() as f32;
        }
        return 0.0;
    }
}

#[derive(Clone, Debug)]
pub struct Lesson {
    pub lesson_id: String,
    pub date: String,
    pub grades: Vec<String>,
}

impl Lesson {
    // Returns the grades vec as vec of specified type
    pub fn to_grades<T>(&self) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.grades
            .iter()
            .map(|g| g.parse::<T>().unwrap())
            .collect()
    }
}
