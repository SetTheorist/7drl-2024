////////////////////////////////////////////////////////////////////////////////

use std::cell::{Ref, RefCell, RefMut};
//use std::ops::{Deref};
use std::rc::{Rc, Weak};

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug,Default)]
pub struct Handle<T>(Rc<RefCell<T>>) where T:?Sized;

impl<T> Clone for Handle<T> where T:?Sized {
    fn clone(&self) -> Self { Handle(self.0.clone()) }
}

impl<T> Handle<T> 
    where T:Sized
{
    pub fn new_from(t:T) -> Self {
        Handle(Rc::new(RefCell::new(t)))
    }
}

impl<T> Handle<T>
    where T:Default
{
    pub fn new() -> Self {
        Handle(Rc::new(RefCell::new(T::default())))
    }
}

impl<T> Handle<T> 
    where T:?Sized
{
    pub fn borrow(&self) -> Ref<T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<T> {
        self.0.borrow_mut()
    }

    pub fn weak(&self) -> WeakHandle<T> {
        WeakHandle(Rc::downgrade(&self.0))
    }

    pub fn ptr_eq(a:&Handle<T>, b:&Handle<T>) -> bool {
        Rc::ptr_eq(&a.0, &b.0)
    }
}

////////////////////////////////////////////////////////////////////////////////

/*
// TODO: doesn't work...
impl<T> Deref for Handle<T> {
    type Target = Ref<T>;
    
    fn deref(&self) -> &Ref<T> {
        self.borrow()
    }
}
*/

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Debug,Default)]
pub struct WeakHandle<T>(Weak<RefCell<T>>) where T:?Sized;

impl <T> WeakHandle<T> {
    pub fn new() -> Self {
        WeakHandle(Weak::new())
    }
}

impl<T> WeakHandle<T>
    where T:?Sized
{
    pub fn upgrade(&self) -> Option<Handle<T>> {
        self.0.upgrade().map(|rc|Handle(rc))
    }
}

////////////////////////////////////////////////////////////////////////////////
