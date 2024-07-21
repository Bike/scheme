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

fn fcons(ll: &ObjP, args: &ObjP) -> EvalResult {
    let (arg0, arg1) = args2(ll, args)?;
    Ok(cons(&arg0, &arg1))
}
fn fcar(ll: &ObjP, args: &ObjP) -> EvalResult {
    let arg0 = args1(ll, args)?;
    match arg0.borrow() {
        Object::Cons {car, ..} => { Ok(car.clone()) }
        _ => { Err(EvalError::NotCons(arg0)) }
    }
}
fn fcdr(ll: &ObjP, args: &ObjP) -> EvalResult {
    let arg0 = args1(ll, args)?;
    match arg0.borrow() {
        Object::Cons {car: _car, cdr} => { Ok(cdr.clone()) }
        _ => { Err(EvalError::NotCons(arg0)) }
    }
}

fn list1(arg0: &ObjP) -> ObjP { cons(arg0, &ObjP::new(Object::Null)) }
fn list2(arg0: &ObjP, arg1: &ObjP) -> ObjP {
    cons(arg0, &cons(arg1, &ObjP::new(Object::Null)))
}

pub fn ground() -> ObjP {
    let cons_n = intern("cons");
    let car_n = intern("car");
    let cdr_n = intern("cdr");
    let pairs = [(&cons_n, ObjP::new(Object::Subr(list2(&car_n, &cdr_n), fcons))),
                 (&car_n, ObjP::new(Object::Subr(list1(&cons_n), fcar))),
                 (&cdr_n, ObjP::new(Object::Subr(list1(&cons_n), fcdr)))];
    let mut env = ObjP::new(Object::Null);
    for (name, subr) in pairs {
        env = acons(name, &subr, &env)
    }
    env
}
