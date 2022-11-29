mod lexer;

pub use lexer::*;

mod one_pass;

pub use one_pass::*;


fn main() {
    let contents = std::fs::read_to_string("testprograms/main.throttle").expect("Cant open test program");
    let tokens = lex(contents);
    println!("{:?}", tokens);
}
