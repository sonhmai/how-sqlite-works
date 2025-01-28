# Pager Checkpoint Result

### How to return checkpoint result from WAL back to Pager?



### Flags for WAL mode

```c 
void sqlite3PagerSetFlags(
  Pager *pPager,        /* The pager to set safety level for */
  unsigned pgFlags      /* Various flags */
)
```

1. OFF: no sync ever occurs
2. NORMAL:
    - WAL synced prior to start of checkpoint
    - and db file is synced at the conclusion of checkpoint if entire content of WAL was written back to db.
    - But not sync ops occur for an ordinary commit in NORMAL mode with WAL.
3. FULL:
    - WAL file is synced following each commit op, in addition to `NORMAL` syncs.

### What does Pager do when Wal.checkpoint returns SQL_BUSY?