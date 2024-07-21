use crate::objects::{EvalResult, EvalError, Object, ObjP, intern, cons, acons};
use crate::eval::eval;
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
            args0(ll, cdr)?;
            Ok(car.clone())
        }
        _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
    }
}
fn args2(ll: &ObjP, args: &ObjP) -> Result<(ObjP, ObjP), EvalError> {
    match args.borrow() {
        Object::Null => { Err(EvalError::NotEnoughArgs(ll.clone(), args.clone())) }
        Object::Cons {car, cdr} => {
            let arg1 = args1(ll, cdr)?;
            Ok((car.clone(), arg1.clone()))
        }
        _ => { Err(EvalError::DottedArgs(ll.clone(), args.clone())) }
    }
}
fn args3(ll: &ObjP, args: &ObjP) -> Result<(ObjP, ObjP, ObjP), EvalError> {
    match args.borrow() {
        Object::Null => { Err(EvalError::NotEnoughArgs(ll.clone(), args.clone())) }
        Object::Cons {car, cdr} => {
            let (arg1, arg2) = args2(ll, cdr)?;
            Ok((car.clone(), arg1.clone(), arg2.clone()))
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

fn feqv(ll: &ObjP, args: &ObjP) -> EvalResult {
    let (arg0, arg1) = args2(ll, args)?;
    Ok(ObjP::new(Object::Boolean(arg0 == arg1)))
}

fn fif(ll: &ObjP, args: &ObjP, env: &ObjP) -> EvalResult {
    let (cond, thn, els) = args3(ll, args)?;
    let econd = eval(&cond, env)?;
    match econd.borrow() {
        Object::Boolean(t) => {
            eval(if *t { &thn } else { &els }, env)
        }
        _ => { Err(EvalError::NotBoolean(econd.clone())) }
    }
}

fn list1(arg0: &ObjP) -> ObjP { cons(arg0, &ObjP::new(Object::Null)) }
fn list2(arg0: &ObjP, arg1: &ObjP) -> ObjP {
    cons(arg0, &cons(arg1, &ObjP::new(Object::Null)))
}
fn list3(arg0: &ObjP, arg1: &ObjP, arg2: &ObjP) -> ObjP {
    cons(arg0, &cons(arg1, &cons(arg2, &ObjP::new(Object::Null))))
}

pub fn ground() -> ObjP {
    let mut env = ObjP::new(Object::Null);
    let cons_n = intern("cons");
    let car_n = intern("car");
    let cdr_n = intern("cdr");
    let eqv_n = intern("eq?");
    let o1_n = intern("o1");
    let o2_n = intern("o2");
    let pairs = [(&cons_n, ObjP::new(Object::Subr(list2(&car_n, &cdr_n), fcons))),
                 (&car_n, ObjP::new(Object::Subr(list1(&cons_n), fcar))),
                 (&cdr_n, ObjP::new(Object::Subr(list1(&cons_n), fcdr))),
                 (&eqv_n, ObjP::new(Object::Subr(list2(&o1_n, &o2_n), feqv)))];
    for (name, subr) in pairs {
        env = acons(name, &subr, &env)
    }
    let if_n = intern("if");
    let condition_n = intern("condition");
    let then_n = intern("then");
    let else_n = intern("else");
    let fpairs = [(&if_n, ObjP::new(Object::Fsubr(list3(&condition_n, &then_n, &else_n),
                                                  fif)))];
    for (name, fsubr) in fpairs {
        env = acons(name, &fsubr, &env)
    }
    env
}
