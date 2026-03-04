use super::node::Node;

pub struct Iter<'a, K, V> {
    pub(super) nodes: &'a [Node<K, V>],
    pub(super) index: usize,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(nodes: &'a [Node<K, V>]) -> Self {
        let _trace = profiler::scope("rehash::iter::new");
        Self { nodes, index: 0 }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let _trace = profiler::scope("rehash::iter::next");
        if self.index >= self.nodes.len() {
            return None;
        }
        let idx = self.index;
        self.index += 1;
        let node = &self.nodes[idx];
        Some((&node.key, &node.value))
    }
}
