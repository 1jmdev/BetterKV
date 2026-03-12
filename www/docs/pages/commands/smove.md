# SMOVE

Works with set values.

## Syntax

```text
SMOVE <source> <destination> <member>
```

## Parameters

- `source`: Source key.
- `destination`: Key that receives the computed result.
- `member`: Set, sorted-set, or geo member.

## Returns

Returns a Redis-compatible reply whose exact shape depends on the command and selected options.

## Examples

```bash
SMOVE backlog processing job-42
```

## BetterKV Notes

- Group: Set.
- Access: Write.
- Status: Implemented in BetterKV and mutates server state or stored data.
