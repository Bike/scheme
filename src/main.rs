use std::io;
use std::io::Write;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexp.pest"]
pub struct SexpParser;

fn print_parse(parse: pest::iterators::Pair<Rule>) {
    match parse.as_rule() {
        Rule::symbol => { print!("{}", parse.as_str()); }
        Rule::integer => { print!("{}", parse.as_str()); }
        Rule::boolean => { print!("{}", parse.as_str()); }
        Rule::proper_list => {
            print!("(");
            let mut inner = parse.into_inner();
            match inner.next() {
                None => {}
                Some(p) => {
                    print_parse(p);
                    for i in inner {
                        print!(" ");
                        print_parse(i);
                    }
                }
            };
            print!(")");
        }
        Rule::dotted_list => {
            print!("(");
            let mut inner = parse.into_inner();
            print_parse(inner.next().unwrap());
            for i in inner {
                print!(" ");
                print_parse(i);
            }
            print!(")");
        }
        Rule::consing_dot => { print!("."); } // KLUDGE
        _unknown_term => { panic!("WTF"); }
    };
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush");
        let mut read = String::new();
        io::stdin()
            .read_line(&mut read)
            .expect("Failed to read line");
        let parse = SexpParser::parse(Rule::sexp, &read);
        match parse {
            Ok(mut p) => {
                print_parse(p.next().unwrap());
                println!("");
            }
            Err(e) => { println!("Failed to parse: {:?}", e); }
        };
    }
}
