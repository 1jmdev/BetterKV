use super::constants::BULK_RESERVE_CAP;
use super::index::hash_key;
use super::node::Node;
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    pub fn insert_batch<I>(&mut self, entries: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        let _trace = profiler::scope("rehash::insert::insert_batch");
        let iter = entries.into_iter();
        let (lower_bound, _) = iter.size_hint();
        self.reserve_for_batch(lower_bound.min(BULK_RESERVE_CAP));

        for (key, value) in iter {
            self.insert(key, value);
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let _trace = profiler::scope("rehash::insert::insert");
        let key_bytes = key.as_ref();
        let hash = hash_key(self.seed, key_bytes);
        let mut bucket = (hash as usize) & self.table.mask;
        let mut idx = self.table.heads[bucket];

        while idx != super::constants::NIL {
            let node = &mut self.nodes[idx as usize];
            if node.hash == hash && node.key.as_ref() == key_bytes {
                return Some(std::mem::replace(&mut node.value, value));
            }
            idx = node.next;
        }

        self.maybe_grow();
        bucket = (hash as usize) & self.table.mask;
        let head = self.table.heads[bucket];
        assert!(
            self.nodes.len() < super::constants::NIL as usize,
            "Max capacity exceeded"
        );
        let idx = self.nodes.len() as u32;

        self.nodes.push(Node {
            hash,
            next: head,
            key,
            value,
        });
        self.table.heads[bucket] = idx;
        None
    }

    pub fn get_or_insert_with<F>(&mut self, key: K, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        let _trace = profiler::scope("rehash::insert::get_or_insert_with");
        let key_bytes = key.as_ref();
        let hash = hash_key(self.seed, key_bytes);
        let mut bucket = (hash as usize) & self.table.mask;
        let mut idx = self.table.heads[bucket];

        while idx != super::constants::NIL {
            let node = &self.nodes[idx as usize];
            if node.hash == hash && node.key.as_ref() == key_bytes {
                return &mut self.nodes[idx as usize].value;
            }
            idx = node.next;
        }

        self.maybe_grow();
        bucket = (hash as usize) & self.table.mask;
        let head = self.table.heads[bucket];
        assert!(
            self.nodes.len() < super::constants::NIL as usize,
            "Max capacity exceeded"
        );
        let idx = self.nodes.len() as u32;

        self.nodes.push(Node {
            hash,
            next: head,
            key,
            value: default(),
        });
        self.table.heads[bucket] = idx;
        &mut self.nodes[idx as usize].value
    }
}
