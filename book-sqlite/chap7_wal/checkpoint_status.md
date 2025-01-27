# Checkpoint status

- SQLITE_BUSY
- SQLITE_OK


## Status DONE

All calls obtain an exclusive "checkpoint" lock on the database file. 
If any other process is running a checkpoint operation at the same time, the lock cannot be obtained and SQLITE_BUSY is returned. Even if there is a busy-handler configured, it will not be invoked in this case.

Pager.checkpoint is not synced so we should call something sync in Connection.checkpoint.
Then store state of num pages in WAL and return the checkpoint result from accessing Pager.wal.

```
Usages
1. Connection.close
2. Pager.checkpoint
3. Pager.cacheflush
4. Pager.end_tx
5. Pager.clear_page_cache
6. Program.step
7. integration test

returning `CheckpointStatus::DONE`
1. Pager.cacheflush -> Result<CheckpointStatus>
2. WalFile.sync -> Result<CheckpointStatus>
3. WalFile.checkpoint -> Result<CheckpointStatus>
4. Pager.end_tx -> Result<CheckpointStatus>
```


```rust
// lib.rs: Connection.close
    pub fn close(&self) -> Result<()> {
        loop {
            // TODO: make this async?
            match self.pager.checkpoint()? {
                CheckpointStatus::Done => {
                    return Ok(());
                }
                CheckpointStatus::IO => {
                    self.pager.io.run_once()?;
                }
            };
        }
    }
```

```rust
// Pager.checkpoint

```

```rust
// Program.step
Insn::Halt {..} => 
    return if self.auto_commit {
        match pager.end_tx() {
            Ok(crate::storage::wal::CheckpointStatus::IO) => Ok(StepResult::IO),
            Ok(crate::storage::wal::CheckpointStatus::Done) => {
                if self.change_cnt_on {
                    if let Some(conn) = self.connection.upgrade() {
                        conn.set_changes(self.n_change.get());
                    }
                }
                Ok(StepResult::Done)
            }
            Err(e) => Err(e),
        }
    } else {
        if self.change_cnt_on {
            if let Some(conn) = self.connection.upgrade() {
                conn.set_changes(self.n_change.get());
            }
        }
        return Ok(StepResult::Done);
    };
```