use crate::lib::scanner::Scanner;

pub mod lib;

fn main() {
    let input = "title: Diagram
    one->two: YAY
    two->one: Yay2";

    let mut scanner = Scanner::new(input);
    let result = scanner.scan();

    println!("{:#?}", result);
}
