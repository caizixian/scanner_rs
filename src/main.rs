extern crate scanner;

use scanner::Scanner;

fn main() {
    let mut stdin = Scanner::new();
    let a: i32 = stdin.next();
    let b: i32 = stdin.next();
    println!("{}", a + b);
}