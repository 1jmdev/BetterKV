# Transactions

BetterKV transactions let you queue multiple commands and execute them atomically.

## MULTI / EXEC

```bash
MULTI          # begin transaction
SET foo "bar"  # queued
INCR counter   # queued
LPUSH list "a" # queued
EXEC           # execute all atomically
```

While in a transaction, commands return `QUEUED` instead of executing. `EXEC` runs them all at once.

### Discard a Transaction

```bash
MULTI
SET foo "oops"
DISCARD        # cancel — nothing executed
```

## Optimistic Locking with WATCH

`WATCH` enables check-and-set (CAS) semantics. If a watched key changes before `EXEC`, the transaction is aborted.

```bash
WATCH balance:alice

MULTI
  SET balance:alice 900
  SET balance:bob 1100
result = EXEC

if result is nil:
    # Transaction aborted — retry
```

### Real Example: Transfer Funds

```js title="transfer.js"
async function transfer(client, from, to, amount) {
  const fromKey = `balance:${from}`;
  const toKey = `balance:${to}`;

  while (true) {
    await client.watch(fromKey, toKey);

    const fromBalance = Number(await client.get(fromKey));
    if (fromBalance < amount) {
      await client.unwatch();
      throw new Error('Insufficient funds');
    }

    const toBalance = Number(await client.get(toKey));

    const result = await client
      .multi()
      .set(fromKey, fromBalance - amount)
      .set(toKey, toBalance + amount)
      .exec();

    if (result !== null) {
      return result; // success
    }
    // null means WATCH detected a change — retry
  }
}
```

## Error Handling

Transactions have two types of errors:

**Syntax errors** (before EXEC) — entire transaction is discarded:
```bash
MULTI
SET foo bar
NOTACOMMAND   # error — transaction aborted
EXEC
# (error) EXECABORT
```

**Runtime errors** (during EXEC) — other commands still execute:
```bash
MULTI
SET foo "hello"
INCR foo       # runtime error (foo is not an integer)
SET bar "world"
EXEC
# 1) OK
# 2) (error) ERR value is not an integer
# 3) OK
```

:::warning
BetterKV does **not** roll back on runtime errors inside `EXEC`. If you need rollback behavior, implement it with Lua scripting instead.
:::

## Transactions vs Lua Scripts

| Feature | MULTI/EXEC | Lua Script |
|---------|-----------|------------|
| Atomicity | Yes | Yes |
| Conditional logic | No (via WATCH) | Yes |
| Rollback on error | No | Partial (pcall) |
| Network round trips | 1 (pipeline) | 1 |
| Performance | Fast | Faster (9x vs Redis) |

For complex conditional logic, prefer **Lua scripts**. Use `MULTI/EXEC` for simple atomic batches.
