use std::io;
use std::io::Write;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexp.pest"]
pub struct SexpParser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush");
        let mut read = String::new();
        io::stdin()
            .read_line(&mut read)
            .expect("Failed to read line");
        let parse = SexpParser::parse(Rule::sexp, read.as_str());
        println!("{:?}", parse);
    }
}
