# SUBSCRIBE

Uses the pub/sub surface for channel-oriented messaging.

## Syntax

```text
SUBSCRIBE <channel> [channel ...]
```

## Parameters

- `channel`: Pub/Sub channel name.

## Returns

Returns the standard Redis pub/sub acknowledgement frame or an implementation-specific unsupported error for commands that are not active yet.

## Examples

```bash
SUBSCRIBE metrics logs
```

## BetterKV Notes

- Group: Pub/Sub.
- Access: Declared.
- Status: This command is present in the registry, but the current server implementation still routes it through an unsupported handler. Document it as part of the public surface, but expect a not-supported response until the runtime implementation lands.
