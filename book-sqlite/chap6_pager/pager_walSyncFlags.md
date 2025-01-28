# Pager walSyncFlags

```c
struct Pager {
    u8 walSyncFlags;
}
```

- syncFlags used for rollback mode.
- walSyncFlags used for WAL mode.

**walSyncFlags** contains the flags used to 
- sync the checkpoint operations in the lower two bits.
- sync flags used for txn commits in WAL file in bits 0x04 and 0x08.

In other words, to get the correct sync flags
for checkpoint operations, use `(walSyncFlags&0x03)` and to get the correct
sync flags for transaction commit, use `((walSyncFlags>>2)&0x03)`.

Note that with `synchronous=NORMAL` in WAL mode, transaction commit is not synced
meaning that the 0x04 and 0x08 bits are both zero.

in Rust
```rust
struct Pager {
    
}
```