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
    assert_eq!(MyType::freeze(), 15974934573951822791);
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
    assert_eq!(MyType::freeze(), 17438168895362026898);
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

    #[derive(Freezable)]
    enum MyType4 {
        A,
        #[assume_frozen]
        OtherVariantName(NonFreezable),
    }
    assert_ne!(MyType3::freeze(), MyType4::freeze());
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
#[test]
fn variant_order() {
    #[derive(Freezable)]
    enum MyType1 {
        B(u32),
        A(u64),
    }
    #[derive(Freezable)]
    enum MyType2 {
        A(u64),
        B(u32),
    }

    assert_eq!(MyType1::freeze(), MyType2::freeze());
}

#[test]
fn enum_discriminant() {
    #[derive(Freezable)]
    enum MyType1 {
        A = 1,
        B = 4,
    }

    #[derive(Freezable)]
    enum MyType2 {
        A = 0 + 1,
        B = 3,
    }

    #[derive(Freezable)]
    enum MyType3 {
        A = 0 + 1,
        B = 2 + 2,
    }

    assert_eq!(MyType1::freeze(), MyType3::freeze());
    assert_ne!(MyType1::freeze(), MyType2::freeze());
}

#[test]
fn enum_variant_inner_type_order() {
    #[derive(Freezable)]
    enum MyType1 {
        A(u8, u32),
    }

    #[derive(Freezable)]
    enum MyType2 {
        A(u32, u8),
    }

    assert_ne!(MyType1::freeze(), MyType2::freeze());
}

#[test]
fn tuple() {
    #[derive(Freezable)]
    enum MyType1 {
        A((u32)),
        B((u32, u8)),
        C((u32, u64, (u64, u64, u64, u64))),
        D((u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)),
    }

    assert_eq!(MyType1::freeze(), 726139213363783291);
}

#[test]
fn external_generic() {
    struct External1<T> {
        a: T,
    }
    struct External2<T> {
        a: T,
    }

    #[derive(Freezable)]
    enum MyType1 {
        A,
    }
    #[derive(Freezable)]
    enum MyType2 {
        B,
        C,
    }
    assert_ne!(MyType1::freeze(), MyType2::freeze());

    #[derive(Freezable)]
    struct MyType3 {
        #[assume_frozen]
        a: External1<MyType1>,
    }

    #[derive(Freezable)]
    struct MyType4 {
        #[assume_frozen(freeze_generics)]
        a: External1<MyType2>,
    }

    #[derive(Freezable)]
    struct MyType5 {
        #[assume_frozen(freeze_generics)]
        a: External2<MyType2>,
    }
    #[derive(Freezable)]
    struct MyType6 {
        #[assume_frozen(freeze_generics)]
        a: External2<MyType1>,
    }
    assert_ne!(MyType3::freeze(), MyType4::freeze());
    assert_eq!(MyType5::freeze(), MyType4::freeze());
    assert_ne!(MyType5::freeze(), MyType6::freeze());

    #[derive(Freezable)]
    enum MyEnumType3 {
        #[assume_frozen]
        A(External1<MyType1>),
    }

    #[derive(Freezable)]
    enum MyEnumType4 {
        #[assume_frozen(freeze_generics)]
        A(External1<MyType2>),
    }

    #[derive(Freezable)]
    enum MyEnumType5 {
        #[assume_frozen(freeze_generics)]
        A(External2<MyType2>),
    }
    #[derive(Freezable)]
    enum MyEnumType6 {
        #[assume_frozen(freeze_generics)]
        A(External2<MyType1>),
    }

    assert_ne!(MyEnumType3::freeze(), MyEnumType4::freeze());
    assert_eq!(MyEnumType5::freeze(), MyEnumType4::freeze());
    assert_ne!(MyEnumType5::freeze(), MyEnumType6::freeze());
}
