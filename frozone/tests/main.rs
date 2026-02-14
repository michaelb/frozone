#![allow(unused)]
use frozone::Freezable;

#[macro_use]
extern crate static_assertions;

#[test]
fn u64() {
    assert_eq!(u64::freeze(), 4711862451334276647);
    assert_eq!(std::primitive::u64::freeze(), 4711862451334276647);
}

// #[test]
// fn const_eval() {
//     static_assertions::const_assert_eq!(u64::freeze(), 4711862451334276647);
// }

#[test]
fn derive() {
    #[derive(Freezable)]
    struct MyType {
        field_a: u64,
        field_b: u32,
        field_c: u64,
        field_d: core::ffi::c_int,
        field_e: MySubType,
    }

    #[derive(Freezable)]
    struct MySubType {
        field_a: u64,
    }

    let mytype_freeze = MyType::freeze();
    assert_eq!(mytype_freeze, 5861431272991508562);

    {
        // check the freeze doesn't just operate on the name
        #[derive(Freezable)]
        struct MyType {
            field_a: u32,
        }
        assert_ne!(MyType::freeze(), mytype_freeze);
    }
}

#[test]
fn derive_container() {
    #[derive(Freezable)]
    struct MyType {
        field_c: Vec<u64>,
        field_a: [u32; 67],
        field_e: Box<MySubType>,
    }

    #[derive(Freezable)]
    struct MySubType {
        field_a: u64,
    }

    assert_eq!(MyType::freeze(), 4080615902123853835);
}
#[test]
fn derive_generic() {
    #[derive(Freezable)]
    struct MyType<T: frozone::Freezable> {
        field_c: T,
        field_d: Box<T>,
    }

    assert_eq!(MyType::<u64>::freeze(), 11469247882469511076);

    #[derive(Freezable)]
    struct MyType2<'a> {
        field_a: &'a [u64],
    }
    assert_eq!(MyType2::freeze(), 14635643794014186131);
}

#[test]
fn derive_ptr() {
    #[derive(Freezable)]
    struct MyType {
        field_c: *const u8,
    }
    assert_eq!(MyType::freeze(), 12713665718889934710);
}

#[test]
fn derive_enum() {
    #[derive(Freezable)]
    enum MyType {
        Unit,
        Unnamed(u64),
        Unnamed2(u64, u32, u64),
    }
    assert_eq!(MyType::freeze(), 13938101513925945732);
}

#[test]
fn derive_composite() {
    #[derive(Freezable)]
    struct MyType3 {
        a: u64,
    }

    #[derive(Freezable)]
    enum MyType2 {
        MyType3,
    }
    #[derive(Freezable)]
    enum MyType {
        Unit,
        Unnamed(u64),
        MyType2,
    }
    assert_eq!(MyType::freeze(), 17749802890545832962);
}

#[test]
fn derive_with_assume_frozen_field() {
    #[derive(Freezable)]
    enum MyType1 {
        A,
        #[assume_frozen]
        B(u32),
    }

    #[derive(Freezable)]
    enum MyType2 {
        A,
        #[assume_frozen]
        B(i64),
    }

    enum NonFreezable {
        C,
    }

    #[derive(Freezable)]
    enum MyType3 {
        A,
        #[assume_frozen]
        B(NonFreezable),
    }

    assert_eq!(MyType1::freeze(), MyType2::freeze());
    assert_eq!(MyType3::freeze(), MyType2::freeze());
}
#[test]
fn derive_with_assume_frozen_variant() {
    #[derive(Freezable)]
    struct MyType1 {
        a: u64,
        #[assume_frozen]
        b: u32,
    }

    #[derive(Freezable)]
    struct MyType2 {
        a: u64,
        #[assume_frozen]
        b: u64,
    }

    struct NonFreezable {
        a: i32,
    }

    #[derive(Freezable)]
    struct MyType3 {
        a: u64,
        #[assume_frozen]
        b: NonFreezable,
    }

    assert_eq!(MyType1::freeze(), MyType2::freeze());
    assert_eq!(MyType3::freeze(), MyType2::freeze());
}

#[test]
fn field_order() {
    #[derive(Freezable)]
    struct MyType1 {
        a: u64,
        b: u64,
    }

    #[derive(Freezable)]
    struct MyType2 {
        b: u64,
        a: u64,
    }

    assert_eq!(MyType1::freeze(), MyType2::freeze());
}
