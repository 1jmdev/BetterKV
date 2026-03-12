# RANDOMKEY

Works at the keyspace layer rather than on a single concrete data type.

## Syntax

```text
RANDOMKEY
```

## Parameters

- This command does not require any positional arguments.

## Returns

Usually returns one value or `nil` when the requested key, field, or element does not exist.

## Examples

```bash
RANDOMKEY
```

## BetterKV Notes

- Group: Keyspace.
- Access: Read-only.
- Status: Implemented in BetterKV and safe to use for read paths.
