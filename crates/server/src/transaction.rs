use std::collections::HashMap;

use engine::store::Store;
use protocol::types::RespFrame;

#[derive(Default)]
pub struct TransactionState {
    in_multi: bool,
    queued: Vec<RespFrame>,
    watched: HashMap<Vec<u8>, Option<Vec<u8>>>,
}

impl TransactionState {
    pub fn handle_frame_with<F>(
        &mut self,
        store: &Store,
        frame: RespFrame,
        mut execute: F,
    ) -> RespFrame
    where
        F: FnMut(&Store, RespFrame) -> RespFrame,
    {
        let _trace = profiler::scope("server::transaction::handle_frame_with");
        let command = match command_name(&frame) {
            Ok(Some(value)) => value,
            Ok(None) => return RespFrame::error_static("ERR empty command"),
            Err(err) => return RespFrame::error_static(err),
        };

        match classify_transaction_command(command) {
            TransactionCommand::Multi => {
                let args = match parse_args(&frame) {
                    Ok(value) => value,
                    Err(err) => return RespFrame::error_static(err),
                };
                self.multi(&args)
            }
            TransactionCommand::Exec => {
                let args = match parse_args(&frame) {
                    Ok(value) => value,
                    Err(err) => return RespFrame::error_static(err),
                };
                self.exec_with(store, &args, execute)
            }
            TransactionCommand::Discard => {
                let args = match parse_args(&frame) {
                    Ok(value) => value,
                    Err(err) => return RespFrame::error_static(err),
                };
                self.discard(&args)
            }
            TransactionCommand::Watch => {
                let args = match parse_args(&frame) {
                    Ok(value) => value,
                    Err(err) => return RespFrame::error_static(err),
                };
                self.watch(store, &args)
            }
            TransactionCommand::Unwatch => {
                let args = match parse_args(&frame) {
                    Ok(value) => value,
                    Err(err) => return RespFrame::error_static(err),
                };
                self.unwatch(&args)
            }
            TransactionCommand::Other => {
                if self.in_multi {
                    self.queued.push(frame);
                    RespFrame::Simple("QUEUED".to_string())
                } else {
                    execute(store, frame)
                }
            }
        }
    }

    fn multi(&mut self, args: &[&[u8]]) -> RespFrame {
        let _trace = profiler::scope("server::transaction::multi");
        if args.len() != 1 {
            return wrong_args("MULTI");
        }
        if self.in_multi {
            return RespFrame::Error("ERR MULTI calls can not be nested".to_string());
        }

        self.in_multi = true;
        self.queued.clear();
        RespFrame::ok()
    }

    fn exec_with<F>(&mut self, store: &Store, args: &[&[u8]], mut execute: F) -> RespFrame
    where
        F: FnMut(&Store, RespFrame) -> RespFrame,
    {
        let _trace = profiler::scope("server::transaction::exec_with");
        if args.len() != 1 {
            return wrong_args("EXEC");
        }
        if !self.in_multi {
            return RespFrame::Error("ERR EXEC without MULTI".to_string());
        }

        self.in_multi = false;
        if self.is_watch_dirty(store) {
            self.queued.clear();
            self.watched.clear();
            return RespFrame::Array(None);
        }

        let queued = std::mem::take(&mut self.queued);
        let mut out = Vec::with_capacity(queued.len());
        for item in queued {
            out.push(execute(store, item));
        }
        self.watched.clear();
        RespFrame::Array(Some(out))
    }

    fn discard(&mut self, args: &[&[u8]]) -> RespFrame {
        let _trace = profiler::scope("server::transaction::discard");
        if args.len() != 1 {
            return wrong_args("DISCARD");
        }
        if !self.in_multi {
            return RespFrame::Error("ERR DISCARD without MULTI".to_string());
        }

        self.in_multi = false;
        self.queued.clear();
        self.watched.clear();
        RespFrame::ok()
    }

    fn watch(&mut self, store: &Store, args: &[&[u8]]) -> RespFrame {
        let _trace = profiler::scope("server::transaction::watch");
        if args.len() < 2 {
            return wrong_args("WATCH");
        }
        if self.in_multi {
            return RespFrame::Error("ERR WATCH inside MULTI is not allowed".to_string());
        }

        for key in &args[1..] {
            self.watched.insert(key.to_vec(), store.dump(key));
        }
        RespFrame::ok()
    }

    fn unwatch(&mut self, args: &[&[u8]]) -> RespFrame {
        let _trace = profiler::scope("server::transaction::unwatch");
        if args.len() != 1 {
            return wrong_args("UNWATCH");
        }
        self.watched.clear();
        RespFrame::ok()
    }

    fn is_watch_dirty(&self, store: &Store) -> bool {
        let _trace = profiler::scope("server::transaction::is_watch_dirty");
        self.watched
            .iter()
            .any(|(key, value)| store.dump(key) != *value)
    }
}

fn command_name<'a>(frame: &'a RespFrame) -> Result<Option<&'a [u8]>, &'static str> {
    let _trace = profiler::scope("server::transaction::command_name");
    let RespFrame::Array(Some(items)) = frame else {
        return Err("ERR protocol error");
    };
    let Some(first) = items.first() else {
        return Ok(None);
    };

    match first {
        RespFrame::Bulk(Some(value)) => Ok(Some(value.as_slice())),
        RespFrame::Simple(value) => Ok(Some(value.as_bytes())),
        _ => Err("ERR invalid argument type"),
    }
}

#[derive(PartialEq, Eq)]
enum TransactionCommand {
    Multi,
    Exec,
    Discard,
    Watch,
    Unwatch,
    Other,
}

#[inline]
fn classify_transaction_command(command: &[u8]) -> TransactionCommand {
    match command.len() {
        4 if command.eq_ignore_ascii_case(b"EXEC") => TransactionCommand::Exec,
        5 => {
            if command.eq_ignore_ascii_case(b"MULTI") {
                TransactionCommand::Multi
            } else if command.eq_ignore_ascii_case(b"WATCH") {
                TransactionCommand::Watch
            } else {
                TransactionCommand::Other
            }
        }
        7 => {
            if command.eq_ignore_ascii_case(b"DISCARD") {
                TransactionCommand::Discard
            } else if command.eq_ignore_ascii_case(b"UNWATCH") {
                TransactionCommand::Unwatch
            } else {
                TransactionCommand::Other
            }
        }
        _ => TransactionCommand::Other,
    }
}

fn parse_args<'a>(frame: &'a RespFrame) -> Result<Vec<&'a [u8]>, &'static str> {
    let _trace = profiler::scope("server::transaction::parse_args");
    let RespFrame::Array(Some(items)) = frame else {
        return Err("ERR protocol error");
    };

    let mut args = Vec::with_capacity(items.len());
    for item in items {
        match item {
            RespFrame::Bulk(Some(value)) => args.push(value.as_slice()),
            RespFrame::Simple(value) => args.push(value.as_bytes()),
            _ => return Err("ERR invalid argument type"),
        }
    }

    Ok(args)
}

fn wrong_args(command: &str) -> RespFrame {
    let _trace = profiler::scope("server::transaction::wrong_args");
    RespFrame::Error(format!(
        "ERR wrong number of arguments for '{command}' command"
    ))
}
