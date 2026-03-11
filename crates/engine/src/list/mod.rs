mod core;
mod moves;
mod multi_pop;

use types::value::{Entry, ListValue};

fn get_list(entry: &Entry) -> Option<&ListValue> {
    entry.as_list()
}

fn get_list_mut(entry: &mut Entry) -> Option<&mut ListValue> {
    entry.as_list_mut()
}
