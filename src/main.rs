mod parser;
mod emit;

fn main() {
    let hello = "# Hello world!";
    let a = parser::parser::parse(&hello);
    println!("{:?}", a)
}
