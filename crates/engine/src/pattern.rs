pub(super) enum CompiledPattern<'a> {
    Any,
    Exact(&'a [u8]),
    Prefix(&'a [u8]),
    Suffix(&'a [u8]),
    Contains(&'a [u8]),
    PrefixSuffix { prefix: &'a [u8], suffix: &'a [u8] },
    Wildcard(&'a [u8]),
}

impl<'a> CompiledPattern<'a> {
    pub(super) fn new(pattern: Option<&'a [u8]>) -> Self {
        let _trace = profiler::scope("engine::pattern::compiled_new");
        let Some(pattern) = pattern else {
            return Self::Any;
        };
        if pattern.is_empty() || pattern == b"*" {
            return Self::Any;
        }

        let mut first_star = None;
        let mut star_count = 0usize;
        let mut has_question = false;

        for (idx, &byte) in pattern.iter().enumerate() {
            match byte {
                b'*' => {
                    star_count += 1;
                    if first_star.is_none() {
                        first_star = Some(idx);
                    }
                }
                b'?' => has_question = true,
                _ => {}
            }
        }

        if has_question {
            return Self::Wildcard(pattern);
        }

        match (star_count, first_star) {
            (0, _) => Self::Exact(pattern),
            (1, Some(pos)) if pos == pattern.len() - 1 => Self::Prefix(&pattern[..pos]),
            (1, Some(0)) => Self::Suffix(&pattern[1..]),
            (1, Some(pos)) => Self::PrefixSuffix {
                prefix: &pattern[..pos],
                suffix: &pattern[pos + 1..],
            },
            (2, Some(0)) if pattern[pattern.len() - 1] == b'*' => {
                Self::Contains(&pattern[1..pattern.len() - 1])
            }
            _ => Self::Wildcard(pattern),
        }
    }
}

pub(super) fn wildcard_match(pattern: &[u8], text: &[u8]) -> bool {
    let _trace = profiler::scope("engine::pattern::wildcard_match");
    let mut pi = 0;
    let mut ti = 0;
    let mut star = None;
    let mut star_match = 0;

    while ti < text.len() {
        if pi < pattern.len() && (pattern[pi] == text[ti] || pattern[pi] == b'?') {
            pi += 1;
            ti += 1;
            continue;
        }

        if pi < pattern.len() && pattern[pi] == b'*' {
            star = Some(pi);
            pi += 1;
            star_match = ti;
            continue;
        }

        match star {
            Some(position) => {
                pi = position + 1;
                star_match += 1;
                ti = star_match;
            }
            None => return false,
        }
    }

    while pi < pattern.len() && pattern[pi] == b'*' {
        pi += 1;
    }

    pi == pattern.len()
}
