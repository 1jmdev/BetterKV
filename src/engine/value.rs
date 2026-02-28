use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

const INLINE_BYTES_CAPACITY: usize = 22;

#[derive(Clone, Debug)]
pub enum CompactBytes {
    Inline {
        len: u8,
        data: [u8; INLINE_BYTES_CAPACITY],
    },
    Heap(Box<[u8]>),
}

impl CompactBytes {
    pub fn from_vec(value: Vec<u8>) -> Self {
        if value.len() <= INLINE_BYTES_CAPACITY {
            let mut data = [0; INLINE_BYTES_CAPACITY];
            data[..value.len()].copy_from_slice(&value);
            Self::Inline {
                len: value.len() as u8,
                data,
            }
        } else {
            Self::Heap(value.into_boxed_slice())
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        match self {
            Self::Inline { len, data } => &data[..*len as usize],
            Self::Heap(value) => value,
        }
    }

    pub fn len(&self) -> usize {
        self.as_slice().len()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    pub fn into_vec(self) -> Vec<u8> {
        match self {
            Self::Inline { len, data } => data[..len as usize].to_vec(),
            Self::Heap(value) => value.into_vec(),
        }
    }
}

impl PartialEq for CompactBytes {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for CompactBytes {}

impl Hash for CompactBytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl Borrow<[u8]> for CompactBytes {
    fn borrow(&self) -> &[u8] {
        self.as_slice()
    }
}

#[derive(Clone, Debug)]
pub struct Entry {
    pub value: CompactBytes,
    pub expires_at_ms: u64,
}

impl Entry {
    pub fn new(value: Vec<u8>, expires_at_ms: u64) -> Self {
        Self {
            value: CompactBytes::from_vec(value),
            expires_at_ms,
        }
    }

    pub fn is_expired(&self, now_ms: u64) -> bool {
        self.expires_at_ms != 0 && now_ms >= self.expires_at_ms
    }
}
