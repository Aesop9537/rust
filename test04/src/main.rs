// 字符串切片
fn main() {
    let s = String::from("hello world");
    // 第一位索引开头可省略0
    let hello = &s[..5];
    // 最后一位索引结尾 可省略s.len
    let world = &s[6..];
    println!("hello:{},world:{}",hello,world);

}

