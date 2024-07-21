use std::rc::Rc;
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

// Rust doesn't wanna do dumb pointer equality - == on Rcs checks the
// underlying content.
// Fair, honestly, even if it makes it a bit weird for functions.
// Also means it'll probably explode if you try comparing circular structures.
#[derive(Debug)]
#[derive(Eq, PartialEq)]
pub enum Object {
    Cons { car: ObjP, cdr: ObjP },
    Null,
    Fixnum(i64),
    Symbol(String),
    Boolean(bool),
    Subr(ObjP, SubrFun),
    Fsubr(ObjP, FsubrFun),
    Expr { form: ObjP, lambda_list: ObjP, env: ObjP },
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
            Object::Subr(..) => { write!(f, "#<SUBR>") }
            Object::Fsubr(..) => { write!(f, "#<FSUBR>") }
            Object::Expr{..} => { write!(f, "#<EXPR>") }
        }
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
