mod parser;

fn main() {
    let hello = "# Hello world!";
    let a = parser::parse(&hello);
    println!("{:?}", a)
}
