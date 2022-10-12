extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("猜数！!");
    println!("猜测一个数");
    let sjs = rand::thread_rng().gen_range(1,101);
    println!("{}",sjs);
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("无法读取行");
    println!("你猜测的数是：{}",guess);
    let guess:u32=guess.trim().parse().expect("type a number");
    match guess.cmp(&sjs) {
        Ordering::Less=>println!("太小了"),
        Ordering::Greater=>println!("太大了"),
        Ordering::Equal=>println!("你赢了"),
    }
}
