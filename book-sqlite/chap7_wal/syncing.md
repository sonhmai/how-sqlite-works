
```rust
struct WalFile {
    syncing: Rc<RefCell<bool>>
}
```

Usages
1. init in WalFile.new
2. WalFile.sync

```rust
fn sync() -> Result<CheckpointStatus> {
    let state = *self.sync_state.borrow();
    match state {
        SyncState::NotSyncing => {
            let syncing = self.syncing.clone();
            *syncing.borrow_mut() = true; // change the value
        }
        SyncState::Syncing => {
            if *self.syncing.borrow() {
                Ok(CheckpointStatus::IO)
            } else {
                self.sync_state.replace(SyncState::NotSyncing);
                Ok(CheckpointStatus::Done)
            }
        }
    }
}
```