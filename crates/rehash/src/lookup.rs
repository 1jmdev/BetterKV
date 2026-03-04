use super::constants::NIL;
use super::index::hash_key;
use super::types::RehashingMap;

impl<K, V> RehashingMap<K, V>
where
    K: Eq + AsRef<[u8]>,
{
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::contains_key");
        self.find_index(key).is_some()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::get");
        let idx = self.find_index(key)?;
        Some(&self.nodes[idx as usize].value)
    }

    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::get_mut");
        let idx = self.find_index(key)?;
        Some(&mut self.nodes[idx as usize].value)
    }

    #[inline(always)]
    pub fn find_index<Q>(&self, key: &Q) -> Option<u32>
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::find_index");
        let hash = hash_key(self.seed, key.as_ref());
        self.find_index_hashed(key, hash)
    }

    #[inline(always)]
    pub fn find_index_hashed<Q>(&self, key: &Q, hash: u64) -> Option<u32>
    where
        Q: AsRef<[u8]> + ?Sized,
    {
        let _trace = profiler::scope("rehash::lookup::find_index_hashed");
        let key_bytes = key.as_ref();
        let bucket = (hash as usize) & self.table.mask;
        let mut idx = unsafe { *self.table.heads.get_unchecked(bucket) };

        while idx != NIL {
            let node = unsafe { self.nodes.get_unchecked(idx as usize) };
            if node.hash == hash && node.key.as_ref() == key_bytes {
                return Some(idx);
            }
            idx = node.next;
        }
        None
    }
}
