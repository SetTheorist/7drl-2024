////////////////////////////////////////////////////////////////////////////////

use crate::point2d::*;
use crate::rect2d::*;

////////////////////////////////////////////////////////////////////////////////

#[derive(Clone,Debug,serde::Serialize,serde::Deserialize)]
pub struct Grid<T:Clone> {
    cells: Vec<T>,
    span: Rect2d,
}

impl<T:Clone> Grid<T> {
    pub fn new(span:Rect2d, default:T) -> Self {
        let size = span.size();
        let count = (size.x * size.y) as usize;
        let cells = vec![default; count];
        Grid { cells, span }
    }

    pub fn get(&self, p:Point2d) -> Option<&T> {
        if let Some(i) = self.span.index(p) {
            Some(&self.cells[i])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, p:Point2d) -> Option<&mut T> {
        if let Some(i) = self.span.index(p) {
            Some(&mut self.cells[i])
        } else {
            None
        }
    }

    pub fn set(&mut self, p:Point2d, t:T) -> bool {
        if let Some(i) = self.span.index(p) {
            self.cells[i] = t;
            true
        } else {
            false
        }
    }

    pub fn contains(&self, p:Point2d) -> bool {
        self.span.contains(p)
    }

    pub fn span(&self) -> Rect2d {
        self.span
    }
}

impl<T:Clone+Copy> Grid<T> {
    pub fn copy_from(&mut self, g:&Grid<T>) {
        self.cells.copy_from_slice(&g.cells);
    }
}

////////////////////////////////////////////////////////////////////////////////
