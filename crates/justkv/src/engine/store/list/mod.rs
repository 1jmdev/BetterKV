mod core_ops;
mod move_ops;
mod multi_pop_ops;

use crate::engine::value::{Entry, ListValue};

fn get_list(entry: &Entry) -> Option<&ListValue> {
    entry.as_list()
}

fn get_list_mut(entry: &mut Entry) -> Option<&mut ListValue> {
    entry.as_list_mut()
}
