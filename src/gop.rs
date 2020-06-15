use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;

// generic instruction
pub struct Ins {
    args: Vec<String>,
}

impl Ins {
    pub fn args(&self) -> &[String] {
        &self.args[..]
    }

    fn parse(s: &str) -> Ins {
        let (s_op, s_args) = match s.find(' ') {
            None => (s.trim(), ""),
            Some(idx) => (s[..idx].trim(), s[idx + 1..].trim()),
        };

        let mut args = Vec::new();
        args.push(String::from(s_op));

        if !s_args.is_empty() {
            let mut rest: Vec<String> = s_args.split(',').map(|s| s.trim().to_string()).collect();
            args.append(&mut rest);
        }

        Ins { args }
    }
}

impl fmt::Display for Ins {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, s) in self.args.iter().enumerate() {
            write!(f, "{}", s)?;
            if idx == 0 {
                write!(f, " ")?;
            } else if idx + 1 != self.args.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "")
    }
}

// generic instruction
pub struct Dir {
    args: Vec<String>,
}

impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, s) in self.args.iter().enumerate() {
            if idx == 0 {
                write!(f, ".")?
            }
            write!(f, "{}", s)?;
            if idx == 0 {
                write!(f, " ")?;
            } else if idx + 1 != self.args.len() {
                write!(f, ", ")?;
            }
        }

        write!(f, "")
    }
}

impl Dir {
    pub fn args(&self) -> &[String] {
        &self.args[..]
    }

    fn parse(s: &str) -> Dir {
        if s.is_empty() || s.chars().next().unwrap() != '.' {
            panic!("Invalid directive");
        }

        let (s_op, s_args) = match s.find(' ') {
            None => (s[1..].trim(), ""),
            Some(idx) => (s[1..idx].trim(), s[idx + 1..].trim()),
        };

        let mut args = Vec::new();
        args.push(String::from(s_op));

        if !s_args.is_empty() {
            let mut rest: Vec<String> = s_args.split(',').map(|s| s.trim().to_string()).collect();
            args.append(&mut rest);
        }

        Dir { args }
    }
}

pub enum DeclBody {
    Ins(Ins),
    Dir(Dir),
}

impl DeclBody {
    fn parse(s: &str) -> DeclBody {
        if !s.is_empty() && s.chars().next().unwrap() == '.' {
            DeclBody::Dir(Dir::parse(s))
        } else {
            DeclBody::Ins(Ins::parse(s))
        }
    }
}

pub struct Decl {
    label_defs: Vec<String>,
    comm_pre: Vec<String>,
    comm_eol: String,

    body: DeclBody,
}

impl Decl {
    pub fn new_ins(
        label_defs: Vec<String>,
        comm_pre: Vec<String>,
        comm_eol: String,
        args: Vec<String>,
    ) -> Decl {
        Decl {
            label_defs,
            comm_pre,
            comm_eol,
            body: DeclBody::Ins(Ins { args }),
        }
    }

    pub fn new_dir(
        label_defs: Vec<String>,
        comm_pre: Vec<String>,
        comm_eol: String,
        args: Vec<String>,
    ) -> Decl {
        Decl {
            label_defs,
            comm_pre,
            comm_eol,
            body: DeclBody::Dir(Dir { args }),
        }
    }

    pub fn label_defs(&self) -> &[String] {
        &self.label_defs[..]
    }

    pub fn body(&self) -> &DeclBody {
        &self.body
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.label_defs.is_empty() {
            write!(f, "\n")?;
            for label in &self.label_defs {
                write!(f, "{}:\n", label)?;
            }
        }

        for comm in &self.comm_pre {
            write!(f, " ; {}\n", comm)?;
        }

        match &self.body {
            DeclBody::Ins(i) => write!(f, "\t{}", i)?,
            DeclBody::Dir(d) => write!(f, "{}", d)?,
        }

        if !self.comm_eol.is_empty() {
            write!(f, " ; {}", self.comm_eol)?;
        }

        write!(f, "\n")
    }
}

pub struct Module {
    decls: Vec<Decl>,
}

impl Module {
    pub fn new(decls: Vec<Decl>) -> Module {
        Module { decls }
    }

    pub fn decls(&self) -> &[Decl] {
        &self.decls[..]
    }

    pub fn parse(path: &str) -> Module {
        let fis = File::open(path).expect("Failed to open file");
        let lines = io::BufReader::new(fis).lines();

        let mut decls: Vec<Decl> = Vec::new();
        let mut label_defs: Vec<String> = Vec::new();
        let mut comm_pre: Vec<String> = Vec::new();

        for line in lines {
            let line = line.expect("Failed to read file");
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.chars().next().unwrap() == ';' {
                comm_pre.push(line[1..].to_string());
                continue;
            }

            if line.chars().last().unwrap() == ':' {
                label_defs.push(line[..line.len() - 1].to_string());
                continue;
            }

            let (comm_eol, body) = match line.find(';') {
                None => (String::new(), DeclBody::parse(line)),
                Some(p) => (line[p + 1..].to_string(), DeclBody::parse(&line[..p])),
            };

            let mut decl_labels = Vec::new();
            let mut decl_comms = Vec::new();
            std::mem::swap(&mut decl_labels, &mut label_defs);
            std::mem::swap(&mut decl_comms, &mut comm_pre);
            decls.push(Decl {
                label_defs: decl_labels,
                comm_pre: decl_comms,
                comm_eol,
                body,
            });
        }

        Module::new(decls)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for d in &self.decls {
            write!(f, "{}", d)?;
        }
        write!(f, "\n")
    }
}
