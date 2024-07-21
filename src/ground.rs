use crate::objects::{EvalResult, EvalError, Object, ObjP, intern, cons, acons};
use std::borrow::Borrow;

fn args0(ll: &ObjP, args: &ObjP) -> Result<(), EvalError> {
    match args.borrow() {
        Object::Null => { Ok(()) }
        Object::Cons{..} => { Err(EvalError::TooManyArgs(ll.clone(),
                                                         args.clone())) }
        _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
    }
}
fn args1(ll: &ObjP, args: &ObjP) -> Result<ObjP, EvalError> {
    match args.borrow() {
        Object::Null => { Err(EvalError::NotEnoughArgs(ll.clone(), args.clone())) }
        Object::Cons {car, cdr} => {
            match cdr.borrow() {
                Object::Null => { Ok(car.clone()) }
                Object::Cons {..} => {
                    Err(EvalError::TooManyArgs(ll.clone(), args.clone()))
                }
                _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
            }
        }
        _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
    }
}
fn args2(ll: &ObjP, args: &ObjP) -> Result<(ObjP, ObjP), EvalError> {
    match args.borrow() {
        Object::Null => { Err(EvalError::NotEnoughArgs(ll.clone(), args.clone())) }
        Object::Cons {car, cdr} => {
            match cdr.borrow() {
                Object::Null => {
                    Err(EvalError::NotEnoughArgs(ll.clone(), args.clone()))
                }
                Object::Cons {car: cadr, cdr: cddr} => {
                    match cddr.borrow() {
                        Object::Null => { Ok((car.clone(), cadr.clone())) }
                        Object::Cons{..} => {
                            Err(EvalError::TooManyArgs(ll.clone(), args.clone()))
                        }
                        _ => {
                            Err(EvalError::DottedArgs(ll.clone(), args.clone()))
                        }
                    }
                }
                _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
            }
        }
        _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
    }
}

fn fcons(args: &ObjP) -> EvalResult {
    let (arg0, arg1) = args2(&cons(&intern("CAR"), &cons(&intern("CDR"), &ObjP::new(Object::Null))), args)?;
    Ok(cons(&arg0, &arg1))
}
fn fcar(args: &ObjP) -> EvalResult {
    let arg0 = args1(&cons(&intern("CONS"), &ObjP::new(Object::Null)), args)?;
    match arg0.borrow() {
        Object::Cons {car, ..} => { Ok(car.clone()) }
        _ => { Err(EvalError::NotCons(arg0)) }
    }
}
fn fcdr(args: &ObjP) -> EvalResult {
    let arg0 = args1(&cons(&intern("CONS"), &ObjP::new(Object::Null)), args)?;
    match arg0.borrow() {
        Object::Cons {car: _car, cdr} => { Ok(cdr.clone()) }
        _ => { Err(EvalError::NotCons(arg0)) }
    }
}

pub fn ground() -> ObjP {
    let pairs = [(intern("cons"), ObjP::new(Object::Subr(fcons))),
                 (intern("car"), ObjP::new(Object::Subr(fcar))),
                 (intern("cdr"), ObjP::new(Object::Subr(fcdr)))];
    let mut env = ObjP::new(Object::Null);
    for (name, subr) in pairs {
        env = acons(&name, &subr, &env)
    }
    env
}
