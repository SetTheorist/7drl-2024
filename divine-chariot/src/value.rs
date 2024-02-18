////////////////////////////////////////////////////////////////////////////////

use std::cmp::Ordering;
use std::collections::{HashMap};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Shl, Shr, Sub, SubAssign};

use crate::entity::{EntityId};
use crate::location::*;
use crate::map::{MapId};
use crate::point2d::*;
use crate::rect2d::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Debug,PartialEq,serde::Serialize,serde::Deserialize)]
pub enum Value {
    B(bool),
    C(char),
    D(f64),
    E(EntityId),
    I(isize),
    L(Location),
    M(HashMap<String,Value>),
    O(Option<Box<Value>>),
    P(Point2d),
    R(Rect2d),
    S(String),
    V(Vec<Value>),
    U,
}

impl Value {
    pub fn as_b_mut(&mut self) -> Option<&mut bool> { match self { Value::B(x) => Some(x), _ => None, } }
    pub fn as_b(&self) -> Option<bool> { match self { Value::B(x) => Some(*x), _ => None, } }
    pub fn as_c_mut(&mut self) -> Option<&mut char> { match self { Value::C(x) => Some(x), _ => None, } }
    pub fn as_c(&self) -> Option<char> { match self { Value::C(x) => Some(*x), _ => None, } }
    pub fn as_d_mut(&mut self) -> Option<&mut f64> { match self { Value::D(x) => Some(x), _ => None, } }
    pub fn as_d(&self) -> Option<f64> { match self { Value::D(x) => Some(*x), _ => None, } }
    pub fn as_e_mut(&mut self) -> Option<&mut EntityId> { match self { Value::E(x) => Some(x), _ => None, } }
    pub fn as_e(&self) -> Option<EntityId> { match self { Value::E(x) => Some(*x), _ => None, } }
    pub fn as_i_mut(&mut self) -> Option<&mut isize> { match self { Value::I(x) => Some(x), _ => None, } }
    pub fn as_i(&self) -> Option<isize> { match self { Value::I(x) => Some(*x), _ => None, } }
    pub fn as_l_mut(&mut self) -> Option<&mut Location> { match self { Value::L(x) => Some(x), _ => None, } }
    pub fn as_l(&self) -> Option<Location> { match self { Value::L(x) => Some(*x), _ => None, } }
    pub fn as_m_mut(&mut self) -> Option<&mut HashMap<String,Value>> { match self { Value::M(x) => Some(x), _ => None, } }
    pub fn as_m(&self) -> Option<&HashMap<String,Value>> { match self { Value::M(x) => Some(x), _ => None, } }
    pub fn as_o_mut(&mut self) -> Option<&mut Option<Box<Value>>> { match self { Value::O(x) => Some(x), _ => None, } }
    pub fn as_o(&self) -> Option<&Option<Box<Value>>> { match self { Value::O(x) => Some(x), _ => None, } }
    pub fn as_p_mut(&mut self) -> Option<&mut Point2d> { match self { Value::P(x) => Some(x), _ => None, } }
    pub fn as_p(&self) -> Option<Point2d> { match self { Value::P(x) => Some(*x), _ => None, } }
    pub fn as_r_mut(&mut self) -> Option<&mut Rect2d> { match self { Value::R(x) => Some(x), _ => None, } }
    pub fn as_r(&self) -> Option<Rect2d> { match self { Value::R(x) => Some(*x), _ => None, } }
    pub fn as_s_mut(&mut self) -> Option<&mut String> { match self { Value::S(x) => Some(x), _ => None, } }
    pub fn as_s(&self) -> Option<&String> { match self { Value::S(x) => Some(x), _ => None, } }
    pub fn as_v_mut(&mut self) -> Option<&mut Vec<Value>> { match self { Value::V(x) => Some(x), _ => None, } }
    pub fn as_v(&self) -> Option<&Vec<Value>> { match self { Value::V(x) => Some(x), _ => None, } }
    pub fn as_u(&self) -> Option<()> { match self { Value::U => Some(()), _ => None, } }
}

impl<'a> From<&'a str> for Value { fn from(x:&str)       -> Self { Value::S(x.to_string()) } }
impl<'a> From<&'a[Value]> for Value { fn from(x:&'a[Value]) -> Self { Value::V(x.to_vec()) } }
//impl<'a> From<&'a Value> for Value { fn from(x:&Value)      -> Self { *x } }
impl From<bool>       for Value { fn from(x:bool)       -> Self { Value::B(x) } }
impl From<char>       for Value { fn from(x:char)       -> Self { Value::C(x) } }
impl From<EntityId>   for Value { fn from(x:EntityId)   -> Self { Value::E(x) } }
impl From<f64>        for Value { fn from(x:f64)        -> Self { Value::D(x) } }
impl From<()>         for Value { fn from(_x:())         -> Self { Value::U } }
impl From<HashMap<String,Value>> for Value { fn from(x:HashMap<String,Value>) -> Self { Value::M(x) } }
impl From<i32>        for Value { fn from(x:i32)        -> Self { Value::I(x as isize) } }
impl From<isize>      for Value { fn from(x:isize)      -> Self { Value::I(x) } }
impl From<Location>   for Value { fn from(x:Location)   -> Self { Value::L(x) } }
impl From<Option<Box<Value>>>      for Value { fn from(x:Option<Box<Value>>)      -> Self { Value::O(x) } }
//impl From<Option<Value>>      for Value { fn from(x:Option<Value>)      -> Self { Value::O(x.map(|v|Box::new(v))) } }
impl From<Point2d>    for Value { fn from(x:Point2d)    -> Self { Value::P(x) } }
impl From<Rect2d>     for Value { fn from(x:Rect2d)     -> Self { Value::R(x) } }
impl From<String>     for Value { fn from(x:String)     -> Self { Value::S(x) } }
impl From<Vec<Value>> for Value { fn from(x:Vec<Value>) -> Self { Value::V(x) } }

#[macro_export]
macro_rules! value_map {
    ( ) => {
        std::collections::HashMap::<::String,::value::Value>::new()
    };
    ( $($k:expr => $v:expr),+ ) => {
        {
            let mut m = std::collections::HashMap::<::String,::value::Value>::new();
            $( m.insert(::String::from($k), ::value::Value::from($v)); )*
            m
        }
    };
}

////////////////////////////////////////////////////////////////////////////////
