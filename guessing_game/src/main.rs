use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数");

    let secret_num = rand::thread_rng().gen_range(1..100);

    println!("神秘数字：{}", secret_num);

    loop {
        println!("猜测一个数");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Result::Ok(num) => num,
            Result::Err(_) => {
                println!("请输入数字");
                continue;
            }
        };

        println!("你猜的数是：{}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("猜对了");
                break;
            }
            Ordering::Greater => println!("太大了"),
        }
    }
}
