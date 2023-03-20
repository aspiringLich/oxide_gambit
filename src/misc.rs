use std::borrow::Cow;

use bevy::{ecs::system::EntityCommands, prelude::Name};

pub trait EntityNamer<'w, 's, 'a> {
    fn name(&'a mut self, name: &str) -> &'a mut EntityCommands<'w, 's, 'a>;
}

impl<'w, 's, 'a> EntityNamer<'w, 's, 'a> for EntityCommands<'w, 's, 'a> {
    fn name(&'a mut self, name: &str) -> &'a mut EntityCommands<'w, 's, 'a> {
        let e = self.id();
        self.insert(Name::new(format!("{} ({:#?})", name, e)));
        self
    }
}
