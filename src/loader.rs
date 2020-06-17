use crate::checker;
use crate::context::Context;
use crate::gop;
use crate::isa::ISA;
use crate::valueref::{BasicBlockRef, FunctionRef, InstructionRef, ValueRef, ValueRefEnum};

use std::collections::HashMap;

struct CodeBuilder {
    act_fun: Option<FunctionRef>,
    act_bb: Option<BasicBlockRef>,
    funs_map: HashMap<String, FunctionRef>,
    vars_map: HashMap<String, ValueRef>,
    bbs_map: HashMap<String, BasicBlockRef>,
    ins_list: Vec<(InstructionRef, Vec<String>)>,
    mock_var: Option<ValueRef>,
}

impl CodeBuilder {
    pub fn new() -> CodeBuilder {
        CodeBuilder {
            act_fun: None,
            act_bb: None,
            funs_map: HashMap::new(),
            vars_map: HashMap::new(),
            bbs_map: HashMap::new(),
            ins_list: vec![],
            mock_var: None,
        }
    }

    pub fn run(&mut self, ctx: &mut Context, gmod: &gop::Module) {
        self.mock_var = Some(ctx.make_const("", 42).into());

        for decl in gmod.decls() {
            if let gop::DeclBody::Dir(d) = decl.body() {
                let args = d.args();
                if args[0] != "fun" {
                    panic!("Unknown directive");
                }

                if decl.label_defs().len() != 1 {
                    panic!("Missing function name");
                }

                self.handle_fun_dir(ctx, decl, d);
            } else if let gop::DeclBody::Ins(ins) = decl.body() {
                let label = if decl.label_defs().is_empty() {
                    None
                } else {
                    Some(&decl.label_defs()[0][..])
                };
                self.handle_ins(ctx, ins.args(), label);
            } else {
                unreachable!();
            }
        }

        self.finish_fun(ctx);

        /*
            if self.mock_var.unwrap().own(ctx).unwrap().users().len() != 0 {
                panic!("mock var still in use");
            }
            ctx.erase_const(self.mock_var.unwrap().raw().into());
        */
    }

    fn handle_fun_dir(&mut self, ctx: &mut Context, decl: &gop::Decl, d: &gop::Dir) {
        if self.act_fun.is_some() {
            self.finish_fun(ctx);
        }

        let args = d.args();

        self.vars_map.clear();
        self.bbs_map.clear();
        self.ins_list.clear();
        let fun_name = &decl.label_defs()[0];

        let args_count = args.len() - 2;
        let fun = ctx.make_fun(&fun_name, args_count, false);
        self.funs_map.insert(fun_name.to_string(), fun);
        let args_ids = fun.own(ctx).unwrap().args().to_vec();
        for (idx, arg) in args_ids.iter().enumerate() {
            let arg_name = &args[2 + idx];
            arg.own_mut(ctx).unwrap().val_mut().rename(&arg_name[1..]);
            self.vars_map
                .insert(arg_name[1..].to_string(), (*arg).into());
        }

        self.act_fun = Some(fun);
    }

    fn handle_ins(&mut self, ctx: &mut Context, args: &[String], label: Option<&str>) {
        let opname = &args[0][..];
        let infos = ISA::instance()
            .find_ins(opname)
            .expect("unknown instruction");
        let is_def = infos.is_def(args);
        let def_name = if is_def { &args[1][1..] } else { "" };
        let rest_args = if is_def { &args[2..] } else { &args[1..] };

        if self.act_bb.is_none() {
            let bb = ctx.make_bb(label.unwrap());
            self.act_bb = Some(bb);
            ctx.bb_insert_in(bb, self.act_fun.unwrap());
            self.bbs_map.insert(label.unwrap().to_string(), bb);
        }

        let vargs: Vec<ValueRef> = rest_args
            .iter()
            .map(|arg| self.handle_arg(ctx, &arg))
            .collect();

        let ins = ctx.make_ins(def_name, opname, is_def, &vargs[..]);
        ctx.ins_insert_in(ins, self.act_bb.unwrap());
        self.ins_list.push((ins, args.to_vec()));

        if is_def {
            self.vars_map.insert(def_name.to_string(), ins.into());
        }

        if infos.is_term(args) {
            self.act_bb = None
        }
    }

    fn handle_arg(&mut self, ctx: &mut Context, arg: &str) -> ValueRef {
        let f = arg.chars().next().unwrap();
        if f == '@' || f == '%' {
            //Register / Label, resolve later
            return self.mock_var.unwrap();
        }

        if f == '-' || (f >= '0' && f <= '9') {
            let v: i64 = arg.parse().expect("invalid number argument");
            return ctx.make_const("", v).into();
        }

        unreachable!();
    }

    fn finish_fun(&mut self, ctx: &mut Context) {
        let fun = self.act_fun.unwrap();
        if self.act_bb.is_some() {
            panic!("function must finish with a term instruction");
        }

        let mut ins_list = vec![];
        std::mem::swap(&mut ins_list, &mut self.ins_list);
        for ins in ins_list {
            self.resolve_ins(ctx, ins.0, &ins.1);
        }
    }

