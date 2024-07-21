use pest::Parser;
use pest_derive::Parser;

use crate::objects::{Object, ObjP, cons};

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
pub enum ReadError {
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

pub fn read(input: &str) -> Result<ObjP, ReadError> {
    let parse = SexpParser::parse(Rule::sexp, input);
    Ok(read_inner(parse?.next().unwrap())?)
}

fn read_list(mut pairs: pest::iterators::Pairs<Rule>) -> Result<ObjP, ReadError> {
    match pairs.next() {
        None => { Ok(ObjP::new(Object::Null)) },
        Some(p) => {
            Ok(cons(&read_inner(p)?, &read_list(pairs)?))
        }
    }
}

fn read_dotted_list(first: ObjP, mut pairs: pest::iterators::Pairs<Rule>)
                    -> Result<ObjP, ReadError> {
    match pairs.next() {
        None => { Ok(first) }
        Some(p) => {
            match p.as_rule() {
                // Skip over the dot. Since we've already iterated past it
                // this should always halt.
                Rule::consing_dot => { read_dotted_list(first, pairs) }
                _other => {
                    Ok(cons(&first,
                            &read_dotted_list(read_inner(p)?, pairs)?))
                }
            }
        }
    }
}

fn read_inner(parse: pest::iterators::Pair<Rule>) -> Result<ObjP, ReadError> {
    match parse.as_rule() {
        Rule::symbol => {
            Ok(ObjP::new(Object::Symbol(String::from(parse.as_str()))))
        }
        Rule::integer => { Ok(ObjP::new(Object::Fixnum(parse.as_str().parse()?))) }
        Rule::boolean => { Ok(ObjP::new(Object::Boolean(parse.as_str() == "#t"))) }
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
