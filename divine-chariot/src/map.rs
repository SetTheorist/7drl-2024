////////////////////////////////////////////////////////////////////////////////

use std::cell::{RefCell};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash};
use std::rc::{Rc};

use crate::entity::{EntityId};
use crate::grid::{Grid};
use crate::point2d::*;
use crate::rect2d::*;

////////////////////////////////////////////////////////////////////////////////

/* 
 * Metamorphosis Alpha:
 *   Warden ship
 *     50 miles long x 25 miles wide x 8.5 miles high with 1/2 mile high dome
 *     330 feet think floors between levels (with some paths...)
 *     17 levels:
 *       1: (31 x 13 x 0.25) - supplies, storage (raw materials, rations, etc.)
 *       2: (34 x 15 x 0.125) - supplies, storage (finished parts, electronics, machines, etc.)
 *       3: (37 x 17 x 0.25) - supplies, storage (for factories in levels 4,5)
 *       4: (38 x 18.5 x 0.25) - mothballed factories for use on arrival; uninhabited wilderness forest; no humans, but wildlife
 *       5: (41 x 20 x 0.25) - more factories; mixed grasslands & forest areas; some small village settlements
 *       6: (42 x 20.5 x 0.125) - labs, generators; woodland, lakes & streams
 *       7: (45 x 21.5 x 0.125) - vast grasslands with ranches
 *       8: (47 x 22.5 x 0.5) - farmlands with rural farms & villages
 *       9: (48 x 23 x 0.5) - administrative & security facilities (& storage)
 *       10: (16 diam circle x 0.25) - control center / bridge, housing quarters, storage
 *       11: (49 x 24 x 0.5 or 0.75) - forest, hill in center, rough mountains at outer edge, some small primitive villages
 *       12: (48.5 x 23.5 x 0.5) - tropical jungle
 *       13: (47.5 x 22.5 x 0.25) - supplies for levels 11,12,14 (sim. to level 1)
 *       14: (46 x 22 x 0.25) - city level (~1.5 million people)
 *       15: (55 x 20.5 x 0.5) - water supply for ship, water ~ 0.25 mile deep; recycling & purification system; lots of fish, etc.
 *       16: (40 x 18.5 x 0.5) - activated factories (many city dwellers work here); chemical & elemental supplies; robot storage
 *       17: (40 x 18.5 x 1) - engines, motors, dynamos, reactors, power-producing devices; ion engines; anti-gravity (for landing); etc.
 *       observation dome: ( x x ) - small bubble on top; observation & sensing devices; astronomy lab
 *  
 *
 *
 * example: level 11:
 *   49 x 24 miles = 1176 miles^2
 *   = 258'720 x 126'720 feet = 32'784'998'400 feet^2
 *   = 52'744 x 25'344 (5') = 1'336'743'936 (5' square)
 *   = 824 x 396 (64 * 5') = 326'304 (64 * 64 * 5' square)
 *     (nb 64 * 64 = 4096)
 *   = 412 x 198 (128 * 5') = 81'576 (128 * 128 * 5' square)
 *     (nb 128 * 128 = 16384)
 *   = 206 x 99 (256 * 5') = 20'394 (256 * 256 * 5' square)
 *     (nb 256 * 256 = 65536)
 *
 *
 *
 *
 *
 *
 */

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Copy,Default,Eq,Hash,Ord,PartialEq,PartialOrd,serde::Serialize,serde::Deserialize)]
pub struct MapId(u32);

impl MapId {
    pub fn is_null(&self) -> bool { self.0==0 }
}

impl std::fmt::Debug for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#m{}", self.0)
    }
}

impl std::fmt::Display for MapId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "#m{}", self.0)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct MapManager<C:Clone> {
    next_id : u32,
    data : HashMap<MapId, Rc<RefCell<Map<C>>>>,
}

impl<C:Clone> MapManager<C> {
    pub fn new() -> Self {
        let next_id = 1;
        let data = HashMap::new();
        MapManager { next_id, data }
    }

    pub
    fn new_id(&mut self) -> MapId {
        let id = MapId(self.next_id);
        self.next_id += 1;
        id
    }

    pub fn get(&self, id:MapId) -> Option<Rc<RefCell<Map<C>>>> {
        self.data.get(&id).cloned()
    }

    pub fn add(&mut self, map:Map<C>) -> MapId {
        let id = self.new_id();
        self.data.insert(id.clone(), Rc::new(RefCell::new(map)));
        id
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Map<C:Clone> {
    name: String,
    cells: Grid<C>,
    entities: HashMap<EntityId,Point2d>,
    entities_at: HashMap<Point2d,Vec<EntityId>>,
    zones: HashMap<EntityId,Rect2d>,
}

impl<
    C:Clone,
> Map<C> {
    pub fn new(name:&str, span:Rect2d, default:C) -> Self {
        let name = name.to_string();
        let cells = Grid::new(span, default);
        let entities = HashMap::new();
        let entities_at = HashMap::new();
        let zones = HashMap::new();
        Map { name, cells, entities, entities_at, zones }
    }

    pub fn span(&self) -> Rect2d {
        self.cells.span()
    }

    pub fn cell(&self, p:Point2d) -> Option<&C> {
        self.cells.get(p)
    }

    pub fn cell_mut(&mut self, p:Point2d) -> Option<&mut C> {
        self.cells.get_mut(p)
    }

    pub fn set_cell(&mut self, p:Point2d, c:C) -> bool {
        self.cells.set(p, c)
    }

    pub fn zone_span(&self, e:EntityId) -> Option<Rect2d> {
        self.zones.get(&e).cloned()
    }

    pub fn zones_at(&self, p:Point2d) -> Option<Vec<EntityId>> {
        let v : Vec<_> =
            self.zones.iter()
                .filter(|(_,r)| r.contains(p))
                .map(|(e,_)| e)
                .cloned().collect();
        if v.len()==0 {None} else {Some(v)}
    }

    pub fn entity_position(&self, e:EntityId) -> Option<Point2d> {
        self.entities.get(&e).cloned()
    }

    pub fn entities_at(&self, p:Point2d) -> Option<&Vec<EntityId>> {
        self.entities_at.get(&p)
    }

    pub fn remove_entity(&mut self, e:EntityId) {
        if let Some(p) = self.entity_position(e) {
            self.entities.remove(&e);
            let widx = self.entities_at[&p].iter().position(|&f|f==e);
            if let Some(idx) = widx {
                self.entities_at.get_mut(&p).unwrap().remove(idx);
            }
        }
    }

    pub fn set_entity_position(&mut self, e:EntityId, p:Point2d) {
        self.remove_entity(e);
        self.entities.insert(e, p);
        self.entities_at.entry(p).or_insert_with(||vec![]).push(e);
    }

    pub fn entities_iter(&self) -> impl Iterator<Item=(&EntityId,&Point2d)> {
        self.entities.iter()
    }

    pub fn neighbors(&self, p:Point2d) -> impl Iterator<Item=Point2d> {
        let span = self.cells.span();
        Point2d::neighbors8(p).filter(move|q|span.contains(*q))
    }
}

////////////////////////////////////////////////////////////////////////////////
