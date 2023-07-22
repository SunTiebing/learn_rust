use crate::course_three::student::Student;

static mut CLASS_NUM: u32 = 0;
pub static mut CLASSES: Vec<Class> = vec![];

#[derive(Debug, Clone)]
pub struct Class {
    pub grade: u32,
    pub num: u32,
    pub students: Vec<Student>,
}

impl Class {
    pub fn new(grade: u32, students: Vec<Student>) -> Self {
        let class = unsafe {
            Self {
                grade,
                num: next_id(),
                students,
            }
        };
        unsafe {
            CLASSES.push(class.clone());
        }
        class
    }

    pub fn add_student(&mut self, stu_id: u32) {
        let student = Student::get_student_by_id(stu_id);
        self.students.push(student)
    }

    pub fn remove_student(&mut self, stu_id: u32) {
        match self.students.iter().position(|stu| stu.num == stu_id) {
            None => panic!("number {} student is not exist", stu_id),
            Some(index) => {
                self.students.remove(index);
            }
        }
    }

    pub fn increase_grade(&mut self) {
        self.grade = match self.grade.checked_add(1) {
            None => panic!("class number over flow"),
            Some(grade) => {
                if grade > 6 {
                    panic!("grade could not be bigger than six")
                } else {
                    grade
                }
            }
        }
    }

    pub fn remove_class(class_id: u32) {
        unsafe {
            match CLASSES.iter().position(|class| class.num == class_id) {
                None => panic!("number {} class is not exist", class_id),
                Some(index) => {
                    CLASSES.remove(index);
                }
            }
        }
    }
}

unsafe fn next_id() -> u32 {
    match CLASS_NUM.checked_add(1) {
        None => panic!("student number over flow"),
        Some(num) => {
            CLASS_NUM = num;
            num
        }
    }
}
