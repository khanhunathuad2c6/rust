//@ check-fail
//@ known-bug: #148283

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
pub struct B;
pub struct C;
pub struct D;
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

trait Trait {
    fn self_ty(&self);

    fn complete(&self) -> Vec<&'static str> {
        self.self_ty();
        let actions = unsafe { ACTIONS.clone() };
        unsafe { ACTIONS.clear() };
        actions
    }
}

impl Trait for TopType {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty TopType"); }
    }
}

impl Trait for ArrayWrapper {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty ArrayWrapper"); }
    }
}

impl Trait for IntWrapper {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty IntWrapper"); }
    }
}

impl Trait for UnsizedArray {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty UnsizedArray"); }
    }
}

impl Trait for FinalType {
    fn self_ty(&self) {
        unsafe { ACTIONS.push("self_ty FinalType"); }
    }
}

fn deref_to_dyn() {
    let x = match 0 {
        0 => &TopTypeNoTrait as &TopTypeNoTrait,
        1 => &TopTypeNoTrait as &FinalType,
        2 => &TopTypeNoTrait as &FinalType as &dyn Trait,
        _ => loop {},
    };
}

fn deref_to_dyn_direct() {
    let x = match 0 {
        0 => &TopTypeNoTrait as &TopTypeNoTrait,
        1 => &TopTypeNoTrait as &FinalType as &dyn Trait,
        _ => loop {},
    };
}

fn direct_to_dyn() {
    let x = &TopTypeNoTrait as &FinalType as &dyn Trait;
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
