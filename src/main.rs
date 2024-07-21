use std::io;
use std::io::Write;
use std::fmt;
use std::borrow::Borrow;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sexp.pest"]
pub struct SexpParser;

// For debugging
/*
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
*/

#[derive(Debug)]
enum Object {
    Cons { car: Box<Object>, cdr: Box<Object> },
    Null,
    Fixnum(i64),
    Symbol(String),
    Boolean(bool),
}

#[derive(Debug)]
enum ReadError {
    // These are only used to dump to debug print,
    // so rustc whines about them being unused.
    #[allow(unused)] Parse(pest::error::Error<Rule>),
    #[allow(unused)] Fixnum(std::num::ParseIntError),
}

impl std::convert::From<pest::error::Error<Rule>> for ReadError {
    fn from(err: pest::error::Error<Rule>) -> Self {
        ReadError::Parse(err)
    }
}
impl std::convert::From<std::num::ParseIntError> for ReadError {
    fn from(err: std::num::ParseIntError) -> Self {
        ReadError::Fixnum(err)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Null => { write!(f, "()") }
            Object::Fixnum(i) => { write!(f, "{}", i) }
            Object::Symbol(s) => { write!(f, "{}", s) }
            Object::Boolean(t) => {
                if *t { write!(f, "#t") } else { write!(f, "#f") }
            }
            Object::Cons {car, cdr} => {
                match cdr.borrow() {
                    Object::Null => { write!(f, "({})", *car) }
                    other => { write!(f, "({} . {})", *car, other) }
                }
            }
        }
    }
}

fn cons(car: Box<Object>, cdr: Box<Object>) -> Object {
    Object::Cons { car: car, cdr: cdr }
}

fn read(input: &str) -> Result<Object, ReadError> {
    let parse = SexpParser::parse(Rule::sexp, input);
    Ok(read_inner(parse?.next().unwrap())?)
}

fn read_list(mut pairs: pest::iterators::Pairs<Rule>) -> Result<Object, ReadError> {
    match pairs.next() {
        None => { Ok(Object::Null) },
        Some(p) => {
            Ok(cons(Box::new(read_inner(p)?), Box::new(read_list(pairs)?)))
        }
    }
}

fn read_dotted_list(first: Object, mut pairs: pest::iterators::Pairs<Rule>)
                    -> Result<Object, ReadError> {
    match pairs.next() {
        None => { Ok(first) }
        Some(p) => {
            match p.as_rule() {
                // Skip over the dot. Since we've already iterated past it
                // this should always halt.
                Rule::consing_dot => { read_dotted_list(first, pairs) }
                _other => {
                    Ok(cons(Box::new(first),
                            Box::new(read_dotted_list(read_inner(p)?, pairs)?)))
                }
            }
        }
    }
}

fn read_inner(parse: pest::iterators::Pair<Rule>) -> Result<Object, ReadError> {
    match parse.as_rule() {
        Rule::symbol => { Ok(Object::Symbol(String::from(parse.as_str()))) }
        Rule::integer => { Ok(Object::Fixnum(parse.as_str().parse()?)) }
        Rule::boolean => { Ok(Object::Boolean(parse.as_str() == "#t")) }
        Rule::proper_list => { read_list(parse.into_inner()) }
        Rule::dotted_list => {
            let mut pairs = parse.into_inner();
            match pairs.next() {
                None => { panic!("Dotted list with no elements"); }
                Some(p) => { Ok(read_dotted_list(read_inner(p)?, pairs)?) }
            }
        }
        _unknown_term => { panic!("Can't read this thing"); }
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match read(&input) {
            Ok(o) => { println!("{o}"); }
            Err(e) => { println!("Read error! {e:?}"); }
        }
        /*
        let parse = SexpParser::parse(Rule::sexp, &read);
        match parse {
            Ok(mut p) => {
                print_parse(p.next().unwrap());
                println!("");
            }
            Err(e) => { println!("Failed to parse: {:?}", e); }
        };
         */
    }
}
