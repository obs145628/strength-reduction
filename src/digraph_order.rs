use crate::digraph::Digraph;

#[derive(PartialEq, Eq)]
pub enum DFSOrder {
    Pre,
    Post,
    RevPost,
}

struct DFS {
    order: DFSOrder,
    start: usize,
    visit_unreachable: bool,
    marked: Vec<bool>,
    res: Vec<usize>,
}

impl DFS {
    fn new(g: &Digraph, order: DFSOrder, start: usize, visit_unreachable: bool) -> DFS {
        DFS {
            order,
            start,
            visit_unreachable,
            marked: vec![false; g.v()],
            res: vec![],
        }
    }

    fn run(&mut self, g: &Digraph) {
        self.dfs(g, self.start);

        if (self.visit_unreachable) {
            for u in g.vertices() {
                if !self.marked[u] {
                    self.dfs(g, u);
                }
            }
        }

        assert!(!self.visit_unreachable || self.res.len() == g.v());

        if self.order == DFSOrder::RevPost {
            self.res.reverse()
        }
    }

    fn dfs(&mut self, g: &Digraph, u: usize) {
        self.marked[u] = true;

        if self.order == DFSOrder::Pre {
            self.res.push(u);
        }

        for u in g.succs(u) {
            if !self.marked[u] {
                self.dfs(g, u);
            }
        }

        if self.order == DFSOrder::Post || self.order == DFSOrder::RevPost {
            self.res.push(u);
        }
    }
}

pub fn digraph_dfs(
    g: &Digraph,
    order: DFSOrder,
    start: usize,
    visit_unreachable: bool,
) -> Vec<usize> {
    let mut dfs = DFS::new(g, order, start, visit_unreachable);
    dfs.run(g);
    dfs.res
}
