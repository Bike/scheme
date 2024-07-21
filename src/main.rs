use std::io;
use std::io::Write;

mod objects;
mod reader;
mod eval;

fn main() {
    let env = crate::objects::ObjP::new(crate::objects::Object::Null);
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match reader::read(&input) {
            Ok(o) => {
                match eval::eval(&o, &env) {
                    Ok(r) => { println!("{}", r); }
                    Err(e) => { println!("Eval error! {e:?}"); }
                }
            }
            Err(e) => { println!("Read error! {e:?}"); }
        }
    }
}
