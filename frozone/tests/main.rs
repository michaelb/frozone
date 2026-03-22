#![allow(unused)]
#![allow(unexpected_cfgs)]
use frozone::Freezable;

#[macro_use]
extern crate static_assertions;

#[test]
fn basic() {
    #[derive(Freezable)]
    struct MyType {
        field: u64,
    }
    #[derive(Freezable)]
    struct MyType2 {
        field_2: MyType,
    }
    assert_eq!(MyType2::freeze(), 9299309758483068998);
}
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

    assert_eq!(MyType::freeze(), 8960718244642525187);
}

#[test]
fn derive_generic() {
    #[derive(Freezable)]
    struct MyType<T: frozone::Freezable> {
        field_c: T,
        field_d: Box<T>,
    }

    assert_eq!(MyType::<u64>::freeze(), 8534488491191601984);

    #[derive(Freezable)]
    struct MyType2<'a> {
        field_a: &'a [u64],
    }
    assert_eq!(MyType2::freeze(), 2482863297349756199);

    #[derive(Freezable)]
    struct MyType3<T: frozone::Freezable, U: Freezable, V: Freezable> {
        field_d: Box<T>,
        field_e: Box<U>,
        field_f: Box<V>,
    }
    assert_eq!(
        MyType3::<u64, Option<()>, Result<u64, u32>>::freeze(),
        2581063238814899371
    );
}

