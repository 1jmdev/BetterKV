use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Entry {
    pub value: Vec<u8>,
    pub expires_at: Option<Instant>,
}

impl Entry {
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(deadline) => Instant::now() >= deadline,
            None => false,
        }
    }
}
