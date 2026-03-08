use crate::lexer::Lexer;

mod constants;
mod lexer;
mod token;

fn main() {
    let input = String::from("=+(){},;");
    let mut lexer = Lexer::new(input);
    let token = lexer.next_token();
    println!("Token: {:?}", token);
}
