# Sorted Set Commands

Sorted sets associate each member with a floating-point score, keeping members ordered by score. All lookups by score or rank are O(log N).

## ZADD

```
ZADD key [NX | XX] [GT | LT] [CH] [INCR] score member [score member ...]
```

Add members with scores. Returns number of members added.

**Complexity:** O(log N) per member

```bash
ZADD leaderboard 1500 "alice" 1200 "bob" 1800 "charlie"

# Options:
ZADD lb NX 2000 "alice"   # only add if NOT exists
ZADD lb XX 2000 "alice"   # only update if EXISTS
ZADD lb GT 1400 "alice"   # only update if new score > current
ZADD lb LT 1400 "alice"   # only update if new score < current
ZADD lb CH 1600 "alice"   # return number of elements changed
ZADD lb INCR 100 "alice"  # increment score (like ZINCRBY)
```

## ZRANGE

```
ZRANGE key min max [BYSCORE | BYLEX] [REV] [LIMIT offset count] [WITHSCORES]
```

Return members by rank, score, or lex order. The unified command (v6.2+).

**Complexity:** O(log N + M) where M is the number of results

```bash
# By rank (0 = lowest score)
ZRANGE leaderboard 0 -1 WITHSCORES    # all, ascending
ZRANGE leaderboard 0 -1 REV WITHSCORES # all, descending (highest first)
ZRANGE leaderboard 0 2 WITHSCORES     # top 3 (ascending)

# By score
ZRANGE leaderboard 1000 2000 BYSCORE WITHSCORES
ZRANGE leaderboard "(1000" "+inf" BYSCORE  # exclusive lower bound

# With pagination
ZRANGE leaderboard 1400 2000 BYSCORE LIMIT 0 10
```

## ZRANGEBYSCORE / ZREVRANGEBYSCORE (legacy)

```
ZRANGEBYSCORE key min max [WITHSCORES] [LIMIT offset count]
ZREVRANGEBYSCORE key max min [WITHSCORES] [LIMIT offset count]
```

Legacy commands (prefer `ZRANGE ... BYSCORE`).

```bash
ZRANGEBYSCORE lb 1000 2000 WITHSCORES
ZRANGEBYSCORE lb -inf +inf LIMIT 0 10
ZRANGEBYSCORE lb "(1000" 2000   # exclusive: 1000 < score <= 2000
```

## ZREVRANGE (legacy)

```
ZREVRANGE key start stop [WITHSCORES]
```

Range by rank, descending. Legacy — prefer `ZRANGE ... REV`.

```bash
ZREVRANGE leaderboard 0 2 WITHSCORES   # top 3
```

## ZRANK / ZREVRANK

```
ZRANK key member [WITHSCORE]
ZREVRANK key member [WITHSCORE]
```

Return rank (0-indexed) of a member. `ZRANK` = ascending (0 = lowest), `ZREVRANK` = descending (0 = highest).

**Complexity:** O(log N)

```bash
ZRANK leaderboard "alice"      # 1 (0-indexed from lowest)
ZREVRANK leaderboard "alice"   # 1 (0-indexed from highest)
ZRANK leaderboard "alice" WITHSCORE  # 1 + score
ZRANK leaderboard "ghost"      # (nil)
```

## ZSCORE

```
ZSCORE key member
```

Returns the score of a member.

**Complexity:** O(1)

```bash
ZSCORE leaderboard "alice"   # "1600"
ZSCORE leaderboard "ghost"   # (nil)
```

## ZINCRBY

```
ZINCRBY key increment member
```

Increment the score of a member atomically.

**Complexity:** O(log N)

```bash
ZINCRBY leaderboard 100 "alice"   # "1700"
ZINCRBY leaderboard -50 "bob"     # "1150"
```

## ZCARD / ZCOUNT / ZLEXCOUNT

```
ZCARD key
ZCOUNT key min max
ZLEXCOUNT key min max
```

Count members.

**Complexity:** O(1) for ZCARD, O(log N) for ZCOUNT/ZLEXCOUNT

```bash
ZCARD leaderboard              # 3
ZCOUNT leaderboard 1400 2000   # 2 (members with score 1400–2000)
ZCOUNT leaderboard -inf +inf   # all
```

## ZREM / ZREMRANGEBYRANK / ZREMRANGEBYSCORE

```
ZREM key member [member ...]
ZREMRANGEBYRANK key start stop
ZREMRANGEBYSCORE key min max
```

Remove members by name, rank range, or score range.

**Complexity:** O(M log N) where M is elements removed

```bash
ZREM leaderboard "bob"             # 1

ZREMRANGEBYRANK leaderboard 0 1    # remove two lowest
ZREMRANGEBYSCORE leaderboard 0 999 # remove all with score < 1000
```

## ZPOPMIN / ZPOPMAX

```
ZPOPMIN key [count]
ZPOPMAX key [count]
```

Remove and return members with the lowest or highest scores.

**Complexity:** O(log N * M)

```bash
ZPOPMIN priority:queue 1    # dequeue highest priority (lowest score)
ZPOPMAX leaderboard 3       # top 3 players
```

## BZPOPMIN / BZPOPMAX

Blocking variants — wait for elements to be available.

```bash
BZPOPMIN priority:queue 30   # block up to 30 seconds
```

## ZUNIONSTORE / ZINTERSTORE / ZDIFFSTORE

```
ZUNIONSTORE destination numkeys key [key ...] [WEIGHTS weight ...] [AGGREGATE SUM | MIN | MAX]
ZINTERSTORE destination numkeys key [key ...] [WEIGHTS weight ...] [AGGREGATE SUM | MIN | MAX]
ZDIFFSTORE destination numkeys key [key ...]
```

Set operations that produce a new sorted set.

**Complexity:** O(N*K + M log M)

```bash
# Combine two leaderboards, summing scores
ZUNIONSTORE combined 2 lb:week lb:alltime AGGREGATE SUM

# Weighted combination
ZUNIONSTORE combined 2 lb:week lb:alltime WEIGHTS 2 1

# Intersection — only users who appear in both
ZINTERSTORE active 2 lb:week users:premium
```

## ZMSCORE / ZRANDMEMBER

```
ZMSCORE key member [member ...]
ZRANDMEMBER key [count [WITHSCORES]]
```

Batch score lookup and random member selection.

```bash
ZMSCORE leaderboard alice bob ghost
# "1700" "1150" (nil)

ZRANDMEMBER leaderboard 2 WITHSCORES
```
