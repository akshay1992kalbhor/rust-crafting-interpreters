use rust_crafting_interpreters::scanner::Scanner;

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
