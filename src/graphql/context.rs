use std::collections::HashMap;

use parking_lot::RwLock;
use uuid::Uuid;

use super::{Human, NewHuman};

pub(crate) struct Context {
    _db: sqlx::PgPool,
    humans: RwLock<HashMap<Uuid, Human>>,
}

impl Context {
    pub(crate) fn new(db: sqlx::PgPool) -> Self {
        Self {
            _db: db,
            humans: Default::default(),
        }
    }

    pub(crate) fn humans(&self) -> Vec<Human> {
        self.humans.read().values().cloned().collect()
    }

    pub(crate) fn find_human(&self, id: &Uuid) -> Result<Human, &'static str> {
        self.humans.read().get(id).cloned().ok_or("not found")
    }

    pub(crate) fn insert_human(&self, new_human: NewHuman) -> Result<Human, &'static str> {
        let mut humans = self.humans.write();

        if humans
            .values()
            .any(|human| human.name() == new_human.name())
        {
            return Err("a human with that name already exists");
        }

        let human = Human::new(new_human);
        humans.insert(human.id(), human.clone());

        Ok(human)
    }
}

impl juniper::Context for Context {}
