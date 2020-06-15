use crate::value::Value;
use crate::valueref::{ArgumentRef, BasicBlockRef, FunctionRef, InstructionRef, ValueRef};

pub struct Argument {
    val: Value,
    arg_pos: usize,
    fun: FunctionRef,
}

impl Argument {
    pub fn new(val: Value, arg_pos: usize, fun: FunctionRef) -> Argument {
        Argument { val, arg_pos, fun }
    }

    pub fn val(&self) -> &Value {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut Value {
        &mut self.val
    }

    pub fn arg_pos(&self) -> usize {
        self.arg_pos
    }

    pub fn fun(&self) -> FunctionRef {
        self.fun
    }
}

impl From<&Argument> for ArgumentRef {
    fn from(x: &Argument) -> Self {
        x.val.id().raw().into()
    }
}
