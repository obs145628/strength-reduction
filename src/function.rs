use crate::value::Value;
use crate::valueref::{ArgumentRef, BasicBlockRef, FunctionRef, InstructionRef, ValueRef};

pub struct Function {
    val: Value,
    args: Vec<ArgumentRef>,
    is_decl: bool,
    bbs_list: Vec<BasicBlockRef>,
}

impl Function {
    pub fn new(val: Value, args: &[ArgumentRef], is_decl: bool) -> Function {
        Function {
            val,
            args: args.to_vec(),
            is_decl,
            bbs_list: vec![],
        }
    }

    pub fn id(&self) -> FunctionRef {
        self.val.id().raw().into()
    }

    pub fn val(&self) -> &Value {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut Value {
        &mut self.val
    }

    pub fn args(&self) -> &[ArgumentRef] {
        &self.args[..]
    }

    pub fn is_decl(&self) -> bool {
        self.is_decl
    }

    pub fn bbs(&self) -> &[BasicBlockRef] {
        assert!(!self.is_decl);
        &self.bbs_list[..]
    }

    pub fn bbs_mut(&mut self) -> &mut [BasicBlockRef] {
        assert!(!self.is_decl);
        &mut self.bbs_list[..]
    }

    pub fn insert_begin(&mut self, bb: BasicBlockRef) {
        assert!(!self.is_decl);
        self.bbs_list.insert(0, bb);
    }

    pub fn insert_end(&mut self, bb: BasicBlockRef) {
        assert!(!self.is_decl);
        self.bbs_list.push(bb);
    }

    pub fn insert_before(&mut self, new_bb: BasicBlockRef, pos: BasicBlockRef) {
        assert!(!self.is_decl);
        self.bbs_list.insert(self.bb_idx(pos).unwrap(), new_bb);
    }

    pub fn insert_after(&mut self, new_bb: BasicBlockRef, pos: BasicBlockRef) {
        assert!(!self.is_decl);
        self.bbs_list.insert(self.bb_idx(pos).unwrap() + 1, new_bb);
    }

    pub fn erase(&mut self, bb: BasicBlockRef) {
        assert!(!self.is_decl);
        self.bbs_list.remove(self.bb_idx(bb).unwrap());
    }

    fn bb_idx(&self, bb: BasicBlockRef) -> Option<usize> {
        assert!(!self.is_decl);
        self.bbs_list.iter().position(|x| *x == bb)
    }
}

impl From<&Function> for FunctionRef {
    fn from(x: &Function) -> Self {
        x.id()
    }
}
