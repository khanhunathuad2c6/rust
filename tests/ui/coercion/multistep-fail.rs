//@ check-fail
//@ known-bug: #148283

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

struct Wrap<T: ?Sized>(T);

// Deref Chain: FinalType <- UnsizedArray <- IntWrapper <- ArrayWrapper <- TopType
struct TopType;
type ArrayWrapper = Wrap<[i32; 0]>;
struct IntWrapper;
type UnsizedArray = Wrap<[i32]>;
struct FinalType;
struct TopTypeNoTrait;

do_trait_impl!(TopType, "self_ty TopType");
do_trait_impl!(ArrayWrapper, "self_ty ArrayWrapper");
do_trait_impl!(IntWrapper, "self_ty IntWrapper");
do_trait_impl!(UnsizedArray, "self_ty UnsizedArray");
do_trait_impl!(FinalType, "self_ty FinalType");
do_trait_impl!(TopTypeNoTrait, "self_ty TopTypeNoTrait");
impl Dynable for FinalType {}

impl Deref for TopType {
    type Target = ArrayWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref TopType->ArrayWrapper"); }
        &Wrap([])
    }
}

impl Deref for ArrayWrapper {
    type Target = IntWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref ArrayWrapper->IntWrapper"); }
        &IntWrapper
    }
}

impl Deref for IntWrapper {
    type Target = UnsizedArray;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref IntWrapper->UnsizedArray"); }
        &Wrap([])
    }
}

impl Deref for UnsizedArray {
    type Target = FinalType;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref UnsizedArray->FinalType"); }
        &FinalType
    }
}

impl Deref for TopTypeNoTrait {
    type Target = ArrayWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref TopTypeNoTrait->ArrayWrapper"); }
        &Wrap([])
    }
}

struct A;
struct B;
struct C;
struct D;

do_trait_impl!(A, "self_ty A");
do_trait_impl!(B, "self_ty B");
do_trait_impl!(C, "self_ty C");
do_trait_impl!(D, "self_ty D");


impl Deref for A {
    type Target = B;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref A->B"); }
        &B
    }
}
impl Deref for B {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref B->D"); }
        &D
    }
}
impl Deref for C {
    type Target = D;
    fn deref(&self) -> &Self::Target {
        unsafe { ACTIONS.push("deref C->D"); }
        &D
    }
}

fn deref_to_dyn() {
    let x = match 0 {
        0 => &TopTypeNoTrait as &TopTypeNoTrait,
        1 => &TopTypeNoTrait as &FinalType,
        2 => &TopTypeNoTrait as &FinalType as &dyn Dynable,
        _ => loop {},
    };
}

fn deref_to_dyn_direct() {
    let x = match 0 {
        0 => &TopTypeNoTrait as &TopTypeNoTrait,
        1 => &TopTypeNoTrait as &FinalType as &dyn Dynable,
        _ => loop {},
    };
}

fn direct_to_dyn() {
    let x = &TopTypeNoTrait as &FinalType as &dyn Dynable;
}

fn skipped_coerce() {
    let a = match 0 {
        0 => &A          as &A,
        1 => &B          as &B,
        2 => &C          as &C,
        3 => &D          as &D,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty UnsizedArray"]);
    let b = match 0 {
        3 => &D          as &D,
        0 => &A          as &A,
        1 => &B          as &B,
        2 => &C          as &C,
        _ => loop {},
    };
    assert_eq!(b.complete(), vec!["self_ty UnsizedArray"]);
}
fn main() {
    deref_to_dyn();
    deref_to_dyn_direct();
    direct_to_dyn();
    skipped_coerce();
}
