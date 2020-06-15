use crate::value::Value;
use crate::valueref::{BasicBlockRef, ConstantRef, FunctionRef, InstructionRef, ValueRef};

pub struct Constant {
    val: Value,
    const_int: i64,
}

impl Constant {
    pub fn new(val: Value, const_int: i64) -> Constant {
        Constant { val, const_int }
    }

    pub fn id(&self) -> ConstantRef {
        self.val.id().raw().into()
    }

    pub fn val(&self) -> &Value {
        &self.val
    }

    pub fn val_mut(&mut self) -> &mut Value {
        &mut self.val
    }

    pub fn const_int(&self) -> i64 {
        self.const_int
    }
}

impl From<&Constant> for ConstantRef {
    fn from(x: &Constant) -> Self {
        x.id()
    }
}
