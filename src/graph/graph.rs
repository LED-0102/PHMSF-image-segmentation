use crate::graph::kruskal::Kruskal;

pub struct Graph {
    pub nodes: u32,                         // number of nodes
    pub adj_list: Vec<Vec<(u32, f32)>>,     // (node, weight)
}
impl Graph {
    pub fn new(nodes: u32) -> Graph {
        Graph {
            nodes,
            adj_list: vec![Vec::new(); (nodes+1) as usize],
        }
    }

    pub fn add_edge(&mut self, u: u32, v: u32, w: f32) {
        self.adj_list[u as usize].push((v, w));
        self.adj_list[v as usize].push((u, w));
    }
}

pub enum Algorithm {
    Kruskal (Kruskal)
}