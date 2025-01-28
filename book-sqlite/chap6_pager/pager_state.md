


Pager CheckpointState
1. Checkpoint
2. SyncDbFile
3. WaitSyncDbFile
4. CheckpointDone

```rust
// checkpoint_inflight where-used
// = write_counter in Wal.checkpoint(pager, write_counter, mode)

struct Pager {
    checkpoint_inflight: Rc<RefCell<usize>>
}

// init
Pager.finish_open -> Result<Self> {
    Self {
        checkpoint_inflight: Rc::new(RefCell::new(0))
    }
}

Pager.checkpoint -> Result<CheckpointStatus> {
    CheckpointState::Checkpoint {
        let in_flight = self.checkpoint_inflight.clone();
        self.wal.borrow_mut().checkpoint(self, in_flight, CheckpointMode::Passive)
    }
}

```

## Pager state eState

State of pager in `Pager.eState`
1. OPEN
2. READER
3. WRITER_LOCKED
4. WRITER_CACHEMOD
5. WRITER_DBMOD
6. WRITER_FINISHED
7. ERROR

``` 
** The Pager.eState variable stores the current 'state' of a pager. A
** pager may be in any one of the seven states shown in the following
** state diagram.
**
**                            OPEN <------+------+
**                              |         |      |
**                              V         |      |
**               +---------> READER-------+      |
**               |              |                |
**               |              V                |
**               |<-------WRITER_LOCKED------> ERROR
**               |              |                ^ 
**               |              V                |
**               |<------WRITER_CACHEMOD-------->|
**               |              |                |
**               |              V                |
**               |<-------WRITER_DBMOD---------->|
**               |              |                |
**               |              V                |
**               +<------WRITER_FINISHED-------->+
```

### READER

- A read transaction may be active (but a write-transaction cannot).
- The dbSize variable may be trusted (even if a user-level read
  transaction is not active). 
- The dbOrigSize and dbFileSize variables may not be trusted at this point.
- If the database is a WAL database, then the WAL connection is open.

### WRITER_LOCKED

The pager moves to this state from `READER` when a write-transaction is first opened on the database.

In this state
- all locks required to start a write-transaction are held.
- but no actual modifications to cache or db have take place.
- write txn is active.
- If the connection is open in WAL-mode, a WAL write transaction
  is open (i.e. sqlite3WalBeginWriteTransaction() has been successfully called).
- The dbSize, dbOrigSize and dbFileSize variables are all valid.
- The contents of the pager cache have not been modified.
- The journal file may or may not be open.
- Nothing (not even the first header) has been written to the journal.

In WAL mode
- `WalBeginWriteTransaction` is called to lock log file.
- If the connection is running with locking_mode=exclusive, an attempt
  is made to obtain an EXCLUSIVE lock on the database file.

### WRITER_DBMOD (not applied for WAL)

The pager transitions from WRITER_CACHEMOD into WRITER_DBMOD state when it modifies the contents of the database file. 

`WAL connections` never enter this state (since they do not modify the database file, just the log file).

