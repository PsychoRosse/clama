use crate::Resource;

use {
    std::any::{Any, TypeId},
    std::collections::HashMap,
};

#[derive(Default)]
pub struct ResourceRegistry {
    resources: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl ResourceRegistry {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            resources: HashMap::with_capacity(capacity),
        }
    }
}

impl ResourceRegistry {
    pub fn contains<R: Resource>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<R>())
    }

    pub fn push<R: Resource>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    pub fn remove<R: Resource>(&mut self) -> Option<R> {
        self.resources
            .remove(&TypeId::of::<R>())
            .and_then(|res| res.downcast::<R>().ok())
            .map(|res| *res)
    }

    pub fn get<R: Resource>(&self) -> Option<&R> {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|resource| resource.downcast_ref::<R>())
    }

    pub fn get_mut<R: Resource>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|res| res.downcast_mut::<R>())
    }
}
