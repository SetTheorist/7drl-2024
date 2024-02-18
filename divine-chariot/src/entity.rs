////////////////////////////////////////////////////////////////////////////////

use std::collections::{HashSet};

use crate::handle::{Handle};

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,Hash,Ord,PartialEq,PartialOrd,serde::Serialize,serde::Deserialize)]
pub struct EntityId(u32);

impl EntityId {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
    pub fn null() -> Self {
        EntityId(0)
    }
}

impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#e{}", self.0)
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#e{}", self.0)
    }
}

impl From<EntityId> for usize {
    fn from(x:EntityId) -> usize {
        x.0 as usize
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct EntityManager {
    next_id: u32,
    active: HashSet<EntityId>,
}

impl EntityManager {
    pub fn new() -> Self {
        let next_id = 1;
        let active = HashSet::new();
        EntityManager { next_id, active }
    }

    pub fn new_id(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.active.insert(id);
        id
    }

    pub fn is_active(&self, e:EntityId) -> bool {
        self.active.contains(&e)
    }

    pub fn deactivate(&mut self, e:EntityId) {
        self.active.remove(&e);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub type EntityManagerHandle = Handle<EntityManager>;

impl EntityManagerHandle {
    pub fn new() -> Self {
        EntityManagerHandle::new_from(EntityManager::new())
    }

    pub fn new_id(&self) -> EntityId { self.borrow_mut().new_id() }
    pub fn is_active(&self, e:EntityId) -> bool { self.borrow().is_active(e) }
    pub fn deactivate(&self, e:EntityId) { self.borrow_mut().deactivate(e) }
}

////////////////////////////////////////////////////////////////////////////////
