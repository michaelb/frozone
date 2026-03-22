# frozone

`frozone` is a crate that allows you to 'freeze' structs and enums
recursively to ensure they never get changed, helping enforcing
backwards-compatibility or API stability (arewesemveryet.org ?)

## Use case

Let's say there is a very important data structure for your application:

```rust
#[derive(Serialize, Deserialize)]
struct Frozone {
    power: Power,
    equipment: Vec<Part>,
    super_suit: Suit
}

#[derive(Serialize, Deserialize)]
enum Part {
    Sunglasses,
    Skates,
    //...
}
```

that gets serialized in `~/.config/frozone/state.json`
on each user's machine. You've released v1 and everyone is happy.

However, for v2, you realize that you would prefer `super_suit` to be
included in `equipment` instead. Good, but now, whenever your v2
program runs and tries to load a 'v1' config, your deserializer will
rightfully tell you: `Honey, where's my super_suit ?`

You probably know that the answer to this backward-compatibility issue
is to add a version field, save the old structure tree somewhere,
and create new ones

```rust
struct FrozoneV2 {
    version: String,
    power: Power,
    equipments: Vec<PartV2>
}

impl From<Frozone> for FrozoneV2 {
    fn from(value: Frozone) -> FrozoneV2 {
        // ..proper migration code
    }
}
```

Of course, you'll never forget to check that no similar changes during
the v1->v2 development have affected `Power`: a deeply-nested struct,
with enough optional fields to make coverage testing a full-on PhD thesis,
spanning multiple modules/crates/developer's responsibilities.

_OF COURSE, RIGHT ?_

`frozone` enforces your 'object tree' semantically never changes, ensuring
serialization and API stability.

## Usage

```rust
use frozone::Freezable;

#[derive(Serialize, Deserialize, Freezable)]
struct Frozone {
    power: Power,
    equipment: Vec<Part>,
    super_suit: Suit
}

#[derive(Serialize, Deserialize, Freezable)]
struct Power {
    // ...
}
// ...

fn main() {
    /// call the associated method `::freeze()` on your structure, and
    /// compare it to the fixed value it evaluated to
    assert_eq!(Frozone::freeze(),  12298013273002774775); // frozone hash from v1 release
    /// .. okay, `Frozone` has not changed since v1
}
```

## What's frozen and what's not

```rust
#[derive(Freezable)]
pub struct StructName {       // visibility qualifier, struct name: NOT FROZEN
    field_name: FieldType,    // field names, fields types: FROZEN

    #[assume_frozen]                              // assume_frozen'd field name: FROZEN
    assumed_frozen_field_name: ExternalFieldType, // assume_frozen'd field type: NOT FROZEN

    #[assume_frozen(freeze_generics)]
    test: Vec<FieldType>, // field name : FROZEN, container type: NOT FROZEN, contained type(s): FROZEN
}
// note: the order of the fields is "not frozen"


#[derive(Freezable)]
enum FieldType {  // enum name: NOT FROZEN
    UnitVariant,  // variant name: FROZEN
    Variant1 = 1, // discriminant value: FROZEN
    StructVariant(Type1, Type2, Type3), // inner types (and their order): FROZEN

    #[assume_frozen]  // assume_frozen'd field name: FROZEN
    AssumedFrozenVariant(Type4, Type5), // assume_frozen'd inner types: NOT FROZEN
}
// note: the order of the variants is "not frozen"
```

<details>

<summary>Note about 'type-recursiveness'</summary>

`frozone` supports enums & structs that are 'type-recursive', aka they embed
themselves (but with indirections, obv.), such as:

```rust
struct T1 {
    a: Box<Option<T1>>
}

// or

struct T2 {
    a: Box<T3>,
}
struct T3 {
    b: Option<T2>
}
```

In some more complex cases, such as:

```rust
struct Cycle2_1 {
    a: Box<Option<Cycle2_2>>
}
struct Cycle2_2 {
    a: Box<Option<Cycle2_1>>
}

struct Cycle3_1 {
    a: Box<Option<Cycle3_2>>
}
struct Cycle3_2 {
    a: Box<Option<Cycle3_3>>
}
struct Cycle3_3 {
    a: Box<Option<Cycle3_1>>
}
```

While in a serialized form it would be hard to distinguish `Cycle2_1` from `Cycle3_1`,
frozone understands that those are not equivalent semantically, and therefore they will
have different `freeze()` values


</details>


## Roadmap

- [x] structs support
- [x] enums support
- [x] core/alloc/std types support
- [x] assume_frozen attribute for external types (incl. Freezable generics support)
- [ ] configurable inclusion of the type names themselves
- [ ] compile-time check (probably requires const trait = nightly Rust)
- [ ] better errors
- [ ] consider #[repr(..)] changes inclusion in hash
- [ ] consideration for non-exhaustive enums
- [ ] common crates shims (uuid, url ...)
- [ ] pub-only fields feature?
