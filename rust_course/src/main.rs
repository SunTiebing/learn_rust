use crate::course_three::class::Class;
use crate::course_three::student::{Student, STUDENTS};

mod course_one;
mod course_three;
mod print_type_one;

#[allow(dead_code)]
fn main() {
    // homework one
    println!("-----------homework one starts -----------");
    println!("type one starts print");
    print_type_one::print();
    println!("type two starts print");
    course_one::print_type_two::print();
    println!("-----------homework one ends -----------");

    // homework three examples not include course and society
    println!("-----------homework two starts -----------");
    let bob = Student::new(String::from("Bob"), 10, None);
    let alice = Student::new(String::from("Alice"), 11, None);
    let mike = Student::new(String::from("Mike"), 9, None);
    unsafe { println!("all students: {:?}", STUDENTS) }
    let mut class = Class::new(1, vec![bob, alice, mike.clone()]);
    println!("class one : {:?}", class);
    class.remove_student(mike.num);
    println!("class one : {:?}", class);
    class.add_student(mike.num);
    println!("class one : {:?}", class);
    class.increase_grade();
    println!("class one : {:?}", class);
    Class::remove_class(class.num);
    println!("class one : {:?}", class);
    println!("-----------homework two ends -----------");
}
