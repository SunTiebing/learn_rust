use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let mut s = String::from("hello");

    s.push_str(", word!");
    println!("{}", s);

    struct Student {
        name: String,
        age: i128,
        number: i128,
    }

    fn build_student(name: String, age: i128) -> Student {
        Student {
            name,
            age,
            number: 1,
        }
    }

    let s1 = build_student(String::from("name"), 1);

    let s2 = Student { number: 2, ..s1 };
}

struct Rectangle {
    width: u128,
    length: u128,
}

impl Rectangle {
    fn area(&self) -> u128 {
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.length
    }

    fn square(size: u128) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn read_file() {
    // 笨方法
    let file = File::open("hello.txt ");
    let f = match file {
        Ok(file) => file,
        Err(err) => panic!("Error opening file {:?}", err),
    };

    // 新方法
    let f = File::open("hello.txt").unwrap(); // 不可以自定义panic
    let f = File::open("hello.txt").expect("Error opening file"); //可以自定义panic

    // 笨方法
    let file = File::open("hello.txt");
    let f = match file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => {
                    panic!("Error creating file {:?}", e);
                }
            },
            other_kind => panic!("Error opening file {:?}", other_kind),
        },
    };

    // 新方法
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|err| {
                panic!("Error creating file {:?}", err);
            })
        } else {
            panic!("Error creating file {:?}", error);
        }
    });
}

fn open_file() -> Result<File, io::Error> {
    //传播panic 笨方法
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    //新方法
    let f = File::open("hello.txt")?;

    Ok(f)
}
