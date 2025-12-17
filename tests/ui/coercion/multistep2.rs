//@ check-fail
//@ known-bug: #00000

#![allow(static_mut_refs)]
#![allow(dead_code)]
use std::ops::Deref;

pub static mut ACTIONS: Vec<&'static str> = Vec::new();

pub struct Wrap<T: ?Sized>(T);

// Deref Chain: FinalType <- UnsizedArray <- IntWrapper <- ArrayWrapper <- TopType
pub struct TopType;
pub type ArrayWrapper = Wrap<[i32; 0]>;
pub struct IntWrapper;
pub type UnsizedArray = Wrap<[i32]>;
pub struct FinalType;
pub struct TopTypeNoTrait;

pub struct A;
pub type B = Wrap<[i32; 0]>;
pub struct C;
pub type D = Wrap<[i32]>;

impl Deref for A {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref A->B"); }
        &Wrap([])
    }
}
impl Deref for B {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref B->D"); }
        &C
    }
}
impl Deref for D {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref C->D"); }
        &C
    }
}
impl Trait for A {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty A"); }
    }
}
impl Trait for B {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty B"); }
    }
}
impl Trait for C {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty C"); }
    }
}
impl Trait for D {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty D"); }
    }
}


trait Trait {
    fn self_ty(&self);

    fn complete(&self) -> Vec<&'static str> {
        self.self_ty();
        let actions = unsafe { ACTIONS.clone() };
        unsafe { ACTIONS.clear() };
        actions
    }
}

fn order_lub() {
    let a = match 0 {
        0 => &A          as &A,
        1 => &Wrap([])   as &B,
        2 => &C          as &C,
        3 => &Wrap([])   as &D,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty UnsizedArray"]);
    let b = match 0 {
        0 => &A          as &A,
        1 => &Wrap([])   as &B,
        3 => &Wrap([])   as &D,
        2 => &C          as &C,
        _ => loop {},
    };
    assert_eq!(b.complete(), vec!["self_ty UnsizedArray"]);
}

fn main() {
    order_lub();
}
