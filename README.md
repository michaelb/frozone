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


## Roadmap

- [x] structs support
- [ ] clearly define what's included in the hash (struct name?)
- [ ] enums support
- [ ] compile-time check
