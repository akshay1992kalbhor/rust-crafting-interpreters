fn test1(s: &str) {
    println!("S: {}", s);
}

fn fun1() {
    // borrowed types: &str, &[T], &T
    // borrowing an owned type: &String, &Vec<T>, &Box<T>
    // "123" = string slice = &str
    let s1 = String::from("123");
    let s3 = &s1 as &str;
    let s2 = &s1[..];
    test1(s2);
    let l1: Box<String> = Box::from(s1);
}

use rust_crafting_interpreters::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut scanner = Scanner::new();
    if args.len() > 1 {
        // TODO: Where do you handle errors?
        let file = std::fs::read_to_string(&args[1]).unwrap();
        scanner.run_file(&file);
        return;
    }
    scanner.run_prompt();
}
