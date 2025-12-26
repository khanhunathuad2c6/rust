//@ run-pass

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

fn simple() {
    let x = match 0 {
        0 => &Wrap([]) as &ArrayWrapper,
        _ => &Wrap([]) as &UnsizedArray,
    };
    assert_eq!(x.complete(), vec!["self_ty UnsizedArray"]);
}

fn long_chain() {
    let a = match 0 {
        0 => &TopType      as &TopType,
        1 => &Wrap([])     as &ArrayWrapper,
        2 => &IntWrapper   as &IntWrapper,
        3 => &Wrap([])     as &UnsizedArray,
        4 => &FinalType    as &FinalType,
        _ => loop {},
    };
    assert_eq!(
        a.complete(),
        vec![
            "deref TopType->ArrayWrapper",
            "deref ArrayWrapper->IntWrapper",
            "deref IntWrapper->UnsizedArray",
            "deref UnsizedArray->FinalType",
            "self_ty FinalType",
        ],
    );

    let b = match 0 {
        0 => &TopType      as &TopType,
        1 => &Wrap([])     as &ArrayWrapper,
        // IntWrapper arm removed
        2 => &Wrap([])     as &UnsizedArray,
        3 => &FinalType    as &FinalType,
        _ => loop {},
    };
    assert_eq!(
        b.complete(),
        vec![
            "deref TopType->ArrayWrapper",
            "deref ArrayWrapper->IntWrapper",
            "deref IntWrapper->UnsizedArray",
            "deref UnsizedArray->FinalType",
            "self_ty FinalType",
        ],
    );
}

fn order_dependence() {
    let a = match 0 {
        0 => &Wrap([])   as &ArrayWrapper,
        1 => &IntWrapper as &IntWrapper,
        2 => &Wrap([])   as &UnsizedArray,
        _ => loop {},
    };
    assert_eq!(a.complete(), vec!["self_ty UnsizedArray"]);

    unsafe { ACTIONS.clear() }
    let b = match 0 {
        0 => &Wrap([])   as &ArrayWrapper,
        1 => &Wrap([])   as &UnsizedArray,
        2 => &IntWrapper as &IntWrapper,
        _ => loop {},
    };
    assert_eq!(b.complete(), vec!["self_ty UnsizedArray"]);
}


fn main() {
    simple();
    long_chain();
    order_dependence();
}
