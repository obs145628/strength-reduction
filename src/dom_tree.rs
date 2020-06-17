use crate::cfg::CFG;
use crate::context::Context;
use crate::digraph::Digraph;
use crate::valueref::{BasicBlockRef, FunctionRef};
use crate::vertex_adapter::VertexAdapter;

pub struct DomTree {
    fun: FunctionRef,
    va: VertexAdapter<BasicBlockRef>,
    root: BasicBlockRef,

    idom: Vec<usize>,
    rpo: Vec<BasicBlockRef>,
    rpo_pos: Vec<usize>,
    tree: Digraph,
}

impl DomTree {
    pub fn new(ctx: &Context, cfg: &CFG, fun: FunctionRef) -> DomTree {
        let mut res = DomTree {
            fun,
            va: cfg.va().clone(),
            root: cfg.va().v2o(0),
            idom: vec![],
            rpo: vec![],
            rpo_pos: vec![],
            tree: Digraph::new(cfg.graph().v()),
        };
        res.build(ctx, cfg);
        res
    }

    pub fn root(&self) -> BasicBlockRef {
        self.root
    }

    pub fn idom(&self, bb: BasicBlockRef) -> BasicBlockRef {
        assert!(bb != self.root);
        self.va.v2o(self.idom[self.va.o2v(bb)])
    }

    pub fn dom(&self, bb: BasicBlockRef) -> Vec<BasicBlockRef> {
        let mut res = vec![];
        let mut node = bb;
        while node != self.root {
            res.push(node);
            node = self.idom(node);
        }
        res.push(node);
        res
    }

    pub fn succs<'a>(&'a self, bb: BasicBlockRef) -> impl Iterator<Item = BasicBlockRef> + 'a {
        self.tree
            .succs(self.va.o2v(bb))
            .map(move |v| self.va.v2o(v))
    }

    pub fn save_tree(&self, path: &str) {
        self.tree
            .save_tree(path)
            .expect("failed to write tree file");
    }

    fn build(&mut self, ctx: &Context, cfg: &CFG) {
        self.init(cfg);
        while !self.iterate(cfg) {}
        self.build_dom_tree(ctx);
    }

    fn init(&mut self, cfg: &CFG) {
        const UNDEF: usize = usize::MAX;

        self.rpo = cfg.rev_postorder();
        assert!(self.rpo[0] == self.root);
        self.rpo_pos = vec![0; self.rpo.len()];
        for (idx, bb) in self.rpo.iter().enumerate() {
            self.rpo_pos[self.va.o2v(*bb)] = idx;
        }

        self.idom = vec![UNDEF; self.rpo.len()];
        self.idom[self.va.o2v(self.root)] = self.va.o2v(self.root);
    }

    fn iterate(&mut self, cfg: &CFG) -> bool {
        const UNDEF: usize = usize::MAX;
        let mut changed = false;

        for bb in &self.rpo {
            if *bb == self.root {
                continue;
            }

            let mut new_idom = UNDEF;
            for pred in cfg.preds(*bb) {
                if self.idom[self.va.o2v(pred)] == UNDEF {
                    continue;
                }
                new_idom = if new_idom == UNDEF {
                    self.va.o2v(pred)
                } else {
                    self.intersect(self.va.o2v(pred), new_idom)
                };
            }
            assert!(new_idom != UNDEF);

            if self.idom[self.va.o2v(*bb)] != new_idom {
                self.idom[self.va.o2v(*bb)] = new_idom;
                changed = true;
            }
        }

        changed
    }

    fn intersect(&self, i: usize, j: usize) -> usize {
        let mut i = i;
        let mut j = j;
        while i != j {
            while self.rpo_pos[i] > self.rpo_pos[j] {
                i = self.idom[i];
            }
            while self.rpo_pos[j] > self.rpo_pos[i] {
                j = self.idom[j];
            }
        }
        i
    }

    fn build_dom_tree(&mut self, ctx: &Context) {
        for v in self.tree.vertices() {
            let bb = self.va.v2o(v);
            let bb_obj = bb.own(ctx).unwrap();
            self.tree.set_label_vertex_name(v, bb_obj.val().name());
            if bb != self.root {
                self.tree.add_edge(self.idom[v], v);
            }
        }
    }
}
