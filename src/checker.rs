use crate::cfg::CFG;
use crate::context::Context;
use crate::dom_tree::DomTree;
use crate::isa::ISA;
use crate::valueref::{BasicBlockRef, FunctionRef, InstructionRef, ValueRef, ValueRefEnum};

use std::collections::HashSet;
use std::hash::Hash;

struct ScopedSet<T: Eq + Hash> {
    stack: Vec<HashSet<T>>,
}

impl<T> ScopedSet<T>
where
    T: Eq + Hash,
{
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn open(&mut self) {
        self.stack.push(HashSet::new());
    }

    fn close(&mut self) {
        self.stack.pop().expect("already empty");
    }

    fn contains(&self, val: T) -> bool {
        self.stack.last().expect("empty stack");
        for st in self.stack.iter().rev() {
            if st.contains(&val) {
                return true;
            }
        }

        false
    }

    fn put(&mut self, val: T) {
        let last = self.stack.last_mut().expect("empty stack");
        last.insert(val);
    }
}

impl<T> Drop for ScopedSet<T>
where
    T: Eq + Hash,
{
    fn drop(&mut self) {
        assert!(self.stack.is_empty())
    }
}

struct Checker {
    funs: HashSet<FunctionRef>,
    bbs: HashSet<BasicBlockRef>,
    vals: Option<ScopedSet<InstructionRef>>,
    cfg: Option<CFG>,
    dom: Option<DomTree>,
}

impl Checker {
    fn new() -> Checker {
        Checker {
            funs: HashSet::new(),
            bbs: HashSet::new(),
            vals: None,
            cfg: None,
            dom: None,
        }
    }

    fn run(&mut self, ctx: &Context) {
        self.init(ctx);
        for fun in ctx.funs() {
            let fun = fun.own(ctx).unwrap();
            if fun.is_decl() {
                continue;
            }

            self.init_fun(ctx, fun.id());
            self.check_fun(ctx, fun.id());
        }
    }

    fn init(&mut self, ctx: &Context) {
        self.funs = ctx.funs().collect();
    }

    fn init_fun(&mut self, ctx: &Context, fun: FunctionRef) {
        let fun = fun.own(ctx).unwrap();
        self.bbs = fun.bbs().iter().map(|x| *x).collect();
    }

    fn check_fun(&mut self, ctx: &Context, fun: FunctionRef) {
        let fun = fun.own(ctx).unwrap();
        if self.bbs.is_empty() {
            panic!("Empty function {}", fun.val().name());
        }

        self.vals = Some(ScopedSet::new());
        self.cfg = Some(CFG::new(ctx, fun.id()));
        self.dom = Some(DomTree::new(ctx, self.cfg.as_ref().unwrap(), fun.id()));
        self.check_bb(ctx, self.dom.as_ref().unwrap().root());
        self.dom = None;
        self.cfg = None;
        self.vals = None;
    }

    fn check_bb(&mut self, ctx: &Context, bb: BasicBlockRef) {
        self.vals.as_mut().unwrap().open();
        let bb = bb.own(ctx).unwrap();
        let dom_succs: Vec<BasicBlockRef> = self.dom.as_ref().unwrap().succs(bb.id()).collect();
        let cfg_succs: Vec<BasicBlockRef> = self.cfg.as_ref().unwrap().succs(bb.id()).collect();
        if bb.ins().is_empty() {
            panic!("Empty basic block {}", bb.val().name());
        }

        for ins in bb.ins() {
            self.check_ins(ctx, *ins);
        }

        self.check_term(ctx, bb.id());

        for succ in cfg_succs {
            self.check_phis(ctx, bb.id(), succ);
        }

        for succ in dom_succs {
            self.check_bb(ctx, succ);
        }

        self.vals.as_mut().unwrap().close();
    }

    fn check_ins(&mut self, ctx: &Context, ins: InstructionRef) {
        let ins = ins.own(ctx).unwrap();
        let vals = self.vals.as_mut().unwrap();

        if ins.opname() != "phi" {
            //Phi operands are tested in CFG preds
            for op in ins.val().ops() {
                if let ValueRefEnum::Ins(op_use) = op.to_enum() {
                    if !vals.contains(op_use) {
                        panic!(
                            "Use before def of operand {}",
                            op_use.own(ctx).unwrap().val().name()
                        );
                    }
                }
            }
        }

        if ins.val().is_def() {
            vals.put(ins.id());
        }
    }

    fn check_phis(&mut self, ctx: &Context, parent: BasicBlockRef, bb: BasicBlockRef) {
        let bb = bb.own(ctx).unwrap();
        let vals = self.vals.as_ref().unwrap();
        let parent_val: ValueRef = parent.into();

        for ins in bb.ins() {
            let ins = ins.own(ctx).unwrap();
            if ins.opname() != "phi" {
                break;
            }

            let ops = ins.val().ops();
            let mut op_pos = None;
            for (idx, op) in ops.iter().enumerate() {
                if *op == parent_val {
                    op_pos = Some(idx);
                }
            }
            let op_pos = op_pos.expect(&format!(
                "Phi predecesor value for {} is missing in {}",
                parent_val.own(ctx).unwrap().name(),
                bb.val().name()
            ));

            if let ValueRefEnum::Ins(op_use) = ops[op_pos + 1].to_enum() {
                if !vals.contains(op_use) {
                    panic!(
                        "Use before def in phi of operand {}",
                        op_use.own(ctx).unwrap().val().name()
                    );
                }
            }
        }
    }

    fn check_term(&mut self, ctx: &Context, bb: BasicBlockRef) {
        let bb = bb.own(ctx).unwrap();
        let bins = bb.ins().last().unwrap();
        let bins = bins.own(ctx).unwrap();
        let bins_infos = ISA::instance()
            .find_ins(bins.opname())
            .expect("Unknown last instruction of basic block");
        if !bins_infos.is_term(&[]) {
            panic!(
                "Last instruction of basic block {} is not a terminal",
                bb.val().name()
            );
        }

        for succ in bins.targets_bbs() {
            if !self.bbs.contains(&succ) {
                panic!(
                    "terminator in basic block {} branch to foreign block {}",
                    bb.val().name(),
                    succ.own(ctx).unwrap().val().name()
                );
            }
        }
    }
}

pub fn check_code(ctx: &Context) {
    Checker::new().run(ctx)
}
