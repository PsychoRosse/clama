# ECS

A lightweight Entity Component System (ECS) for Rust, designed to organize game logic into reusable systems and plugins.

## Features

- Entity spawning through `Commands`
- Component queries with `Query<T>`
- Read and write access via `Read<T>` and `Write<T>`
- Shared resources via `Res<T>`
- Plugin-based system registration
- Query filters: `With<T>`, `Added<T>`, and `Changed<T>`

## Usage

```rust
use kodanu::ecs::{Commands, Plugin, Registry, Stage};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry.with_system(Stage::Startup, setup_system);
    }
}

fn setup_system(commands: Commands) {
    commands.spawn((
        Transform::default(),
        PlayerState::default(),
    ));
}
```

Systems can query entities, access components, and work with shared resources.
