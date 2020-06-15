use crate::argument::Argument;
use crate::basicblock::BasicBlock;
use crate::constant::Constant;
use crate::function::Function;
use crate::indexable::Indexable;
use crate::instruction::Instruction;
use crate::value::Value;
use crate::valueref::{
    ArgumentRef, BasicBlockRef, ConstantRef, FunctionRef, InstructionRef, RawValueRef, SubValueRef,
    ValueRef,
};

pub struct Context {
    data_ins: Vec<Option<Box<Instruction>>>,
    data_bbs: Vec<Option<Box<BasicBlock>>>,
    data_args: Vec<Option<Box<Argument>>>,
    data_funs: Vec<Option<Box<Function>>>,
    data_consts: Vec<Option<Box<Constant>>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            data_ins: vec![],
            data_bbs: vec![],
            data_args: vec![],
            data_funs: vec![],
            data_consts: vec![],
        }
    }

    pub fn get_data_value(&self, r: ValueRef) -> Option<&Value> {
        let raw = RawValueRef::new(r.to_index());
        let (id, pos) = (raw.get_id(), raw.get_pos());
        match id {
            InstructionRef::ID => match self.data_ins.get(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val()),
                None => None,
            },
            BasicBlockRef::ID => match self.data_bbs.get(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val()),
                None => None,
            },
            ArgumentRef::ID => match self.data_args.get(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val()),
                None => None,
            },
            FunctionRef::ID => match self.data_funs.get(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val()),
                None => None,
            },
            ConstantRef::ID => match self.data_consts.get(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val()),
                None => None,
            },
            _ => unreachable!(),
        }
    }

    pub fn get_data_value_mut(&mut self, r: ValueRef) -> Option<&mut Value> {
        let raw = RawValueRef::new(r.to_index());
        let (id, pos) = (raw.get_id(), raw.get_pos());
        match id {
            InstructionRef::ID => match self.data_ins.get_mut(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val_mut()),
                None => None,
            },
            BasicBlockRef::ID => match self.data_bbs.get_mut(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val_mut()),
                None => None,
            },
            ArgumentRef::ID => match self.data_args.get_mut(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val_mut()),
                None => None,
            },
            FunctionRef::ID => match self.data_funs.get_mut(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val_mut()),
                None => None,
            },
            ConstantRef::ID => match self.data_consts.get_mut(pos).expect("invalid ref") {
                Some(obj) => Some(obj.val_mut()),
                None => None,
            },
            _ => unreachable!(),
        }
    }

    pub fn get_data_instruction(&self, r: InstructionRef) -> Option<&Instruction> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_ins.get(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_instruction_mut(&mut self, r: InstructionRef) -> Option<&mut Instruction> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_ins.get_mut(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_basic_block(&self, r: BasicBlockRef) -> Option<&BasicBlock> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_bbs.get(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_basic_block_mut(&mut self, r: BasicBlockRef) -> Option<&mut BasicBlock> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_bbs.get_mut(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_argument(&self, r: ArgumentRef) -> Option<&Argument> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_args.get(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_argument_mut(&mut self, r: ArgumentRef) -> Option<&mut Argument> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_args.get_mut(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_function(&self, r: FunctionRef) -> Option<&Function> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_funs.get(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_function_mut(&mut self, r: FunctionRef) -> Option<&mut Function> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_funs.get_mut(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_constant(&self, r: ConstantRef) -> Option<&Constant> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_consts.get(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    pub fn get_data_constant_mut(&mut self, r: ConstantRef) -> Option<&mut Constant> {
        let pos = RawValueRef::new(r.to_index()).get_pos();
        match self.data_consts.get_mut(pos).expect("invalid ref") {
            Some(obj) => Some(obj),
            None => None,
        }
    }

    fn make_value(&mut self, name: &str, id: ValueRef, is_def: bool, ops: &[ValueRef]) -> Value {
        let res = Value::new(name, id, is_def, ops);

        for op in res.ops() {
            if let Some(op) = op.own_mut(self) {
                op.users_add(id);
            }
        }

        res
    }

    pub fn make_ins(
        &mut self,
        name: &str,
        opname: &str,
        is_def: bool,
        ops: &[ValueRef],
    ) -> InstructionRef {
        let vref: ValueRef = RawValueRef::make(InstructionRef::ID, self.data_ins.len()).into();
        let iref: InstructionRef = vref.raw().into();
        let val = self.make_value(name, vref, is_def, ops);
        let ins = Instruction::new(val, opname);
        self.data_ins.push(Some(Box::new(ins)));
        iref
    }

    pub fn erase_ins(&mut self, ins: InstructionRef) {
        let pos = RawValueRef::new(ins.to_index()).get_pos();
        self.data_ins[pos] = None;
    }

    pub fn ins_set_op(&mut self, ins: InstructionRef, idx: usize, new_val: ValueRef) {
        let ins_val = ins.own_mut(self).unwrap().val_mut();
        let old_val = ins_val.ops()[idx];
        if old_val == new_val {
            return;
        }

        ins_val.ops_mut()[idx] = new_val;

        // Instruction may use same val for multiple operands
        if !ins_val.has_op(old_val) {
            if let Some(old_val) = old_val.own_mut(self) {
                old_val.users_del(ins.into())
            }
        }

        if let Some(new_val) = new_val.own_mut(self) {
            new_val.users_add(ins.into())
        }
    }

    pub fn ins_detach(&mut self, ins: InstructionRef) {
        let ins_obj = ins.own_mut(self).unwrap();
        if ins_obj.parent().is_none() {
            return;
        }

        let bb = ins_obj.parent().unwrap();
        ins_obj.set_parent(None);
        bb.own_mut(self).unwrap().erase(ins);
    }

    // Insert at end of basic block
    pub fn ins_insert_in(&mut self, ins: InstructionRef, bb: BasicBlockRef) {
        let ins_obj = ins.own_mut(self).unwrap();
        if ins_obj.parent().is_some() {
            panic!("Instruction already belongs to a basic block");
        }

        ins_obj.set_parent(Some(bb));
        bb.own_mut(self).unwrap().insert_end(ins);
    }

    pub fn ins_insert_before(&mut self, ins: InstructionRef, pos: InstructionRef) {
        let bb = pos.own(self).unwrap().parent().unwrap();
        let ins_obj = ins.own_mut(self).unwrap();
        if ins_obj.parent().is_some() {
            panic!("Instruction already belongs to a basic block");
        }

        ins_obj.set_parent(Some(bb));
        bb.own_mut(self).unwrap().insert_before(ins, pos);
    }

    pub fn ins_insert_after(&mut self, ins: InstructionRef, pos: InstructionRef) {
        let bb = pos.own(self).unwrap().parent().unwrap();
        let ins_obj = ins.own_mut(self).unwrap();
        if ins_obj.parent().is_some() {
            panic!("Instruction already belongs to a basic block");
        }

        ins_obj.set_parent(Some(bb));
        bb.own_mut(self).unwrap().insert_after(ins, pos);
    }

    pub fn make_bb(&mut self, name: &str) -> BasicBlockRef {
        let vref: ValueRef = RawValueRef::make(BasicBlockRef::ID, self.data_bbs.len()).into();
        let bref: BasicBlockRef = vref.raw().into();
        let val = self.make_value(name, vref, true, &[]);
        let bb = BasicBlock::new(val);
        self.data_bbs.push(Some(Box::new(bb)));
        bref
    }

    pub fn erase_bb(&mut self, bb: BasicBlockRef) {
        let pos = RawValueRef::new(bb.to_index()).get_pos();
        self.data_bbs[pos] = None;
    }

    pub fn bb_detach(&mut self, bb: BasicBlockRef) {
        let bb_obj = bb.own_mut(self).unwrap();
        if bb_obj.parent().is_none() {
            return;
        }

        let fun = bb_obj.parent().unwrap();
        bb_obj.set_parent(None);
        fun.own_mut(self).unwrap().erase(bb);
    }

    // Insert at end of function
    pub fn bb_insert_in(&mut self, bb: BasicBlockRef, fun: FunctionRef) {
        let bb_obj = bb.own_mut(self).unwrap();
        if bb_obj.parent().is_some() {
            panic!("Basic block already belongs to a function");
        }

        bb_obj.set_parent(Some(fun));
        fun.own_mut(self).unwrap().insert_end(bb);
    }

    pub fn bb_insert_before(&mut self, bb: BasicBlockRef, pos: BasicBlockRef) {
        let fun = pos.own(self).unwrap().parent().unwrap();
        let bb_obj = bb.own_mut(self).unwrap();
        if bb_obj.parent().is_some() {
            panic!("Basic block already belongs to a function");
        }

        bb_obj.set_parent(Some(fun));
        fun.own_mut(self).unwrap().insert_before(bb, pos);
    }

    pub fn bb_insert_after(&mut self, bb: BasicBlockRef, pos: BasicBlockRef) {
        let fun = pos.own(self).unwrap().parent().unwrap();
        let bb_obj = bb.own_mut(self).unwrap();
        if bb_obj.parent().is_some() {
            panic!("Basic block already belongs to a function");
        }

        bb_obj.set_parent(Some(fun));
        fun.own_mut(self).unwrap().insert_after(bb, pos);
    }

    fn make_arg(&mut self, name: &str, arg_pos: usize, fun: FunctionRef) -> ArgumentRef {
        let vref: ValueRef = RawValueRef::make(ArgumentRef::ID, self.data_args.len()).into();
        let aref: ArgumentRef = vref.raw().into();
        let val = self.make_value(name, vref, true, &[]);
        let arg = Argument::new(val, arg_pos, fun);
        self.data_args.push(Some(Box::new(arg)));
        aref
    }

    fn erase_arg(&mut self, arg: ArgumentRef) {
        let pos = RawValueRef::new(arg.to_index()).get_pos();
        self.data_args[pos] = None;
    }

    pub fn make_fun(&mut self, name: &str, args_count: usize, is_decl: bool) -> FunctionRef {
        let vref: ValueRef = RawValueRef::make(FunctionRef::ID, self.data_funs.len()).into();
        let fref: FunctionRef = vref.raw().into();
        let val = self.make_value(name, vref, true, &[]);

        let args: Vec<ArgumentRef> = (0..args_count)
            .map(|pos| self.make_arg("", pos, fref))
            .collect();

        let fun = Function::new(val, &args[..], is_decl);
        self.data_funs.push(Some(Box::new(fun)));
        fref
    }

    pub fn erase_fun(&mut self, fun: FunctionRef) {
        let pos = RawValueRef::new(fun.to_index()).get_pos();
        self.data_funs[pos] = None;
    }

    pub fn fun_get_arg_mut(&mut self, fun: FunctionRef, idx: usize) -> &mut Argument {
        let arg = fun.own(self).unwrap().args()[idx];
        arg.own_mut(self).unwrap()
    }

    pub fn funs<'a>(&'a self) -> impl Iterator<Item = FunctionRef> + 'a {
        self.data_funs
            .iter()
            .filter(|f| f.is_some())
            .map(|f| f.as_ref().unwrap().id())
    }

    pub fn make_const(&mut self, name: &str, const_int: i64) -> ConstantRef {
        let vref: ValueRef = RawValueRef::make(ConstantRef::ID, self.data_consts.len()).into();
        let cref: ConstantRef = vref.raw().into();
        let val = self.make_value(name, vref, true, &[]);
        let c = Constant::new(val, const_int);
        self.data_consts.push(Some(Box::new(c)));
        cref
    }

    pub fn erase_const(&mut self, c: ConstantRef) {
        let pos = RawValueRef::new(c.to_index()).get_pos();
        self.data_consts[pos] = None;
    }
}

#[test]
fn simple_function() {
    let mut ctx = Context::new();
    let my_num = ctx.make_const("", 671);
    let my_ret = ctx.make_ins("ret1", "ret", false, &[my_num.into()]);
    let my_bb = ctx.make_bb("my_bb");
    ctx.ins_insert_in(my_ret, my_bb);

    let my_fun = ctx.make_fun("foo", 2, false);
    ctx.fun_get_arg_mut(my_fun, 0).val_mut().rename("x");
    ctx.fun_get_arg_mut(my_fun, 1).val_mut().rename("y");
    ctx.bb_insert_in(my_bb, my_fun);

    assert_eq!(my_ret.own(&ctx).unwrap().val().name(), "ret1");
    assert_eq!(my_bb.own(&ctx).unwrap().val().name(), "my_bb");
    assert_eq!(
        my_ret
            .own(&ctx)
            .unwrap()
            .parent()
            .unwrap()
            .own(&ctx)
            .unwrap()
            .val()
            .name(),
        "my_bb"
    );

    assert_eq!(my_fun.own(&ctx).unwrap().val().name(), "foo");

    let funs = ctx.funs().collect::<Vec<_>>();
    for f in funs {
        assert_eq!(f.own(&ctx).unwrap().val().name(), "foo");
    }
}
