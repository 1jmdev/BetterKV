pub struct Iter<'a, K, V> {
    keys: std::slice::Iter<'a, K>,
    values: std::slice::Iter<'a, V>,
}

impl<'a, K, V> Iter<'a, K, V> {
    pub(super) fn new(keys: &'a [K], values: &'a [V]) -> Self {
        Self {
            keys: keys.iter(),
            values: values.iter(),
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match (self.keys.next(), self.values.next()) {
            (Some(k), Some(v)) => Some((k, v)),
            _ => None,
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.keys.size_hint()
    }
}

impl<'a, K, V> ExactSizeIterator for Iter<'a, K, V> {}
