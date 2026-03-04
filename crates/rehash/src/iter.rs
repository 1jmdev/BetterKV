use super::node::Node;

pub struct Iter<'a, K, V> {
    iter: std::slice::Iter<'a, Node<K, V>>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(nodes: &'a [Node<K, V>]) -> Self {
        Self { iter: nodes.iter() }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|n| (&n.key, &n.value))
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}