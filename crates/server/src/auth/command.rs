use commands::dispatch::{
    ChannelExtraction, CommandAuthSpec, CommandId, KeyExtraction, command_auth_spec,
};
use types::value::CompactArg;

pub(super) use commands::dispatch::AclCategory;

#[derive(Clone, Copy)]
pub(super) struct CommandSpec {
    pub(super) name: &'static str,
    pub(super) categories: &'static [AclCategory],
    key_extractor: KeyExtraction,
    channel_extractor: ChannelExtraction,
}

impl CommandSpec {
    fn from_auth_spec(spec: CommandAuthSpec) -> Self {
        Self {
            name: command_name(spec.name),
            categories: spec.categories,
            key_extractor: spec.key_extraction,
            channel_extractor: spec.channel_extraction,
        }
    }

    pub(super) fn unknown() -> Self {
        Self {
            name: "UNKNOWN",
            categories: &[AclCategory::Slow],
            key_extractor: KeyExtraction::None,
            channel_extractor: ChannelExtraction::None,
        }
    }

    pub(super) fn keys<'a>(&self, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
        extract_keys(self.key_extractor, args)
    }

    pub(super) fn channels<'a>(&self, args: &'a [CompactArg]) -> Vec<&'a [u8]> {
        extract_channels(self.channel_extractor, args)
    }
}

pub(super) fn acl_categories() -> &'static [AclCategory] {
    AclCategory::ALL_COMMAND_CATEGORIES
}

pub(super) fn commands_in_category(category: AclCategory) -> Vec<&'static str> {
    CommandId::ALL
        .iter()
        .copied()
        .filter_map(command_auth_spec)
        .filter(|spec| spec.categories.contains(&category))
        .map(|spec| command_name(spec.name))
        .collect()
}

pub(super) fn command_spec(command: CommandId) -> Option<CommandSpec> {
    command_auth_spec(command).map(CommandSpec::from_auth_spec)
}

fn command_name(bytes: &'static [u8]) -> &'static str {
    unsafe { std::str::from_utf8_unchecked(bytes) }
}

fn extract_keys(extractor: KeyExtraction, args: &[CompactArg]) -> Vec<&[u8]> {
    match extractor {
        KeyExtraction::None => Vec::new(),
        KeyExtraction::Single => get_arg(args, 1).into_iter().collect(),
        KeyExtraction::Pair => [get_arg(args, 1), get_arg(args, 2)]
            .into_iter()
            .flatten()
            .collect(),
        KeyExtraction::AllFrom(index) => {
            args.iter().skip(index).map(CompactArg::as_slice).collect()
        }
        KeyExtraction::AllExceptLastFrom(index) => args
            .get(index..args.len().saturating_sub(1))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtraction::EveryOtherFrom(index) => args
            .iter()
            .skip(index)
            .step_by(2)
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtraction::Counted {
            count_index,
            first_key,
        } => parse_count(args, count_index)
            .and_then(|count| args.get(first_key..first_key + count))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtraction::EvalStyle => parse_count(args, 2)
            .and_then(|count| args.get(3..3 + count))
            .unwrap_or(&[])
            .iter()
            .map(CompactArg::as_slice)
            .collect(),
        KeyExtraction::XReadStyle => args
            .iter()
            .position(|arg| arg.as_slice().eq_ignore_ascii_case(b"STREAMS"))
            .map(|streams_index| {
                let after_streams = &args[streams_index + 1..];
                let split = after_streams.len() / 2;
                after_streams[..split]
                    .iter()
                    .map(CompactArg::as_slice)
                    .collect()
            })
            .unwrap_or_default(),
        KeyExtraction::SortStore => {
            let mut keys = get_arg(args, 1).into_iter().collect::<Vec<_>>();
            let mut index = 2;
            while index + 1 < args.len() {
                if args[index].as_slice().eq_ignore_ascii_case(b"STORE") {
                    keys.push(args[index + 1].as_slice());
                    break;
                }
                index += 1;
            }
            keys
        }
    }
}

fn extract_channels(extractor: ChannelExtraction, args: &[CompactArg]) -> Vec<&[u8]> {
    match extractor {
        ChannelExtraction::None => Vec::new(),
        ChannelExtraction::First => get_arg(args, 1).into_iter().collect(),
        ChannelExtraction::AllFrom(index) => {
            args.iter().skip(index).map(CompactArg::as_slice).collect()
        }
    }
}

fn get_arg(args: &[CompactArg], index: usize) -> Option<&[u8]> {
    args.get(index).map(CompactArg::as_slice)
}

fn parse_count(args: &[CompactArg], index: usize) -> Option<usize> {
    std::str::from_utf8(args.get(index)?.as_slice())
        .ok()?
        .parse()
        .ok()
}
