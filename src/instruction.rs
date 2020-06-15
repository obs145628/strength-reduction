use crate::value::Value;
use crate::valueref::{BasicBlockRef, InstructionRef, ValueRef};

pub struct Instruction {
    val: Value,
    parent: Option<BasicBlockRef>,

    opname: String,
}

impl Instruction {
    pub fn new(val: Value, opname: &str) -> Instruction {
        Instruction {
            val,
            parent: None,
            opname: opname.to_string(),
        }
    }

    pub fn val(&self) -> &Value {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut Value {
        &mut self.val
    }

    pub fn opname(&self) -> &str {
        &self.opname
    }

    pub fn parent(&self) -> Option<BasicBlockRef> {
        self.parent
    }

    pub fn set_parent(&mut self, new_parent: Option<BasicBlockRef>) {
        self.parent = new_parent;
    }
}

impl From<&Instruction> for InstructionRef {
    fn from(x: &Instruction) -> Self {
        x.val.id().raw().into()
    }
}
