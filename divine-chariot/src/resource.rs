////////////////////////////////////////////////////////////////////////////////

use std::any::{Any,TypeId};
use std::cell::{RefCell};
use std::collections::{BTreeMap,BTreeSet,HashMap,HashSet};
use std::fmt::{Debug};
use std::rc::{Rc};

use typename::{TypeName};

use crate::b64::*;
use crate::entity::{EntityId};

////////////////////////////////////////////////////////////////////////////////

pub trait Resource { }

////////////////////////////////////////

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct ResourceId(TypeId);

impl ResourceId {
    pub fn new<T:'static+Resource>() -> Self {
        ResourceId(TypeId::of::<T>())
    }
}

////////////////////////////////////////

#[derive(Debug)]
pub struct ResourceRegistry {
    names : HashMap<ResourceId, Box<str>>,
    data : HashMap<ResourceId, Box<dyn Any>>,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        let names = HashMap::new();
        let data = HashMap::new();
        ResourceRegistry { names, data }
    }

    pub fn register<T:'static+Resource+TypeName>(&mut self, t:T) -> Result<(),()>
    {
        let rid = ResourceId::new::<T>();
        let name = T::type_name();
        println!("ResourceRegistry::register({}, {:?})", name, rid);
        if self.has::<T>() { return Err(()); }
        self.names.insert(rid, name.into_boxed_str());
        self.data.insert(rid, Box::new(t));
        Ok(())
    }

    pub fn has<T:'static+Resource>(&self) -> bool
    {
        let rid = ResourceId::new::<T>();
        self.data.contains_key(&rid)
    }

    pub fn get<T:'static+Resource+Clone>(&self) -> Option<T> {
        let rid = ResourceId::new::<T>();
        self.data.get(&rid).and_then(|x|x.downcast_ref::<T>()).cloned()
    }

    pub fn get_ref<T:'static+Resource>(&self) -> Option<&T> {
        let rid = ResourceId::new::<T>();
        self.data.get(&rid).and_then(|x|x.downcast_ref::<T>())
    }

    pub fn get_mut<T:'static+Resource>(&mut self) -> Option<&mut T> {
        let rid = ResourceId::new::<T>();
        self.data.get_mut(&rid).and_then(|x|x.downcast_mut::<T>())
    }

    pub fn set<T:'static+Resource>(&mut self, t:T) -> Result<(),()> {
        if !self.has::<T>() { return Err(()); }
        let rid = ResourceId::new::<T>();
        self.data.insert(rid, Box::new(t));
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////
