use crate::course_three::student::{Student, STUDENTS};

mod course_one;
mod course_three;
mod print_type_one;

#[allow(dead_code)]
fn main() {
    // homework one
    // println!("type one starts print");
    // print_type_one::print();

    // println!("type two starts print");
    // course_one::print_type_two::print();

    // homework three
    let bob = Student::new(String::from("Bob"), 10, None);
    let alice = Student::new(String::from("Alice"), 11, None);
    let mike = Student::new(String::from("Mike"), 9, None);

    unsafe { println!("all students: {:?}", STUDENTS) }
}
