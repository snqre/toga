# toga

Toga is a proc macro crate for cleaner and more ergonomic rust.

## Features

* **Single-Block Syntax**: Define inherent methods and multiple trait implementations for a type within one macro invocation.
* **Generics & Where Clauses**: Preserve type and trait generics, as well as `where` constraints, across all generated `impl`s.
* **Flexible Trait Paths**: Support for absolute, relative, and `self::`-qualified trait paths.
* **Reduced Boilerplate**: Eliminate repetitive `impl` declarations and unify method and trait definitions.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies.toga]
version = "*"
```

Then import the macro in your crate:

```rust
use toga::impls;
```

## Usage

```rust
// Define some traits
trait Health<T> {
    fn health(&self) -> T;
}

trait Wizard {
    fn you_shall_not_pass(&self);
}

// A generic struct with a const parameter
struct Player<const A: usize, B>(B);

// Use the `impls!` macro to generate implementations
impls! {
    impl<const A: usize, B> Player<A, B>
    where
        B: Clone;

    // Inherent methods
    pub fn hello_world(&self) {
        println!("Hello, world!");
    }

    pub fn give_me_a_number(&self) -> u8 {
        50
    }

    // Trait implementations
    Wizard {
        fn you_shall_not_pass(&self) {
            println!("You shall not pass!");
        }
    }

    Health<u8> {
        fn health(&self) -> u8 {
            100
        }
    }
}

fn main() {
    let player: Player<5, _> = Player("Data");
    player.hello_world();
    assert_eq!(player.give_me_a_number(), 50);
    player.you_shall_not_pass();
    assert_eq!(player.health(), 100);
}
```

## Macro Syntax

```text
impls! {
    impl<ImplGenerics> Type<TypeGenerics>
    where
        ...;

    // Inherent methods
    #[attr]
    fn method1(...) { ... }

    // Trait blocks
    path::ToTrait<TraitGenerics> {
        fn trait_method(...) -> ... { ... }
    }
}
```

* **Header**: Starts with `impl`, optional generics, and `where` clause.
* **Inherent Methods**: Any `fn` declarations before trait blocks.
* **Trait Blocks**: Blocks prefixed by a trait path are parsed as trait implementations.

## License

Apache-2.0