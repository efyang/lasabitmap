use std::iter;

use node::Node;
use coord::Coord;

pub struct Graph {
    data: Vec<Vec<Node>>,
    width: usize,
    height: usize,
    endpoints: Vec<Coord>,
    current_0_count: usize,
}

impl Graph {
    pub fn new(data: &mut Vec<Vec<Node>>) -> Graph {
        pad_graph_data(data, Node::new(true));
        Graph {
            data: data.to_owned(),
            width: data[0].len(),
            height: data.len(),
            endpoints: vec![Coord::new(1, 1)],
            current_0_count: 0,
        }
    }

    pub fn get_count(&self) -> usize {
        self.current_0_count
    }

    // Some(num of endpoints) or None (finished)
    pub fn iterate(&mut self) -> Option<usize> {
        let endpoint_amnt = self.endpoints.len();
        if endpoint_amnt == 0 {
            return None;
        } else {
            for i in (0..endpoint_amnt).rev() {
                let coord = self.endpoints.remove(i);
                self.handle_coord(coord);
            }
            return Some(endpoint_amnt);
        }
    }

    fn handle_coord(&mut self, coord: Coord) {
        let up = Coord::new(coord.x, coord.y + 1);
        let down = Coord::new(coord.x, coord.y - 1);
        let left = Coord::new(coord.x - 1, coord.y);
        let right = Coord::new(coord.x + 1, coord.y);
        let mut coords = vec![up, right];
        if coord.x != 0 {
            coords.push(left);
        } 
        if coord.y != 0 {
            coords.push(down);
        }
        for newcoord in coords.iter() {
            if newcoord.inbounds(self.width, self.height) {
                let ref mut node = self.data[newcoord];
                if !node.checked {
                    node.checked = true;
                    if node.value == false /*eq 0*/ {
                        self.current_0_count += 1;
                        self.endpoints.push(newcoord.to_owned());
                    }
                }
            }
        }
    }
}

fn pad_graph_data(graph: &mut Vec<Vec<Node>>, pad_value: Node) {
    let width = graph[0].len();
    let horizontal_row = iter::repeat(pad_value.clone()).take(width + 2).collect::<Vec<Node>>();
    for row in graph.iter_mut() {
        row.insert(0, pad_value.clone());
        row.push(pad_value.clone());
    }
    graph.insert(0, horizontal_row.clone());
    graph.push(horizontal_row);
}
