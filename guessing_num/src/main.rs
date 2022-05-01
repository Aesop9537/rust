use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rand_num = rand::thread_rng().gen_range(1..101);
    println!("神秘数字：{}", rand_num);
    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取");
        // 非数字重新输入
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个数字");
                continue;
            },
        };
        // 比较
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too greater!"),
            Ordering::Equal => {
                println!("猜对了！！！");
                break;
            },
        }
    }
}
