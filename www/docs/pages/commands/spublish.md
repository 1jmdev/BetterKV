# SPUBLISH

Uses the pub/sub surface for channel-oriented messaging.

## Syntax

```text
SPUBLISH <channel> <message>
```

## Parameters

- `channel`: Pub/Sub channel name.
- `message`: Message payload to echo or publish.

## Returns

Returns the standard Redis pub/sub acknowledgement frame or an implementation-specific unsupported error for commands that are not active yet.

## Examples

```bash
SPUBLISH shard:events "node-1 up"
```

## BetterKV Notes

- Group: Pub/Sub.
- Access: Declared.
- Status: This command is present in the registry, but the current server implementation still routes it through an unsupported handler. Document it as part of the public surface, but expect a not-supported response until the runtime implementation lands.