    fn resolve_ins(&mut self, ctx: &mut Context, ins: InstructionRef, args: &[String]) {
        let opname = &args[0];
        let infos = ISA::instance().find_ins(opname).unwrap();
        let is_def = infos.is_def(args);
        let mock_var = self.mock_var.unwrap();
        let args_start = if is_def { 1 } else { 0 };
        let args_count = args.len() - 1 - args_start;

        for i in 0..args_count {
            let old_val = ins.own(ctx).unwrap().val().ops()[i];
            if old_val != mock_var {
                continue;
            }
            let new_val = self.resolve_arg(ctx, args, i + args_start + 1);
            ctx.ins_set_op(ins, i, new_val);
        }
    }

    fn resolve_arg(&mut self, ctx: &mut Context, args: &[String], idx: usize) -> ValueRef {
        let opname = &args[0];
        let arg = &args[idx];
        let f = arg.chars().next().unwrap();

        if f == '%' {
            *self.vars_map.get(&arg[1..]).expect(&format!(
                "Use undefined register value {} at {:?}",
                &arg[1..],
                args
            ))
        } else if f == '@' {
            let is_fun = opname == "call";
            if is_fun {
                self.find_fun(ctx, &arg[1..]).into()
            } else {
                (*self
                    .bbs_map
                    .get(&arg[1..])
                    .expect("Use undefined basic block"))
                .into()
            }
        } else {
            panic!("bad arg: {:?} - {:?}", args, arg)
        }
    }

    // Find or insert a function name
    // @TODO: doesn't handle case where referencing a function defined later
    fn find_fun(&mut self, ctx: &mut Context, name: &str) -> FunctionRef {
        let it = self.funs_map.get(name);
        if it.is_some() {
            return *it.unwrap();
        }

        let fun = ctx.make_fun(name, 0, true);
        self.funs_map.insert(name.to_string(), fun);
        fun
    }
}

pub fn load_gop(ctx: &mut Context, gmod: &gop::Module) {
    CodeBuilder::new().run(ctx, gmod);
}

pub fn build_gop(ctx: &Context) -> gop::Module {
    let mut decls: Vec<gop::Decl> = vec![];

    for fun in ctx.funs() {
        let fun = fun.own(ctx).unwrap();
        if fun.is_decl() {
            continue;
        }

        let fun_name = fun.val().name().to_string();
        let mut args_names = fun
            .args()
            .iter()
            .map(|arg| "%".to_string() + arg.own(ctx).unwrap().val().name())
            .collect::<Vec<_>>();
        let mut dir_args = vec!["fun".to_string(), "int".to_string()];
        dir_args.append(&mut args_names);

        decls.push(gop::Decl::new_dir(
            vec![fun_name],
            vec![],
            String::new(),
            dir_args,
        ));

        for bb in fun.bbs() {
            let bb = bb.own(ctx).unwrap();
            let mut bb_label = Some(bb.val().name());

            for ins in bb.ins() {
                let ins = ins.own(ctx).unwrap();

                let mut ins_args = vec![ins.opname().to_string()];
                if ins.val().is_def() {
                    ins_args.push("%".to_string() + ins.val().name());
                }
                ins_args.append(
                    &mut ins
                        .val()
                        .ops()
                        .iter()
                        .map(|arg| val_to_gop_arg(ctx, *arg))
                        .collect::<Vec<String>>(),
                );

                let labels = match bb_label {
                    Some(l) => vec![l.to_string()],
                    None => vec![],
                };
                bb_label = None;

                decls.push(gop::Decl::new_ins(labels, vec![], String::new(), ins_args));
            }
        }
    }

    gop::Module::new(decls)
}

fn val_to_gop_arg(ctx: &Context, val: ValueRef) -> String {
    match val.to_enum() {
        ValueRefEnum::Ins(r) => "%".to_string() + r.own(ctx).unwrap().val().name(),
        ValueRefEnum::BB(r) => "@".to_string() + r.own(ctx).unwrap().val().name(),
        ValueRefEnum::Fun(r) => "@".to_string() + r.own(ctx).unwrap().val().name(),
        ValueRefEnum::Const(r) => r.own(ctx).unwrap().const_int().to_string(),
        ValueRefEnum::Arg(r) => "%".to_string() + r.own(ctx).unwrap().val().name(),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_path(path: &str) -> String {
        use std::path::Path;
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(path)
            .to_str()
            .unwrap()
            .to_string()
    }

    fn test_file(path: &str) {
        let path = find_path(path);

        let mut ctx = Context::new();
        load_gop(&mut ctx, &gop::Module::parse(&path));
        checker::check_code(&ctx);
        let gmod = build_gop(&ctx);
        println!("{}", gmod);
    }

    #[test]
    fn load_fact_iter() {
        test_file("examples/fact_iter.ir");
    }

    #[test]
    fn load_fact_rec() {
        test_file("examples/fact_rec.ir");
    }

    #[test]
    fn load_cycle1() {
        test_file("examples/cycle1.ir");
    }
}
