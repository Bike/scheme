use std::rc::Rc;
use std::borrow::Borrow;
use std::fmt;

#[derive(Debug)]
pub enum Object {
    Cons { car: ObjP, cdr: ObjP },
    Null,
    Fixnum(i64),
    Symbol(String),
    Boolean(bool),
}

pub type ObjP = Rc<Object>;

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

pub fn cons(car: ObjP, cdr: ObjP) -> ObjP {
    Rc::new(Object::Cons { car: car, cdr: cdr })
}
