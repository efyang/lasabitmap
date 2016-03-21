use std::ops::{Index, IndexMut};
use node::Node;

#[derive(Clone, Debug)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y,
        }
    }

    pub fn inbounds(&self, xmax: usize, ymax: usize) -> bool {
        (self.x < xmax) && (self.y < ymax)
    }
}

impl<'b> Index<&'b Coord> for Vec<Vec<Node>> {
    type Output = Node;
    fn index<'a>(&'a self, coord: &Coord) -> &'a Node {
        self.get(coord.y).unwrap().get(coord.x).unwrap()
    }
}

impl<'b> IndexMut<&'b Coord> for Vec<Vec<Node>> {
    fn index_mut<'a>(&'a mut self, coord: &Coord) -> &'a mut Node {
        self.get_mut(coord.y).unwrap().get_mut(coord.x).unwrap()
    }
}
