////////////////////////////////////////////////////////////////////////////////

use crate::point2d::*;
use crate::rect2d::*;
use crate::grid::*;
use crate::handle::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Window<Cell:Clone> {
    id: String,
    parent: WindowWeakHandle<Cell>,     // weak ref
    children: Vec<WindowHandle<Cell>>,  // strong refs
    offset: Point2d,                    // offset into parent local coordinates
    size: Point2d,                      // ie local coordinates
    span: Rect2d,                       // (0,0) -- (sizex,sizey)
    hidden: bool,                       // hides all subchildren also!
    default: Cell,                      // default value
    self_data: Grid<Cell>,              // self data, no children
    data: Grid<Cell>,                   // merged with children
    //dirty: Grid<bool>,                  // has anything changed in given cell
}

// TODO: track "dirty" flags to minimize copying (& screen updates)...

impl<Cell:Clone+Copy+Default+Eq> Window<Cell> {
    fn new(id:String, span:Rect2d) -> Self {
        let parent = WeakHandle::new();
        let children = Vec::new();
        let offset = Point2d::default();
        let size = span.size();
        let hidden = false;
        let default = Cell::default();
        let self_data = Grid::new(span, default);
        let data = Grid::new(span, default);
        //let dirty = Grid::new(span, false);
        Window { id, parent, children, offset, size, span, hidden, default, self_data, data
        //, dirty
        }
    }

    fn get_self_data(&self, p:Point2d) -> Option<Cell> {
        self.self_data.get(p).cloned()
    }

    fn get_data(&self, p:Point2d) -> Option<Cell> {
        self.data.get(p).cloned()
    }

    fn set_self_data(&mut self, p:Point2d, cell:Cell) -> bool {
        //if !self.span.contains(p) { return false; }
        //if cell != self.self_data.get(p).unwrap() {
            //self.dirty.set(p, true);
            self.self_data.set(p, cell)
        //}
    }

    fn update_data(&mut self) {
        self.data.copy_from(&self.self_data);
        for n in 0 .. self.children.len() {
            let mut ch = self.children[n].borrow_mut();
            if ch.hidden { continue; }
            ch.update_data();
            for p in ch.span {
                if let Some(chdata) = ch.get_data(p) {
                    let relp = ch.offset + (p - ch.span.bl);
                    self.data.set(relp, chdata);
                }
            }
        }
    }
}

impl<Cell:Clone> Drop for Window<Cell> {
    fn drop(&mut self) {
        print!("[Dropping Window '{}']", self.id);
    }
}

////////////////////////////////////////////////////////////////////////////////

pub type WindowWeakHandle<Cell> = WeakHandle<Window<Cell>>;

pub type WindowHandle<Cell> = Handle<Window<Cell>>;

impl<Cell:Clone+Copy+Default+::std::fmt::Debug+Eq> WindowHandle<Cell> {
    pub fn new(id:String, span:Rect2d) -> Self {
        WindowHandle::new_from(Window::new(id, span))
    }

    pub fn new_child(&self, id:String, offset:Point2d, span:Rect2d) -> WindowHandle<Cell> {
        let chw = WindowHandle::new(id, span);

        chw.borrow_mut().offset = offset;
        chw.borrow_mut().parent = self.weak();
        //chw.0.borrow_mut().z = self.0.borrow().children.len();
        self.borrow_mut().children.push(chw.clone());

        chw
    }

    pub fn get_offset(&self) -> Point2d {
        self.borrow().offset
    }

    pub fn set_offset(&self, offset:Point2d) {
        self.borrow_mut().offset = offset;
    }

    pub fn get_size(&self) -> Point2d {
        self.borrow().size
    }

    pub fn set_size(&self, size:Point2d) {
        let mut w = self.borrow_mut();
        if size == w.size { return; }
        let span = Rect2d::new(self.span().bl, self.span().bl+size);

        let mut self_data = Grid::new(span, Cell::default());
        let data = Grid::new(span, Cell::default());

        // TODO: blit
        for p in w.span {
            if let Some(cell) = w.get_self_data(p) {
                self_data.set(p, cell);
            } else {
                self_data.set(p, w.default.clone());
            }
        }
        w.self_data = self_data;
        w.data = data;
        w.span = span;
    }

    pub fn get_default(&self) -> Cell {
        self.borrow().default
    }

    pub fn set_default(&self, c:Cell) {
        self.borrow_mut().default = c;
    }

    pub fn get_hidden(&self) -> bool {
        self.borrow().hidden
    }

    pub fn set_hidden(&self, h:bool) {
        self.borrow_mut().hidden = h;
    }


    pub fn span(&self) -> Rect2d {
        self.borrow().span
    }

    pub fn fill(&self, cell:Cell) {
        let mut w = self.borrow_mut();
        for p in w.span {
            w.set_self_data(p, cell);
        }
    }

