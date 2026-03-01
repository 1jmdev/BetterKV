use std::hash::Hash;

use super::constants::{NIL, REHASH_STEPS_PER_WRITE};
use super::index::bucket_index;
use super::node::Node;
use super::types::{RehashingMap, TargetTable};

impl<K, V> RehashingMap<K, V>
where
    K: Eq + Hash,
{
    pub(in crate::engine::store) fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.rehash_step(REHASH_STEPS_PER_WRITE);

        if let Some(idx) = self.find_index(&key) {
            let node = self.nodes[idx as usize].as_mut().unwrap();
            return Some(std::mem::replace(&mut node.value, value));
        }

        let target = if self.rehash_table.is_some() {
            TargetTable::New
        } else {
            TargetTable::Old
        };
        self.insert_new(target, key, value);
        self.len += 1;
        self.maybe_start_rehash();
        None
    }

    pub(in crate::engine::store) fn get_or_insert_with<F>(&mut self, key: K, default: F) -> &mut V
    where
        F: FnOnce() -> V,
    {
        self.rehash_step(REHASH_STEPS_PER_WRITE);

        if let Some(idx) = self.find_index(&key) {
            return &mut self.nodes[idx as usize].as_mut().unwrap().value;
        }

        let target = if self.rehash_table.is_some() {
            TargetTable::New
        } else {
            TargetTable::Old
        };
        let idx = self.insert_new(target, key, default());
        self.len += 1;
        self.maybe_start_rehash();
        &mut self.nodes[idx as usize].as_mut().unwrap().value
    }

    pub(super) fn insert_new(&mut self, target: TargetTable, key: K, value: V) -> u32 {
        let idx = self.alloc_node(Node {
            key,
            value,
            next: NIL,
        });

        match target {
            TargetTable::Old => {
                let bucket = bucket_index(
                    &self.hash_builder,
                    &self.nodes[idx as usize].as_ref().unwrap().key,
                    self.table.len(),
                );
                let head = self.table.heads[bucket];
                self.nodes[idx as usize].as_mut().unwrap().next = head;
                self.table.heads[bucket] = idx;
            }
            TargetTable::New => {
                if let Some(table) = self.rehash_table.as_mut() {
                    let bucket = bucket_index(
                        &self.hash_builder,
                        &self.nodes[idx as usize].as_ref().unwrap().key,
                        table.len(),
                    );
                    let head = table.heads[bucket];
                    self.nodes[idx as usize].as_mut().unwrap().next = head;
                    table.heads[bucket] = idx;
                }
            }
        }

        idx
    }

    pub(super) fn alloc_node(&mut self, node: Node<K, V>) -> u32 {
        if let Some(idx) = self.free.pop() {
            self.nodes[idx as usize] = Some(node);
            idx
        } else {
            let idx = self.nodes.len() as u32;
            self.nodes.push(Some(node));
            idx
        }
    }
}
