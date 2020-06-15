use crate::context::Context;
use crate::valueref::ValueRef;

pub struct Value {
    name: String,
    id: ValueRef,
    is_def: bool,
    users: Vec<ValueRef>,
    ops: Vec<ValueRef>,
}

impl Value {
    pub fn new(name: &str, id: ValueRef, is_def: bool, ops: &[ValueRef]) -> Value {
        Value {
            name: name.to_string(),
            id,
            is_def,
            users: vec![],
            ops: ops.to_vec(),
        }
    }

    pub fn id(&self) -> ValueRef {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rename(&mut self, new_name: &str) {
        self.name = new_name.to_string()
    }

    pub fn is_def(&self) -> bool {
        self.is_def
    }

    pub fn users(&self) -> &[ValueRef] {
        &self.users[..]
    }

    pub fn users_mut(&mut self) -> &mut [ValueRef] {
        &mut self.users[..]
    }

    pub fn ops(&self) -> &[ValueRef] {
        &self.ops[..]
    }

    pub fn ops_mut(&mut self) -> &mut [ValueRef] {
        &mut self.ops[..]
    }

    pub fn users_del(&mut self, v: ValueRef) {
        self.users.retain(|x| *x != v);
    }

    pub fn users_add(&mut self, v: ValueRef) {
        if !self.users.iter().find(|x| **x == v).is_some() {
            self.users.push(v)
        }
    }

    pub fn has_op(&self, v: ValueRef) -> bool {
        self.ops.iter().find(|x| **x == v).is_some()
    }

    pub fn set_op(&mut self, ctx: &mut Context, idx: usize, val: ValueRef) {
        let old_val = self.ops[idx];
        if old_val == val {
            return;
        }

        if let Some(old_val) = old_val.own_mut(ctx) {
            old_val.users.retain(|x| *x != self.id);
        }

        if let Some(val) = val.own_mut(ctx) {
            val.users.push(self.id);
        }

        self.ops[idx] = val;
    }
}

impl From<&Value> for ValueRef {
    fn from(x: &Value) -> Self {
        x.id
    }
}
