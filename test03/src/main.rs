// 所有权与函数
fn main() {
    let s = String::from("hello world");
    // s所有权移动到函数中，s此时没有东西输出
    let s2 = return_str(&s);
    println!("s:{},s2:{}", s, s2);
    // s2为s的引用
    println!("{}", &s == s2);

    let i = 10;
    // i 复制一份传入函数，i 仍可输出
    let i2 = return_int(i);
    println!("i:{},i2:{}", i, i2);
}

fn return_str(s: &String) -> &String {
    s
}

fn return_int(i: isize) -> isize {
    i
}