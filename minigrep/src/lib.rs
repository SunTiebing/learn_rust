use std::collections::HashMap;
use std::error::Error;
use std::time::Duration;
use std::{fs, thread};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    for ele in search(&config.query, &contents) {
        print!("{}", ele);
    }
    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => {
                return Err("Didn't get a query string");
            }
        };
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => {
                return Err("Didn't get a file_name string");
            }
        };
        Ok(Config { query, file_name })
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();

    // for line in content.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }

    // res

    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "a";
        let content = "a";
        assert_eq!(vec![content], search(query, content))
    }
}

// 闭包部分
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut new_expensive_closure = Cacher::new(|num| {
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity == 0 {
        print!("{}", expensive_closure(intensity));
        print!("{}", expensive_closure(intensity));

        print!("{}", new_expensive_closure.value(intensity));
        print!("{}", new_expensive_closure.value(intensity));
    } else {
        if random_number == 0 {
            print!("{}", expensive_closure(intensity));

            print!("{}", new_expensive_closure.value(intensity));
        } else {
            print!("{}", expensive_closure(intensity));

            print!("{}", new_expensive_closure.value(intensity));
        }
    }
}

// 缓存器
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, args: u32) -> u32 {
        match self.value.get(&args) {
            Some(args) => *args,
            None => {
                let r = (self.calculation)(args);
                self.value.insert(r, r);
                r
            }
        }
    }
}

// move 关键字
fn move_test() {
    let vec = vec![0, 1, 2, 3, 4, 5];

    // vec多有权已经移动
    let test = move |num| num == vec;

    // print!("不能在用vec了 {}", vec);
    print!("test {}", test(vec![1]));
}

// 迭代器
#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn iterator_map() {
        // map 方法
        let mut v1 = vec![1, 2, 3];
        let mut v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);

        // into_iter filter 方法
        let v3 = vec![Config {
            query: String::from("foo"),
            file_name: String::from("foo.txt"),
        }];
        let v4: Vec<Config> = v3.into_iter().filter(|x| x.query == "foo").collect();
        assert_eq!(
            v4,
            vec![Config {
                query: String::from("foo"),
                file_name: String::from("foo.txt"),
            }]
        );

        // zip 方法
        let v5: i32 = v1
            .iter() // [1, 2, 3]
            .zip(v2.iter().skip(1)) // [(1, 3), (2, 4), (3,)]
            .map(|(a, b)| a * b) // [3, 8,]
            .filter(|x| x % 3 == 0) // [3]
            .sum(); // 3
    }
}