#[test]
fn derive_ptr() {
    #[derive(Freezable)]
    struct MyType {
        field_c: *const u8,
    }
    #[derive(Freezable)]
    struct MyType2 {
        field_c: *mut u8,
    }
    assert_eq!(MyType::freeze(), 9315545481621597728);
    assert_eq!(MyType2::freeze(), 14445949281328555364);
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

    assert_eq!(MyType1::freeze(), 5870148933263435734);
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

#[test]
fn struct_names_non_importance() {
    // names of the structs shouldn't matter to frozone
    #[derive(Freezable)]
    enum MyType1 {
        A(u8, u32),
    }

    #[derive(Freezable)]
    enum MyType2 {
        A(u8, u32),
    }

    #[derive(Freezable)]
    struct MyType3 {
        a: (u8, u32),
    }

    #[derive(Freezable)]
    struct MyType4 {
        a: (u8, u32),
    }

    assert_eq!(MyType1::freeze(), MyType2::freeze());
    assert_eq!(MyType3::freeze(), MyType4::freeze());

    #[derive(Freezable)]
    enum MyType5 {
        A(u8, MyType1),
    }

    #[derive(Freezable)]
    enum MyType6 {
        A(u8, MyType2),
    }
    // works because MyType1 and MyType2 are the same
    assert_eq!(MyType3::freeze(), MyType4::freeze());

    #[derive(Freezable)]
    struct MyType7 {
        a: MyType1,
    }

    #[derive(Freezable)]
    struct MyType8 {
        a: MyType2,
    }
    // works because MyType1 and MyType2 are the same
    assert_eq!(MyType7::freeze(), MyType8::freeze());

    #[derive(Freezable)]
    struct MyType9 {
        a: (u8, MyType1),
    }

    #[derive(Freezable)]
    struct MyType10 {
        a: (u8, MyType2),
    }
    // works because MyType1 and MyType2 are the same
    assert_eq!(MyType10::freeze(), MyType9::freeze());
}

#[test]
#[cfg(feature = "alloc")]
fn full_type_names() {
    #[derive(Freezable)]
    struct MyType1 {
        a: String,
    }

    #[derive(Freezable)]
    struct MyType2 {
        a: std::string::String,
    }
    extern crate alloc;
    #[derive(Freezable)]
    struct MyType3 {
        a: alloc::string::String,
    }
    assert_eq!(MyType2::freeze(), MyType1::freeze());
    assert_eq!(MyType3::freeze(), MyType1::freeze());
}

#[test]
fn variants_cfg() {
    // names of the structs shouldn't matter to frozone
    #[derive(Freezable)]
    enum MyType1 {
        #[cfg(test)]
        A(u8, u32),
        #[cfg(not(test))] // configure out a struct member
        B(u8, u32),
    }
    #[derive(Freezable)]
    enum MyType2 {
        A(u8, u32),
    }
    assert_eq!(MyType1::freeze(), MyType2::freeze());

    #[derive(Freezable)]
    enum MyType3 {
        #[cfg(not(inexistent_feature))] // configure out a struct member
        A(u8, u32),
        #[cfg(inexistent_feature)]
        B(u8, u32),
    }
    #[derive(Freezable)]
    enum MyType4 {
        A(u8, u32),
    }
    assert_eq!(MyType3::freeze(), MyType4::freeze());
}

#[test]
fn fields_cfg() {
    // names of the structs shouldn't matter to frozone
    #[derive(Freezable)]
    struct MyType1 {
        a: u8,
        #[cfg(not(test))] // configure out a struct member
        b: u8,
    }
    #[derive(Freezable)]
    struct MyType2 {
        #[cfg(test)]
        a: u8,
    }
    assert_eq!(MyType1::freeze(), MyType2::freeze());

    #[derive(Freezable)]
    struct MyType3 {
        a: u8,
        #[cfg(not(inexistent_feature))] // configure out a struct member
        b: u8,
    }
    #[derive(Freezable)]
    struct MyType4 {
        a: u8,
        b: u8,
        #[cfg(inexistent_feature)]
        c: u8,
    }
    assert_eq!(MyType3::freeze(), MyType4::freeze());

    #[derive(Freezable)]
    struct MyType5 {
        a: u8,
        #[cfg(not(feature = "inexistent_feature"))] // configure out a struct member
        b: u8,
    }
    #[derive(Freezable)]
    struct MyType6 {
        a: u8,
        b: u8,
        #[cfg(feature = "inexistent_feature")]
        c: u8,
    }
    assert_eq!(MyType5::freeze(), MyType6::freeze());
}

#[test]
fn zero_sized_type() {
    #[derive(Freezable)]
    struct Empty;
    assert_eq!(Empty::freeze(), 0);
    #[derive(Freezable)]
    struct Empty2 {}
    assert_eq!(Empty2::freeze(), 0);
    #[derive(Freezable)]
    enum Empty3 {}
    assert_eq!(Empty3::freeze(), 0);
    // #[derive(Freezable)]
    // type Empty4;
    // assert_eq!(Empty4::freeze(), 0);
}

#[test]
fn recursive_types() {
    #[derive(Freezable)]
    struct MyType1 {
        a: Option<MyType2>,
    }
    #[derive(Freezable)]
    struct MyType2 {
        a: Option<&'static MyType1>,
    }
    assert_eq!(MyType1::freeze(), 11355567332989466119);

    #[derive(Freezable)]
    enum MyType3 {
        A(Option<MyType4>),
    }
    #[derive(Freezable)]
    enum MyType4 {
        A(Option<&'static MyType3>),
    }
    assert_eq!(MyType3::freeze(), 2378750370645629984);
}

#[test]
fn recursive_types_advanced() {
    #[derive(Freezable)]
    struct Cycle2_1 {
        a: Box<Option<Cycle2_2>>,
    }
    #[derive(Freezable)]
    struct Cycle2_2 {
        a: Box<Option<Cycle2_1>>,
    }

    #[derive(Freezable)]
    struct Cycle3_1 {
        a: Box<Option<Cycle3_2>>,
    }
    #[derive(Freezable)]
    struct Cycle3_2 {
        a: Box<Option<Cycle3_3>>,
    }
    #[derive(Freezable)]
    struct Cycle3_3 {
        a: Box<Option<Cycle3_1>>,
    }
    assert_eq!(Cycle2_1::freeze(), Cycle2_2::freeze());
    assert_eq!(Cycle3_1::freeze(), Cycle3_2::freeze());
    assert_eq!(Cycle3_1::freeze(), Cycle3_3::freeze());
    assert_ne!(Cycle2_1::freeze(), Cycle3_1::freeze());
}

#[test]
fn complex() {
    // test on iced Application structure
    pub trait Program: Sized {
        type State: Freezable;
        type Message: Send + 'static + Freezable;
        type Theme: Base;
        type Renderer: Renderer;
        type Executor: Executor;
    }
    pub trait Base {}
    pub trait Renderer {}
    pub trait Executor: Sized {}
    #[derive(Freezable)]
    pub struct Size<T: Freezable = f32> {
        pub width: T,
        pub height: T,
    }
    #[derive(Freezable)]
    pub struct Point<T: Freezable = f32> {
        pub x: T,
        pub y: T,
    }
    #[derive(Freezable)]
    pub enum Position {
        Default,
        Centered,
        Specific(Point),
        #[assume_frozen] // TODO: limitation to fix
        SpecificWith(fn(Size, Size) -> Point),
    }
    #[derive(Freezable)]
    pub struct Color {
        pub r: f32,
        pub g: f32,
        pub b: f32,
        pub a: f32,
    }
    #[derive(Freezable)]
    pub struct Palette {
        pub background: Color,
        pub text: Color,
        pub primary: Color,
        pub success: Color,
        pub warning: Color,
        pub danger: Color,
    }
    #[derive(Freezable)]
    pub struct Pair {
        pub color: Color,
        pub text: Color,
    }
    #[derive(Freezable)]
    pub struct Background {
        pub base: Pair,
        pub weakest: Pair,
        pub weaker: Pair,
        pub weak: Pair,
        pub neutral: Pair,
        pub strong: Pair,
        pub stronger: Pair,
        pub strongest: Pair,
    }
    #[derive(Freezable)]
    pub struct Primary {
        pub base: Pair,
        pub weak: Pair,
        pub strong: Pair,
    }
    #[derive(Freezable)]
    pub struct Secondary {
        pub base: Pair,
        pub weak: Pair,
        pub strong: Pair,
    }
    #[derive(Freezable)]
    pub struct Success {
        pub base: Pair,
        pub weak: Pair,
        pub strong: Pair,
    }
    #[derive(Freezable)]
    pub struct Danger {
        pub base: Pair,
        pub weak: Pair,
        pub strong: Pair,
    }
    #[derive(Freezable)]
    pub struct Warning {
        pub base: Pair,
        pub weak: Pair,
        pub strong: Pair,
    }
    #[derive(Freezable)]
    pub struct Extended {
        pub background: Background,
        pub primary: Primary,
        pub secondary: Secondary,
        pub success: Success,
        pub warning: Warning,
        pub danger: Danger,
        pub is_dark: bool,
    }
    #[derive(Freezable)]
    pub struct Custom {
        #[assume_frozen] // real limitation, Cow's generic type must impl clone
        // for it to be freezable
        name: std::borrow::Cow<'static, str>,
        palette: Palette,
        extended: Extended,
    }
    impl Base for Theme {}
    use std::sync::Arc;
    #[derive(Freezable)]
    pub enum Theme {
        Light,
        Dark,
        Dracula,
        Nord,
        SolarizedLight,
        SolarizedDark,
        GruvboxLight,
        GruvboxDark,
        CatppuccinLatte,
        CatppuccinFrappe,
        CatppuccinMacchiato,
        CatppuccinMocha,
        TokyoNight,
        TokyoNightStorm,
        TokyoNightLight,
        KanagawaWave,
        KanagawaDragon,
        KanagawaLotus,
        Moonfly,
        Nightfly,
        Oxocarbon,
        Ferra,
        Custom(Arc<Custom>),
    }
    #[derive(Freezable)]
    pub enum Level {
        Normal,
        AlwaysOnBottom,
        AlwaysOnTop,
    }
    #[derive(Freezable)]
    pub struct Icon {
        rgba: Vec<u8>,
        size: Size<u32>,
    }
    #[derive(Freezable)]
    pub struct PlatformSpecific {
        pub application_id: String,
        pub override_redirect: bool,
    }
    #[derive(Freezable)]
    pub struct Settings {
        pub size: Size,
        pub maximized: bool,
        pub fullscreen: bool,
        pub position: Position,
        pub min_size: Option<Size>,
        pub max_size: Option<Size>,
        pub visible: bool,
        pub resizable: bool,
        pub closeable: bool,
        pub minimizable: bool,
        pub decorations: bool,
        pub transparent: bool,
        pub blur: bool,
        pub level: Level,
        pub icon: Option<Icon>,
        pub platform_specific: PlatformSpecific,
        pub exit_on_close_request: bool,
    }

    #[derive(Freezable)]
    pub struct Instant(std::time::Instant);

    #[derive(Freezable)]
    pub enum RedrawRequest {
        NextFrame,
        At(Instant),
        Wait,
    }
    #[derive(Freezable)]
    pub enum Status {
        Ignored,
        Captured,
    }
    #[derive(Freezable)]
    pub struct Action<Message: Freezable> {
        message_to_publish: Option<Message>,
        redraw_request: RedrawRequest,
        event_status: Status,
    }
    pub trait Stream {
        type Item;
    }
    pub type BoxStream<T> = core::pin::Pin<Box<dyn Stream<Item = T> + Send>>;
    #[derive(Freezable)]
    pub struct Task<T: Freezable> {
        #[assume_frozen] // TODO: limitation to fix
        stream: Option<BoxStream<Action<T>>>,
        units: usize,
    }

    #[derive(Freezable)]
    pub struct Preset<State: Freezable, Message: Freezable> {
        #[assume_frozen] // real limitation, Cow's generic type must impl clone
        // for it to be freezable
        name: std::borrow::Cow<'static, str>,
        #[assume_frozen] // TODO: limitation to fix
        boot: Box<dyn Fn() -> (State, Task<Message>)>,
    }

    #[derive(Freezable)]
    pub struct Application<P: Program + Freezable> {
        raw: P,
        settings: Settings,
        window: Settings,
        presets: Vec<Preset<P::State, P::Message>>,
    }

    #[derive(Freezable)]
    pub struct MyState {
        a: u64,
    }
    #[derive(Freezable)]
    pub enum MyMessage {
        A,
    }

    impl Executor for () {}
    impl Renderer for () {}

    type Main = ();
    impl Program for Main {
        type State = MyState;
        type Message = MyMessage;
        type Theme = Theme;
        type Executor = ();
        type Renderer = ();
    }

    assert_eq!(Application::<Main>::freeze(), 1146165835468690553)
}
