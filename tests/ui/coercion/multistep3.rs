//@ check-pass

#![feature(unsize, coerce_unsized)]
#![allow(static_mut_refs)]
#![allow(dead_code)]
use std::ops::Deref;

pub static mut ACTIONS: Vec<&'static str> = Vec::new();

pub struct Wrap<T: ?Sized>(T);

impl<'b, T: ?Sized + std::marker::Unsize<U> + std::ops::CoerceUnsized<U>, U: ?Sized>
    std::ops::CoerceUnsized<Wrap<U>> for Wrap<T> {}

struct Inner;
type A = Wrap<Inner>;
type B = Wrap<dyn Trait + Send>;
type C = Wrap<dyn Trait>;

impl Deref for C {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref B->C"); }
        &Wrap(Inner)
    }
}

/*
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
        &Wrap
    }
}
impl Deref for D {
    type Target = C;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref C->D"); }
        &C
    }
}
*/
impl Trait for A {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty A"); }
    }
}
/*
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
*/
impl Trait for Inner {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty Inner"); }
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
    /**/
    let a = match 0 {
        0 => &Wrap(Inner)      as &A,
        1 => &Wrap(Inner)      as &B,
        2 => &Wrap(Inner)      as &C,
        _ => loop {},
    };
    assert_eq!(
        std::any::type_name_of_val(a),
        "playground::Wrap<dyn playground::Trait + core::marker::Send>",
    );
    assert_eq!(a.0.complete(), vec!["self_ty Inner"]);
    let a = match 0 {
        0 => &Wrap(Inner)      as &A,
        2 => &Wrap(Inner)      as &C,
        1 => &Wrap(Inner)      as &B,
        _ => loop {},
    };
    assert_eq!(std::any::type_name_of_val(a), "playground::Wrap<dyn playground::Trait>");
    assert_eq!(a.0.complete(), vec!["self_ty Inner"]);
        /*
    let b = match 0 {
        0 => &A          as &A,
        1 => &Wrap([])   as &B,
        3 => &Wrap([])   as &D,
        2 => &C          as &C,
        _ => loop {},
    };
    assert_eq!(b.complete(), vec!["self_ty UnsizedArray"]);
    */
}

fn main() {
    order_lub();
}
