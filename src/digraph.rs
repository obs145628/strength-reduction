#[derive(Debug, Clone)]
pub struct Digraph {
    v: usize,
    e: usize,
    adj: Vec<bool>,

    labels_vertex_names: Vec<String>,
}

impl Digraph {
    pub fn new(v: usize) -> Digraph {
        Digraph {
            v,
            e: 0,
            adj: vec![false; v * v],
            labels_vertex_names: vec![String::new(); v],
        }
    }

    pub fn v(&self) -> usize {
        self.v
    }

    pub fn e(&self) -> usize {
        self.e
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        self.adj[self.adj_index(u, v)]
    }

    pub fn add_edge(&mut self, u: usize, v: usize) -> bool {
        let idx = self.adj_index(u, v);
        let v = self.adj.get_mut(idx).unwrap();
        let res = !*v;
        *v = true;
        if res {
            self.e += 1
        }
        res
    }

    pub fn del_edge(&mut self, u: usize, v: usize) -> bool {
        let idx = self.adj_index(u, v);
        let v = self.adj.get_mut(idx).unwrap();
        let res = *v;
        *v = false;
        if res {
            self.e -= 1
        }
        res
    }

    pub fn vertices(&self) -> impl Iterator<Item = usize> {
        (0..self.v)
    }

    pub fn edges<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        let v = self.v;
        (0..v * v)
            .map(move |x| (x / v, x % v))
            .filter(move |(u, v)| self.has_edge(*u, *v))
    }

    pub fn preds<'a>(&'a self, u: usize) -> impl Iterator<Item = usize> + 'a {
        self.vertices().filter(move |v| self.has_edge(*v, u))
    }

    pub fn succs<'a>(&'a self, u: usize) -> impl Iterator<Item = usize> + 'a {
        self.vertices().filter(move |v| self.has_edge(u, *v))
    }

    pub fn set_label_vertex_name(&mut self, u: usize, name: &str) {
        self.labels_vertex_names[u] = name.to_string();
    }

    pub fn dump_tree<T: std::io::Write>(&self, os: &mut T) -> std::io::Result<()> {
        write!(os, "digraph G {{\n")?;

        for u in self.vertices() {
            write!(
                os,
                "  {} [ label=\"{}\" ];\n",
                u, self.labels_vertex_names[u]
            )?;
        }

        for (u, v) in self.edges() {
            write!(os, "  {} -> {}\n", u, v)?;
        }

        write!(os, "}}\n")
    }

    pub fn save_tree(&self, path: &str) -> std::io::Result<()> {
        let os = std::fs::File::create(path)?;
        let mut os = std::io::BufWriter::new(&os);
        self.dump_tree(&mut os)
    }

    fn adj_index(&self, u: usize, v: usize) -> usize {
        assert!(u < self.v);
        assert!(v < self.v);
        u * self.v + v
    }
}
