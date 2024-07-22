use gc::{Gc, Trace, Finalize};
use std::borrow::Borrow;
use std::fmt;

#[derive(Debug)]
pub enum EvalError {
    #[allow(unused)] Unbound(ObjP),
    #[allow(unused)] ImproperList(ObjP),
    #[allow(unused)] TooManyArgs(ObjP, ObjP),
    #[allow(unused)] NotEnoughArgs(ObjP, ObjP),
    #[allow(unused)] DottedArgs(ObjP, ObjP),
    #[allow(unused)] NotCombiner(ObjP),
    #[allow(unused)] NotCons(ObjP),
    #[allow(unused)] NotBoolean(ObjP),
}
pub type EvalResult = Result<ObjP, EvalError>;

// lambda list, args as a list
type SubrFun = fn(&ObjP, &ObjP) -> EvalResult;
// "lambda list", unevaluated arguments, environment
type FsubrFun = fn(&ObjP, &ObjP, &ObjP) -> EvalResult;

// Rust doesn't wanna do dumb pointer equality - == on Gcs checks the
// underlying content.
// Fair, honestly, even if it makes it a bit weird for functions.
// Also means it'll probably explode if you try comparing circular structures.
#[derive(Debug)]
#[derive(Eq, PartialEq, Trace, Finalize)]
pub enum Object {
    Cons { car: ObjP, cdr: ObjP },
    Null,
    Fixnum(i64),
    Symbol(String),
    Boolean(bool),
    // GC doesn't know it can ignore function pointers, for some reason.
    Subr(ObjP, #[unsafe_ignore_trace] SubrFun),
    Fsubr(ObjP, #[unsafe_ignore_trace] FsubrFun),
    Expr { form: ObjP, lambda_list: ObjP, env: ObjP },
}

#[derive(Debug, Eq, PartialEq, Clone, Trace, Finalize)]
pub struct ObjP {
    object: Gc<Object>,
}

impl ObjP {
    pub fn new(o : Object) -> Self {
        Self {
            object: Gc::new(o)
        }
    }
}

impl Borrow<Object> for ObjP {
    fn borrow(&self) -> &Object {
        self.object.borrow()
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Null => { write!(f, "()") }
            Object::Fixnum(ref i) => { write!(f, "{}", i) }
            Object::Symbol(ref s) => { write!(f, "{}", s) }
            Object::Boolean(ref t) => {
                if *t { write!(f, "#t") } else { write!(f, "#f") }
            }
            Object::Cons {ref car, ref cdr} => {
                write!(f, "({}", car)?;
                let mut tail = cdr;
                loop {
                    match *tail.borrow() {
                        Object::Null => { break write!(f, ")"); }
                        Object::Cons {ref car, ref cdr} => {
                            write!(f, " {}", car)?;
                            tail = cdr;
                        }
                        _ => { break write!(f, " . {})", tail); }
                    }
                }
            }
            Object::Subr(..) => { write!(f, "#<SUBR>") }
            Object::Fsubr(..) => { write!(f, "#<FSUBR>") }
            Object::Expr{..} => { write!(f, "#<EXPR>") }
        }
    }
}
impl fmt::Display for ObjP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let o : &Object = self.borrow(); // type annot req. to disambiguate borrow
        o.fmt(f)
    }
}

// no actual package system right now, but == does string compare anyway
pub fn intern(name: &str) -> ObjP {
    ObjP::new(Object::Symbol(String::from(name)))
}

pub fn cons(car: &ObjP, cdr: &ObjP) -> ObjP {
    ObjP::new(Object::Cons { car: car.clone(), cdr: cdr.clone() })
}

pub fn acons(key: &ObjP, val: &ObjP, alist: &ObjP) -> ObjP {
    cons(&cons(key, val), alist)
}

pub fn nil() -> ObjP {
    ObjP::new(Object::Null)
}

pub fn make_symbol(name: &str) -> ObjP {
    ObjP::new(Object::Symbol(String::from(name)))
}

pub fn make_fixnum(i: i64) -> ObjP {
    ObjP::new(Object::Fixnum(i))
}

pub fn make_bool(b: bool) -> ObjP {
    ObjP::new(Object::Boolean(b))
}
