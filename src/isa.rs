use std::collections::HashMap;

pub struct InsInfos {
    name: &'static str,
    is_term: bool, //terminator instruction (ret / jump / branch)
    is_def: bool,  //define a value (instruction store result in some reg)
}

impl InsInfos {
    pub fn name(&self) -> &'static str {
        &self.name
    }

    pub fn is_term(&self, _args: &[String]) -> bool {
        self.is_term
    }

    pub fn is_def(&self, args: &[String]) -> bool {
        if (self.name == "call") {
            self.is_def_call(args)
        } else {
            self.is_def
        }
    }

    fn is_def_call(&self, args: &[String]) -> bool {
        return args[1].chars().next().unwrap() == '%';
    }
}

pub struct ISA {
    ins_infos: HashMap<&'static str, InsInfos>,
}

impl ISA {
    pub fn instance() -> &'static ISA {
        &ISA_INSTANCE
    }

    pub fn find_ins(&self, name: &str) -> Option<&InsInfos> {
        self.ins_infos.get(name)
    }

    fn new() -> ISA {
        let mut res = ISA {
            ins_infos: HashMap::new(),
        };
        res.setup();
        res
    }

    fn add_ins(&mut self, name: &'static str, is_term: bool, is_def: bool) {
        let infos = InsInfos {
            name,
            is_term,
            is_def,
        };
        self.ins_infos.insert(name, infos);
    }

    fn setup(&mut self) {
        self.add_ins("add", /*is_term=*/ false, /*is_def=*/ true);
        self.add_ins("b", /*is_term=*/ true, /*is_def=*/ false);
        self.add_ins("bc", /*is_term=*/ true, /*is_def=*/ false);
        self.add_ins("call", /*is_term=*/ false, /*is_def=*/ false);
        self.add_ins("cmplt", /*is_term=*/ false, /*is_def=*/ true);
        self.add_ins("mul", /*is_term=*/ false, /*is_def=*/ true);
        self.add_ins("phi", /*is_term=*/ false, /*is_def=*/ true);
        self.add_ins("ret", /*is_term=*/ true, /*is_def=*/ false);
        self.add_ins("sub", /*is_term=*/ false, /*is_def=*/ true);
    }
}

lazy_static! {
    static ref ISA_INSTANCE: ISA = ISA::new();
}