    pub fn fill_rect(&self, rect:Rect2d, cell:Cell) {
        let mut w = self.borrow_mut();
        for p in rect {
            w.set_self_data(p, cell);
        }
    }

    pub fn set(&self, p:Point2d, cell:Cell) -> bool {
        let mut w = self.borrow_mut();
        w.set_self_data(p, cell)
    }

    pub fn get(&self, p:Point2d) -> Option<Cell> {
        let w = self.borrow_mut();
        w.get_self_data(p)
    }

    pub fn data(&self, p:Point2d) -> Option<Cell> {
        let w = self.borrow_mut();
        w.get_data(p)
    }

    fn index_in_parent(&self) -> Option<usize> {
        let w = self.borrow();
        let par = w.parent.upgrade();
        if par.is_none() { return None; }
        let p = par.unwrap();
        let p = p.borrow();
        for i in 0..p.children.len() {
            if WindowHandle::ptr_eq(&p.children[i], &self) {
                return Some(i);
            }
        }
        None
    }

    pub fn lower(&self) {
        if let Some(idx) = self.index_in_parent() {
            let par = self.parent().unwrap();
            let mut par = par.borrow_mut();
            for oth in 0..idx {
                par.children.swap(oth, idx);
            }
        }
    }

    pub fn upper(&self) {
        if let Some(idx) = self.index_in_parent() {
            let par = self.parent().unwrap();
            let mut par = par.borrow_mut();
            let n = par.children.len();
            for oth in idx..(n-1) {
                par.children.swap(oth, oth+1);
            }
        }
    }

    pub fn parent(&self) -> Option<WindowHandle<Cell>> {
        self.borrow().parent.upgrade()
    }

    pub fn is_orphan(&self) -> bool {
        self.parent().is_none()
    }

    // delinks from parent, but keeps subtree intact
    pub fn orphan(&self) {
        if let Some(idx) = self.index_in_parent() {
            let p = self.parent().unwrap();
            let mut p = p.borrow_mut();
            p.children.remove(idx);
        }
    }

    // does not fixup parent links, just destroys the connects
    pub fn internal_demolish(&self) {
        let mut w = self.borrow_mut();
        w.parent = WeakHandle::new();
        w.children.iter().for_each(|ch|ch.internal_demolish());
        w.children.clear();
        w.update_data();
    }

    // delinks from parent, and destroys subtree
    pub fn demolish(&self) {
        self.orphan();
        self.internal_demolish();
    }

    pub fn id(&self) -> String {
        self.borrow().id.clone()
    }

    pub fn offset(&self) -> Point2d {
        self.borrow().offset
    }

    pub fn size(&self) -> Point2d {
        self.borrow().size
    }

    // which window/subwindow is under indicated point (relative to THIS window)
    // returns handle & point relativized to that window
    pub fn select(&self, p:Point2d) -> Option<(WindowHandle<Cell>,Point2d)> {
        let w = self.borrow();
        if w.span.contains(p) {
            for ch in w.children.iter().rev() {
                let o = ch.select(p - ch.borrow().offset);
                if !o.is_none() { return o; }
            }
            Some((self.clone(),p))
        } else {
            None
        }
    }

    pub fn update_data(&self) {
        self.borrow_mut().update_data();
    }

    pub fn dump(&self) {
        let w = self.borrow_mut();
        for y in 0..w.size.y {
            for x in 0..w.size.x {
                print!("{:?}", w.get_data(Point2d::new(x,y)).unwrap());
            }
            println!("");
        }
    }

    pub fn find_by_id(&self, id:&str) -> Option<WindowHandle<Cell>> {
        if self.id() == id { return Some(self.clone()); }
        for ch in self.borrow().children.iter() {
            let x = ch.find_by_id(id);
            if x.is_some() { return x; }
        }
        return None;
    }

    fn dump_tree_internal(&self, n:usize) {
        print!("{:1$}", "", n);
        println!("{} {} {} @ {}", self.id(), self.size(), self.span(), self.offset());
        let w = self.borrow();
        w.children.iter().for_each(|ch|ch.dump_tree_internal(n+2));
    }

    pub fn dump_tree(&self) {
        self.dump_tree_internal(0);
    }

    // is self an ancestor of w?
    // (search parents of w upward)
    pub fn ancestor(&self, w:&WindowHandle<Cell>) -> bool {
        let mut w = w.clone();
        loop {
            if Handle::ptr_eq(self, &w) {
                return true;
            }
            let par = w.parent();
            if let Some(wp) = par {
                w = wp;
            } else {
                return false;
            }
        }
        //unreachable!();
    }

    pub fn root(&self) -> WindowHandle<Cell> {
        let mut r = self.clone();
        while let Some(par) = r.parent() {
            r = par;
        }
        r
    }
}

////////////////////////////////////////////////////////////////////////////////
