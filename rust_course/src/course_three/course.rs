use crate::course_three::student::Student;

#[derive(Debug, Clone)]
pub struct Course {
    pub name: String,
    pub students: Vec<Student>,
}

impl Course {
    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn remove_student(&mut self, name: String) -> bool {
        if let Some(index) = self.students.iter().position(|s| s.name == name) {
            self.students.remove(index);
            return true;
        }
        false
    }

    pub fn new(name: String, students: Vec<Student>) -> Self {
        Self { name, students }
    }
}
