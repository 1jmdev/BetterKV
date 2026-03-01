use std::borrow::Borrow;
use std::hash::Hash;

use ahash::RandomState;

use super::constants::NIL;
use super::node::Node;

pub(super) fn bucket_index<Q: Hash + ?Sized>(
    hash_builder: &RandomState,
    key: &Q,
    bucket_count: usize,
) -> usize {
    (hash_builder.hash_one(key) as usize) & (bucket_count - 1)
}

pub(super) fn find_in_chain<K, V, Q>(
    nodes: &[Option<Node<K, V>>],
    mut head: u32,
    key: &Q,
) -> Option<u32>
where
    K: Borrow<Q>,
    Q: Eq + ?Sized,
{
    while head != NIL {
        let node = nodes[head as usize].as_ref().unwrap();
        if node.key.borrow() == key {
            return Some(head);
        }
        head = node.next;
    }
    None
}
