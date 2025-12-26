//@ check-fail
//@ known-bug: #00000

#![feature(unsize, coerce_unsized)]
#![allow(static_mut_refs)]
#![allow(dead_code)]
use std::ops::Deref;

static mut ACTIONS: Vec<&'static str> = Vec::new();

trait Trait {
    fn self_ty(&self);

    fn complete(&self) -> Vec<&'static str> {
        self.self_ty();
        let actions = unsafe { ACTIONS.clone() };
        unsafe { ACTIONS.clear() };
        actions
    }
}

macro_rules! do_trait_impl {
    ($self:ident, $self_ty:literal) => {
        impl Trait for $self {
            fn self_ty(&self) {
                unsafe { ACTIONS.push($self_ty); }
            }
        }
    }    
}

trait Dynable {}
struct Inner;
impl Dynable for Inner {}

struct Wrap4<T: ?Sized>(T);

struct O;
struct P;
struct Q;

do_trait_impl!(O, "self_ty O");
do_trait_impl!(P, "self_ty P");
do_trait_impl!(Q, "self_ty Q");

impl Deref for O {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref O->P"); }
        &P
    }
}
impl Deref for P {
    type Target = Q;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref P->Q"); }
        &Q
    }
}
impl Deref for Q {
    type Target = P;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref Q->P"); }
        &P
    }
}

fn order_lub() {
    let a = match 0 {
        0 => &O      as &O,
        1 => &P      as &P,
        2 => &Q      as &Q,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty P"]);
}

fn main() {
    order_lub();
}
