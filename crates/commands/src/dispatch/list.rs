use super::registry::with_command_registry;

macro_rules! generate_command_ids {
    (
        $(
            $len:literal => {
                $(
                    $first:expr => {
                        $(
                            {
                                variant: $variant:ident,
                                bytes: $bytes:expr,
                                dispatch: [ $($dispatch:tt)* ],
                                supported: $supported:tt,
                                group: $group:literal,
                                shape: ($arity:expr, $first_key:expr, $last_key:expr, $step:expr),
                                readonly: $readonly:tt,
                                write: $write:tt,
                                auth: $auth_kind:ident $( {
                                    categories: $categories:expr,
                                    keys: $keys:expr,
                                    channels: $channels:expr,
                                } )?,
                                notify: $notify_kind:ident $( {
                                    event: $event:expr,
                                    class: $class:expr,
                                    keys: $notify_keys:expr,
                                    response: $response:expr,
                                } )?,
                            }
                        )*
                    }
                )*
            }
        )*
    ) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum CommandId {
            Unknown,
            $( $( $( $variant, )* )* )*
        }

        impl CommandId {
            pub const ALL: &'static [Self] = &[
                $( $( $( Self::$variant, )* )* )*
            ];

            #[inline(always)]
            pub fn name(self) -> Option<&'static str> {
                self.name_bytes().map(|bytes| unsafe { std::str::from_utf8_unchecked(bytes) })
            }

            #[inline(always)]
            pub fn name_bytes(self) -> Option<&'static [u8]> {
                Some(match self {
                    Self::Unknown => return None,
                    $( $( $( Self::$variant => $bytes, )* )* )*
                })
            }
        }
    };
}

with_command_registry!(generate_command_ids);
