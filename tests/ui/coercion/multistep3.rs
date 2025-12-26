//@ run-pass

#![feature(unsize, coerce_unsized)]
#![allow(static_mut_refs)]
#![allow(dead_code)]
#![allow(unused_macros)]
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

struct Wrap3<T: ?Sized>(T);

impl<'b, T: ?Sized + std::marker::Unsize<U> + std::ops::CoerceUnsized<U>, U: ?Sized>
    std::ops::CoerceUnsized<Wrap3<U>> for Wrap3<T> {}

type I = Wrap3<Inner>;
type J = Wrap3<dyn Dynable + Send>;
type K = Wrap3<dyn Dynable>;

do_trait_impl!(I, "self_ty I");
do_trait_impl!(J, "self_ty J");
do_trait_impl!(K, "self_ty K");

impl Deref for K {
    type Target = J;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref K->J"); }
        &Wrap3(Inner)
    }
}

fn order_lub() {
    let a = match 0 {
        0 => &Wrap3(Inner)      as &I,
        1 => &Wrap3(Inner)      as &J,
        2 => &Wrap3(Inner)      as &K,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty J"]);
    let a = match 0 {
        0 => &Wrap3(Inner)      as &I,
        2 => &Wrap3(Inner)      as &K,
        1 => &Wrap3(Inner)      as &J,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty K"]);
}

fn main() {
    order_lub();
}
