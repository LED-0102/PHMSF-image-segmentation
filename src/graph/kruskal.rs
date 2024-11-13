use crate::graph::dsu::DisjointSetUnion;
use crate::graph::graph::Graph;

pub struct Kruskal {
    edges: Vec<(f32, u32, u32)>,    // (u, v, w)
    dsu: DisjointSetUnion,
}

impl Kruskal {
    pub fn new (graph: &Graph) -> Kruskal {
        let mut edges: Vec<(f32, u32, u32)> = Vec::new();
        for u in 0..graph.nodes {
            for v in &graph.adj_list[u as usize] {
                edges.push((v.1, u, v.0));
            }
        }
        edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        Kruskal {
            edges,
            dsu: DisjointSetUnion::new(graph.nodes),
        }
    }
}