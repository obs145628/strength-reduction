use crate::argument::Argument;
use crate::basicblock::BasicBlock;
use crate::constant::Constant;
use crate::context::Context;
use crate::function::Function;
use crate::indexable::Indexable;
use crate::instruction::Instruction;
use crate::value::Value;

use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct RawValueRef(usize);

impl Indexable for RawValueRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl RawValueRef {
    pub fn new(x: usize) -> RawValueRef {
        RawValueRef(x)
    }

    pub fn get_id(&self) -> usize {
        (self.to_index() & 0xFFFF0000) >> 16
    }

    pub fn get_pos(&self) -> usize {
        self.to_index() & 0xFFFF
    }

    pub fn make(id: usize, pos: usize) -> RawValueRef {
        RawValueRef::new((id << 16) | pos)
    }
}

impl fmt::Debug for RawValueRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "RawValueRef({}, id={}, pos={})",
            self.0,
            self.get_id(),
            self.get_pos()
        )
    }
}

pub trait SubValueRef {
    fn get_raw(&self) -> RawValueRef;
    const ID: usize;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ValueRef(usize);

impl Indexable for ValueRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl<T: SubValueRef> From<T> for ValueRef {
    fn from(x: T) -> ValueRef {
        ValueRef(x.get_raw().to_index())
    }
}

impl From<RawValueRef> for ValueRef {
    fn from(x: RawValueRef) -> Self {
        Self(x.to_index())
    }
}

impl ValueRef {
    pub fn raw(&self) -> RawValueRef {
        RawValueRef::new(self.to_index())
    }

    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a Value> {
        ctx.get_data_value(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut Value> {
        ctx.get_data_value_mut(*self)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct InstructionRef(usize);

impl Indexable for InstructionRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl SubValueRef for InstructionRef {
    fn get_raw(&self) -> RawValueRef {
        RawValueRef::new(self.0)
    }

    const ID: usize = 1;
}

impl From<RawValueRef> for InstructionRef {
    fn from(x: RawValueRef) -> Self {
        if x.get_id() != Self::ID {
            panic!("invalid ref {:?}", x);
        };
        Self(x.to_index())
    }
}

impl InstructionRef {
    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a Instruction> {
        ctx.get_data_instruction(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut Instruction> {
        ctx.get_data_instruction_mut(*self)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BasicBlockRef(usize);

impl Indexable for BasicBlockRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl SubValueRef for BasicBlockRef {
    fn get_raw(&self) -> RawValueRef {
        RawValueRef::new(self.0)
    }

    const ID: usize = 2;
}

impl From<RawValueRef> for BasicBlockRef {
    fn from(x: RawValueRef) -> Self {
        if x.get_id() != Self::ID {
            panic!("invalid ref {:?}", x);
        };
        Self(x.to_index())
    }
}

impl BasicBlockRef {
    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a BasicBlock> {
        ctx.get_data_basic_block(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut BasicBlock> {
        ctx.get_data_basic_block_mut(*self)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FunctionRef(usize);

impl Indexable for FunctionRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl SubValueRef for FunctionRef {
    fn get_raw(&self) -> RawValueRef {
        RawValueRef::new(self.0)
    }

    const ID: usize = 3;
}

impl From<RawValueRef> for FunctionRef {
    fn from(x: RawValueRef) -> Self {
        if x.get_id() != Self::ID {
            panic!("invalid ref {:?}", x);
        };
        Self(x.to_index())
    }
}

impl FunctionRef {
    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a Function> {
        ctx.get_data_function(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut Function> {
        ctx.get_data_function_mut(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ConstantRef(usize);

impl Indexable for ConstantRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl SubValueRef for ConstantRef {
    fn get_raw(&self) -> RawValueRef {
        RawValueRef::new(self.0)
    }

    const ID: usize = 4;
}

impl From<RawValueRef> for ConstantRef {
    fn from(x: RawValueRef) -> Self {
        if x.get_id() != Self::ID {
            panic!("invalid ref {:?}", x);
        };
        Self(x.to_index())
    }
}

impl ConstantRef {
    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a Constant> {
        ctx.get_data_constant(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut Constant> {
        ctx.get_data_constant_mut(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ArgumentRef(usize);

impl Indexable for ArgumentRef {
    fn to_index(&self) -> usize {
        self.0
    }
}

impl SubValueRef for ArgumentRef {
    fn get_raw(&self) -> RawValueRef {
        RawValueRef::new(self.0)
    }

    const ID: usize = 5;
}

impl From<RawValueRef> for ArgumentRef {
    fn from(x: RawValueRef) -> Self {
        if x.get_id() != Self::ID {
            panic!("invalid ref {:?}", x);
        };
        Self(x.to_index())
    }
}

impl ArgumentRef {
    pub fn own<'a>(&self, ctx: &'a Context) -> Option<&'a Argument> {
        ctx.get_data_argument(*self)
    }

    pub fn own_mut<'a>(&self, ctx: &'a mut Context) -> Option<&'a mut Argument> {
        ctx.get_data_argument_mut(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ValueRefEnum {
    Ins(InstructionRef),
    BB(BasicBlockRef),
    Fun(FunctionRef),
    Const(ConstantRef),
    Arg(ArgumentRef),
}

impl ValueRef {
    pub fn to_enum(&self) -> ValueRefEnum {
        let raw = RawValueRef::new(self.to_index());
        match raw.get_id() {
            InstructionRef::ID => ValueRefEnum::Ins(raw.into()),
            BasicBlockRef::ID => ValueRefEnum::BB(raw.into()),
            FunctionRef::ID => ValueRefEnum::Fun(raw.into()),
            ConstantRef::ID => ValueRefEnum::Const(raw.into()),
            ArgumentRef::ID => ValueRefEnum::Arg(raw.into()),
            _ => unreachable!(),
        }
    }
}
