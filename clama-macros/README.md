# clama-macros

Procedural macros for the [Clama](https://github.com/PsychoRosse/clama) Entity Component System (ECS) library.

`clama-macros` provides derive macros that implement Clama's core traits for your types.

## Features

- `#[derive(Component)]` implements the `clama::Component` trait.
- `#[derive(Resource)]` implements the `clama::Resource` trait.
- `#[derive(Event)]` implements the `clama::Event` trait.

## Installation

Add `clama` to your `Cargo.toml`:

```toml
[dependencies]
clama = "0.1"
```

The derive macros are provided by `clama-macros` and can be re-exported by the main `clama` crate.

If your version of `clama` does not re-export these macros, add the macro crate directly:

```toml
[dependencies]
clama = "0.1"
clama-macros = "0.1"
```

Use versions compatible with your project.

## Usage

Import the derive macros and use them on your types:

```rust
use clama::{Component, Event, Resource};

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Resource)]
struct GameTime {
    elapsed: f32,
}

#[derive(Event)]
struct PlayerDied {
    entity_id: u64,
}
```

Each derive macro generates an implementation of the corresponding Clama trait.

For example, `#[derive(Component)]` generates an implementation equivalent to:

```rust
impl clama::Component for Position {}
```

The generated implementation uses the path to the `clama` crate as resolved from your Cargo dependencies.

## Requirements

- Rust with procedural macro support.
- The `clama` ECS library.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
