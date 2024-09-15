mod parser;
mod convert;

use std::fs;
use std::io;

fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

fn read_file_to_string(path: &str) -> io::Result<String> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn main()  -> io::Result<()> {
    let contents = read_file_to_string("./test.md")?;
    let contents: &str = &contents;
    let mds = parser::parser::parse(&contents);
    let output = convert::convert::mds_to_html(&mds);
    let output: &str = &output;
    write_to_file("./test.html", output)?;
    Ok(())
}
