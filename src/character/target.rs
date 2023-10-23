use crate::prelude::*;

#[derive(Component, Default)]
pub struct Target(Option<Entity>);

impl Target {
    pub fn id(&self) -> Option<Entity> {
        self.0
    }

    pub fn set(&mut self, id: Entity) {
        self.0 = Some(id);
    }

    pub fn clear(&mut self) {
        self.0 = None
    }
}
