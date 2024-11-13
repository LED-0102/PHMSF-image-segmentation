pub struct DisjointSetUnion {
    parent: Vec<u32>,
    rank: Vec<u32>,
}

impl DisjointSetUnion {
    pub(crate) fn new (nodes: u32) -> DisjointSetUnion {
        let mut parent: Vec<u32> = vec![0; (nodes + 1) as usize];
        let mut rank: Vec<u32> = vec![0; (nodes + 1) as usize];
        for i in 0..nodes {
            parent.push(i);
            rank.push(0);
        }
        DisjointSetUnion {
            parent,
            rank,
        }
    }
}