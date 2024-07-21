use std::io;
use std::io::Write;

mod reader;
mod objects;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().expect("Couldn't flush");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match reader::read(&input) {
            Ok(o) => { println!("{}", *o); }
            Err(e) => { println!("Read error! {e:?}"); }
        }
    }
}
