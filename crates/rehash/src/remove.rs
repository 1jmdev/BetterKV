use super::index::hash_key;
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::remove::remove");
        let key_bytes = key.as_ref();
        let hash = hash_key(self.seed, key_bytes);
        let bucket = (hash as usize) & self.table.mask;

        let mut cur = self.table.heads[bucket];
        let mut prev = super::constants::NIL;

        // Single pass: Find AND unlink in one go
        while cur != super::constants::NIL {
            let node = &self.nodes[cur as usize];
            if node.hash == hash && node.key.as_ref() == key_bytes {
                let next = node.next;
                if prev == super::constants::NIL {
                    self.table.heads[bucket] = next;
                } else {
                    self.nodes[prev as usize].next = next;
                }

                let removed = self.nodes.swap_remove(cur as usize);
                if (cur as usize) < self.nodes.len() {
                    let old_last_idx = self.nodes.len() as u32;
                    self.patch_swapped(old_last_idx, cur);
                }

                return Some(removed.value);
            }
            prev = cur;
            cur = node.next;
        }
        None
    }
}
