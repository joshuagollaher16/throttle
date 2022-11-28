mod lexer;

pub use lexer::*;

fn main() {
    let contents = std::fs::read_to_string("testprograms/main.throttle").expect("Cant open test program");

    println!("{:?}", lex(contents));
}
