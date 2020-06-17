use crate::value::Value;
use crate::valueref::{BasicBlockRef, FunctionRef, InstructionRef, ValueRef};

pub struct BasicBlock {
    val: Value,
    parent: Option<FunctionRef>,
    ins_list: Vec<InstructionRef>,
}

impl BasicBlock {
    pub fn new(val: Value) -> BasicBlock {
        BasicBlock {
            val,
            parent: None,
            ins_list: vec![],
        }
    }

    pub fn val(&self) -> &Value {
        &self.val
    }

    pub fn id(&self) -> BasicBlockRef {
        self.val.id().raw().into()
    }

    pub fn val_mut(&mut self) -> &mut Value {
        &mut self.val
    }

    pub fn ins(&self) -> &[InstructionRef] {
        &self.ins_list[..]
    }

    pub fn ins_mut(&mut self) -> &mut [InstructionRef] {
        &mut self.ins_list[..]
    }

    pub fn parent(&self) -> Option<FunctionRef> {
        self.parent
    }

    pub fn set_parent(&mut self, new_parent: Option<FunctionRef>) {
        self.parent = new_parent;
    }

    pub fn insert_begin(&mut self, ins: InstructionRef) {
        self.ins_list.insert(0, ins);
    }

    pub fn insert_end(&mut self, ins: InstructionRef) {
        self.ins_list.push(ins);
    }

    pub fn insert_before(&mut self, new_ins: InstructionRef, pos: InstructionRef) {
        self.ins_list.insert(self.ins_idx(pos).unwrap(), new_ins);
    }

    pub fn insert_after(&mut self, new_ins: InstructionRef, pos: InstructionRef) {
        self.ins_list
            .insert(self.ins_idx(pos).unwrap() + 1, new_ins);
    }

    pub fn erase(&mut self, ins: InstructionRef) {
        self.ins_list.remove(self.ins_idx(ins).unwrap());
    }

    fn ins_idx(&self, ins: InstructionRef) -> Option<usize> {
        self.ins_list.iter().position(|x| *x == ins)
    }
}

impl From<&BasicBlock> for BasicBlockRef {
    fn from(x: &BasicBlock) -> Self {
        x.id()
    }
}
