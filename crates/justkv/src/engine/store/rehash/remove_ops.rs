use std::borrow::Borrow;
use std::hash::Hash;

use super::constants::{NIL, REHASH_STEPS_PER_WRITE};
use super::index::bucket_index;
use super::types::{RehashingMap, TargetTable};

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub(in crate::engine::store) fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.rehash_step(REHASH_STEPS_PER_WRITE);

        if let Some(value) = self.remove_from_table(TargetTable::New, key) {
            self.len -= 1;
            return Some(value);
        }

        if let Some(value) = self.remove_from_table(TargetTable::Old, key) {
            self.len -= 1;
            return Some(value);
        }

        None
    }

    pub(in crate::engine::store::rehash) fn remove_from_table<Q>(
        &mut self,
        target: TargetTable,
        key: &Q,
    ) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let (bucket_count, mut head) = match target {
            TargetTable::Old => {
                let bucket = bucket_index(&self.hash_builder, key, self.table.len());
                (self.table.len(), self.table.heads[bucket])
            }
            TargetTable::New => {
                let table = self.rehash_table.as_ref()?;
                let bucket = bucket_index(&self.hash_builder, key, table.len());
                (table.len(), table.heads[bucket])
            }
        };

        if bucket_count == 0 {
            return None;
        }

        let bucket = bucket_index(&self.hash_builder, key, bucket_count);
        let mut prev = NIL;
        while head != NIL {
            let next = self.nodes[head as usize].as_ref().unwrap().next;
            if self.nodes[head as usize].as_ref().unwrap().key.borrow() == key {
                if prev == NIL {
                    match target {
                        TargetTable::Old => self.table.heads[bucket] = next,
                        TargetTable::New => {
                            if let Some(table) = self.rehash_table.as_mut() {
                                table.heads[bucket] = next;
                            }
                        }
                    }
                } else {
                    self.nodes[prev as usize].as_mut().unwrap().next = next;
                }

                let node = self.nodes[head as usize].take().unwrap();
                self.free.push(head);
                return Some(node.value);
            }
            prev = head;
            head = next;
        }
        None
    }
}
