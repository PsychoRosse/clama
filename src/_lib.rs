mod ecs;
mod plugin;
mod scheduler;

pub mod prelude {
    pub use crate::{
        Bundle, Component, Entity, Event, EventBuffer, Plugin, Registry, Resource, Stage, World,
    };
}

pub use {clama_macros::*, ecs::api::*, ecs::error::*, plugin::api::*, scheduler::api::*};

pub(crate) use {ecs::internal::*, scheduler::internal::*};
