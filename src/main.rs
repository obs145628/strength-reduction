mod argument;
mod basicblock;
mod constant;
mod context;
mod function;
mod gop;
mod indexable;
mod instruction;
mod isa;
mod loader;
mod value;
mod valueref;

#[macro_use]
extern crate lazy_static;

use crate::context::Context;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fpath = args.get(1).expect("Missing file path");
    let mut ctx = Context::new();
    loader::load_gop(&mut ctx, &gop::Module::parse(&fpath));

    let gmod = loader::build_gop(&ctx);
    println!("{}", gmod);
}
