use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone)]
pub struct VertexAdapter<T: Clone + Eq + Hash> {
    v2o: Vec<T>,
    o2v: HashMap<T, usize>,
}

impl<T> VertexAdapter<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new(data: &[T]) -> Self {
        let v2o = data.to_vec();
        let mut o2v = HashMap::new();
        for (v, o) in data.iter().enumerate() {
            o2v.insert(o.clone(), v);
        }

        assert!(v2o.len() == o2v.len());
        Self { v2o, o2v }
    }

    pub fn count(&self) -> usize {
        self.v2o.len()
    }

    pub fn v2o(&self, v: usize) -> T {
        self.v2o[v].clone()
    }

    pub fn o2v(&self, o: T) -> usize {
        *self.o2v.get(&o).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str() {
        let va = VertexAdapter::new(&["a".to_string(), "b".to_string(), "c".to_string()]);
        assert_eq!(va.count(), 3);
        assert_eq!(va.v2o(0), "a");
        assert_eq!(va.v2o(1), "b");
        assert_eq!(va.v2o(2), "c");
        assert_eq!(va.o2v("a".to_string()), 0);
        assert_eq!(va.o2v("b".to_string()), 1);
        assert_eq!(va.o2v("c".to_string()), 2);
    }
}
