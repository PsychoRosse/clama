mod ecs;
mod plugin;
mod scheduler;

pub mod prelude {
    pub use crate::{
        Bundle, Component, Entity, Event, EventBuffer, Plugin, Registry, Resource, Stage, World,
    };
}

pub use {ecs::api::*, ecs::error::*, macros::*, plugin::api::*, scheduler::api::*};

pub(crate) use {ecs::internal::*, scheduler::internal::*};
