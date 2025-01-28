

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