use crate::context::Context;
use crate::digraph::Digraph;
use crate::digraph_order;
use crate::valueref::{BasicBlockRef, FunctionRef, ValueRefEnum};
use crate::vertex_adapter::VertexAdapter;

pub struct CFG {
    fun: FunctionRef,
    va: VertexAdapter<BasicBlockRef>,
    g: Digraph,
}

impl CFG {
    pub fn new(ctx: &Context, fun: FunctionRef) -> CFG {
        let fun = fun.own(ctx).unwrap();
        let va = VertexAdapter::new(fun.bbs());
        let g = Digraph::new(va.count());

        let mut res = CFG {
            fun: fun.id(),
            va,
            g,
        };
        res.prepare(ctx);
        res
    }

    pub fn va(&self) -> &VertexAdapter<BasicBlockRef> {
        &self.va
    }

    pub fn graph(&self) -> &Digraph {
        &self.g
    }

    pub fn preds<'a>(&'a self, bb: BasicBlockRef) -> impl Iterator<Item = BasicBlockRef> + 'a {
        self.g.preds(self.va.o2v(bb)).map(move |v| self.va.v2o(v))
    }

    pub fn succs<'a>(&'a self, bb: BasicBlockRef) -> impl Iterator<Item = BasicBlockRef> + 'a {
        self.g.succs(self.va.o2v(bb)).map(move |v| self.va.v2o(v))
    }

    pub fn rev_postorder(&self) -> Vec<BasicBlockRef> {
        digraph_order::digraph_dfs(&self.g, digraph_order::DFSOrder::RevPost, 0, true)
            .iter()
            .map(|v| self.va.v2o(*v))
            .collect()
    }

    pub fn save_tree(&self, path: &str) {
        self.g.save_tree(path).expect("Failed to write tree file");
    }

    fn prepare(&mut self, ctx: &Context) {
        let fun = self.fun.own(ctx).unwrap();
        for bb in fun.bbs() {
            let bb = bb.own(ctx).unwrap();
            self.g
                .set_label_vertex_name(self.va.o2v(bb.id()), bb.val().name());
            let bins = bb.ins().iter().last().unwrap();
            let bins = bins.own(ctx).unwrap();
            for arg in bins.val().ops() {
                if let ValueRefEnum::BB(target) = arg.to_enum() {
                    self.g.add_edge(self.va.o2v(bb.id()), self.va.o2v(target));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gop;
    use crate::loader;

    fn find_path(path: &str) -> String {
        use std::path::Path;
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    }

    fn build_cfg(path: &str) -> CFG {
        let path = find_path(path);

        let mut ctx = Context::new();
        loader::load_gop(&mut ctx, &gop::Module::parse(&path));
        let fun = ctx.funs().next().unwrap();
        CFG::new(&ctx, fun)
    }

    #[test]
    fn cfg_fact_iter() {
        let cfg = build_cfg("examples/fact_iter.ir");
        let g = cfg.graph();
        assert!(!g.has_edge(0, 0));
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(0, 2));
        assert!(!g.has_edge(1, 0));
        assert!(g.has_edge(1, 1));
        assert!(g.has_edge(1, 2));
        assert!(!g.has_edge(2, 0));
        assert!(!g.has_edge(2, 1));
        assert!(!g.has_edge(2, 2));
    }

    #[test]
    fn load_fact_rec() {
        let cfg = build_cfg("examples/fact_rec.ir");
        let g = cfg.graph();
        assert!(!g.has_edge(0, 0));
        assert!(g.has_edge(0, 1));
        assert!(g.has_edge(0, 2));
        assert!(!g.has_edge(0, 3));
        assert!(!g.has_edge(1, 0));
        assert!(!g.has_edge(1, 1));
        assert!(!g.has_edge(1, 2));
        assert!(g.has_edge(1, 3));
        assert!(!g.has_edge(2, 0));
        assert!(!g.has_edge(2, 1));
        assert!(!g.has_edge(2, 2));
        assert!(g.has_edge(2, 3));
        assert!(!g.has_edge(3, 0));
        assert!(!g.has_edge(3, 1));
        assert!(!g.has_edge(3, 2));
        assert!(!g.has_edge(3, 3));
    }
}
