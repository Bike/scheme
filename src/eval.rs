use crate::objects::{EvalResult, EvalError, Object, ObjP, cons, acons};
use std::borrow::Borrow;

pub fn eval(form: &ObjP, env: &ObjP) -> EvalResult {
    match form.borrow() {
        Object::Cons {car: op, cdr: args} => { combine(&eval(op, env)?, args, env) }
        Object::Symbol(_name) => { lookup(form, env) }
        _ => { Ok(form.clone()) } // self evaluating
    }
}

// Get the value from an alist, or None if it ain't there.
// Unlike assoc, panics on error (our env is an alist by construction so they
//  should never happen) and returns the value rather than the pair.
fn assocv(key: &ObjP, alist: &ObjP) -> Option<ObjP> {
    match alist.borrow() {
        Object::Null => { None }
        Object::Cons {car, cdr} => {
            match car.borrow() {
                Object::Cons {car: caar, cdr: cdar} => {
                    if key == caar {
                        Some(cdar.clone())
                    } else { assocv(key, cdr) }
                }
                _ => { panic!("Environment {} is ill-formed!", alist) }
            }
        }
        _ => { panic!("Environment {} is ill-formed!", alist) }
    }
}

fn lookup(name: &ObjP, env: &ObjP) -> EvalResult {
    match assocv(name, env) {
        Some(value) => { Ok(value) }
        None => { Err(EvalError::Unbound(name.clone())) }
    }
}

fn evlis(forms: &ObjP, env: &ObjP) -> EvalResult {
    match forms.borrow() {
        Object::Cons {car, cdr} => {
            Ok(cons(&eval(car, env)?, &evlis(cdr, env)?))
        }
        Object::Null => { Ok(forms.clone()) }
        _ => { Err(EvalError::ImproperList(forms.clone())) }
    }
}

fn combine(combiner: &ObjP, combinand: &ObjP, env: &ObjP)
           -> EvalResult {
    match combiner.borrow() {
        Object::Subr(fun) => { fun(&evlis(combinand, env)?) }
        Object::Fsubr(fun) => { fun(combinand, env) }
        Object::Expr {form, lambda_list: ll, env} => {
            eval(form, &augment(env, ll, &evlis(combinand, env)?)?)
        }
        _ => { Err(EvalError::NotCombiner(combiner.clone())) }
    }
}

fn augment(env: &ObjP, lambda_list: &ObjP, values: &ObjP) -> EvalResult {
    // This inner function does the obvious recursive thing,
    // but reports errors using the originals.
    fn augment_aux(oll: &ObjP, ovs: &ObjP,
                   ll: &ObjP, vs: &ObjP, env: &ObjP) -> EvalResult {
        match ll.borrow() {
            Object::Null => {
                match vs.borrow() {
                    Object::Null => { Ok(env.clone()) }
                    _ => { Err(EvalError::TooManyArgs(oll.clone(), ovs.clone())) }
                }
            }
            Object::Cons { car: lcar, cdr: lcdr } => {
                // Here we assume the lambda list has a symbol in its car.
                // This could be checked at expr construction time.
                // (Will I be too lazy to do so? Probably.)
                match lcdr.borrow() {
                    Object::Cons { car: ref vcar, cdr: ref vcdr } => {
                        augment_aux(oll, ovs, lcdr, vcdr, &acons(lcar, vcar, env))
                    }
                    _ => { Err(EvalError::NotEnoughArgs(oll.clone(), ovs.clone())) }
                }
            }
            Object::Symbol(_) => { Ok(acons(ll, vs, env)) }
            // invalid lambda list: should be impossible by construction
            _ => { panic!("Bad lambda list {}", oll) }
        }
    }
    augment_aux(lambda_list, values, lambda_list, values, env)
}
