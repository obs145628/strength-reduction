mod argument;
mod basicblock;
mod cfg;
mod checker;
mod constant;
mod context;
mod digraph;
mod digraph_order;
mod dom_tree;
mod function;
mod gop;
mod indexable;
mod instruction;
mod isa;
mod loader;
mod value;
mod valueref;
mod vertex_adapter;

#[macro_use]
extern crate lazy_static;

use crate::cfg::CFG;
use crate::context::Context;
use crate::dom_tree::DomTree;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fpath = args.get(1).expect("Missing file path");
    let mut ctx = Context::new();
    loader::load_gop(&mut ctx, &gop::Module::parse(&fpath));
    checker::check_code(&ctx);

    let gmod = loader::build_gop(&ctx);
    println!("{}", gmod);

    for fun in ctx.funs() {
        let fun = fun.own(&ctx).unwrap();
        if fun.is_decl() {
            continue;
        }

        let cfg = CFG::new(&ctx, fun.id());
        cfg.save_tree(&format!("./cfg_{}.dot", fun.val().name()));

        let dom = DomTree::new(&ctx, &cfg, fun.id());
        dom.save_tree(&format!("./dom_{}.dot", fun.val().name()));
    }
}
