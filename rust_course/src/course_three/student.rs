use crate::course_three::class::Class;

static mut STU_NUM: u32 = 0;
pub static mut STUDENTS: Vec<Student> = vec![];

#[derive(Debug, Clone)]
pub struct Student {
    pub num: u32,
    pub name: String,
    pub age: u32,
    pub class: Option<Class>,
}

impl Student {
    pub fn new(name: String, age: u32, class: Option<Class>) -> Self {
        let student = unsafe {
            Self {
                num: next_id(),
                name,
                age,
                class,
            }
        };
        unsafe {
            STUDENTS.push(student.clone());
        }
        student
    }

    pub fn change_name(&mut self, name: String) {
        self.name = name
    }

    pub fn change_age(&mut self, age: u32) {
        self.age = age
    }

    pub fn change_class(&mut self, class: Class) {
        self.class = Some(class)
    }

    pub fn get_student_by_id(id: u32) -> Student {
        unsafe {
            if let Some(stu) = STUDENTS.iter().find(|&s| s.num == id) {
                stu.clone()
            } else {
                panic!("number {} student is not exist", id)
            }
        }
    }

    pub fn dismiss_student(id: u32) {
        unsafe {
            match STUDENTS.iter().position(|stu| stu.num == id) {
                None => panic!("number {} student is not exist", id),
                Some(index) => {
                    STUDENTS.remove(index);
                }
            }
        }
    }
}

unsafe fn next_id() -> u32 {
    match STU_NUM.checked_add(1) {
        None => panic!("student number over flow"),
        Some(num) => {
            STU_NUM = num;
            num
        }
    }
}
