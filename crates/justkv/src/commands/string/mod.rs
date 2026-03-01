mod counter_ops;
mod expiry_ops;
mod get_set_ops;
mod length_ops;
mod multi_ops;

use crate::commands::util::{Args, CommandId};
use crate::engine::store::Store;
use crate::protocol::types::RespFrame;

pub fn handle(store: &Store, cmd: CommandId, args: &Args) -> RespFrame {
    match cmd {
        // get/set group
        CommandId::Get => get_set_ops::get(store, args),
        CommandId::Set => get_set_ops::set(store, args),
        CommandId::Setnx => get_set_ops::setnx(store, args),
        CommandId::Getset => get_set_ops::getset(store, args),
        CommandId::Getdel => get_set_ops::getdel(store, args),
        // expiry group
        CommandId::Setex => expiry_ops::setex(store, args),
        CommandId::Psetex => expiry_ops::psetex(store, args),
        CommandId::Getex => expiry_ops::getex(store, args),
        // length group
        CommandId::Append => length_ops::append(store, args),
        CommandId::Strlen => length_ops::strlen(store, args),
        CommandId::Setrange => length_ops::setrange(store, args),
        CommandId::Getrange => length_ops::getrange(store, args),
        // multi group
        CommandId::Mget => multi_ops::mget(store, args),
        CommandId::Mset => multi_ops::mset(store, args),
        CommandId::Msetnx => multi_ops::msetnx(store, args),
        // counter group
        CommandId::Incr => counter_ops::incr(store, args),
        CommandId::Incrby => counter_ops::incrby(store, args),
        CommandId::Decr => counter_ops::decr(store, args),
        CommandId::Decrby => counter_ops::decrby(store, args),
        _ => unreachable!("string::handle called with non-string command"),
    }
}
