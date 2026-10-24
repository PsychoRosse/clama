# Clama

A lightweight Entity Component System for Rust, designed to organize game logic into modular systems with explicit component access and flexible entity queries.

## Features

- **Entity commands** for spawning entities and managing their components.
- **Queries** to iterate over entities matching specific component requirements.
- **Access control** through `Read<T>` and `Write<T>`.
- **Query filters** to select entities based on component presence and change tracking.
- **Resources** for storing shared data accessible to systems.
- **Plugins** for organizing and registering systems.
- **Execution stages** for controlling when systems run.

## Queries

`Query<T>` provides access to entities and their components. Use `Read<T>` and `Write<T>` to specify whether a component is accessed immutably or mutably.

Available filters:

- `With<T>` selects entities that have component `T`.
- `Without<T>` selects entities that do not have component `T`.
- `Added<T>` selects entities with a component that was recently added.
- `Changed<T>` selects entities with a component that has changed.

Example:

```rust
use clama::{Query, Read, Write, With, Without};

fn movement_system(
    query: Query<(Write<Transform>, Read<Velocity>), With<Player>>,
) {
    for (mut transform, velocity) in query {
        transform.position += velocity.linear;
    }
}

fn npc_system(
    query: Query<Read<Transform>, Without<Player>>,
) {
    for transform in query {
        // Process entities without Player.
    }
}
```

## Commands

`Commands` provides an interface for managing entities from systems.

```rust
use clama::Commands;

fn setup_system(commands: Commands) {
    commands.spawn((
        Transform::default(),
        Player::default(),
    ));
}
```

## Resources

Resources provide shared data that systems can access without attaching it to individual entities.

- `Res<T>` provides read access to a resource.
- `ResMut<T>` provides mutable access to a resource.

```rust
use clama::{Res, ResMut};

fn game_system(
    time: Res<Time>,
    mut settings: ResMut<GameSettings>,
) {
    settings.elapsed += time.delta();
}
```

## Plugins and Systems

Implement `Plugin` to register resources and systems through `Registry`. Systems can be assigned to execution stages such as `Startup`, `Update`, and `LateUpdate`.

```rust
use clama::{Plugin, Registry, Stage};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, registry: &mut impl Registry) {
        registry
            .with_system(Stage::Startup, setup_system)
            .with_system(Stage::Update, update_system);
    }
}
```
