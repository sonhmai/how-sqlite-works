# number of pages in wal

Checkpoint result
1. `pnLog` number of pages in wal
2. `pnCkpt` number of pages done moving to db file

## mxFrame

In SQLite, checkpoint results number of pages in WAL. This value is taken from the field `mxFrame` of `WalIndexHdr`. It holds the index of last valid frame in the WAL for current transaction.

```c
*pnLog = (int)pWal->hdr.mxFrame;
```



is `mxFrame` in SQLite and `max_frame` in Rust the same?


C
```c
// wal.c
int sqlite3WalCheckpoint(
  Wal *pWal,                      /* Wal connection */
  sqlite3 *db,                    /* Check this handle's interrupt flag */
  int eMode,                      /* PASSIVE, FULL, RESTART, or TRUNCATE */
  int (*xBusy)(void*),            /* Function to call when busy */
  void *pBusyArg,                 /* Context argument for xBusyHandler */
  int sync_flags,                 /* Flags to sync db file with (or 0) */
  int nBuf,                       /* Size of temporary buffer */
  u8 *zBuf,                       /* Temporary buffer to use */
  // output
  int *pnLog,                     /* OUT: Number of frames in WAL */
  int *pnCkpt                     /* OUT: Number of backfilled frames in WAL */
){
  // ...
  if( pnLog ) *pnLog = (int)pWal->hdr.mxFrame;
  if( pnCkpt ) *pnCkpt = (int)(walCkptInfo(pWal)->nBackfill);
  // ...
}

struct Wal {
  WalIndexHdr hdr;           /* Wal-index header for current transaction */
  u32 minFrame;              /* Ignore wal frames before this one */
}

struct WalIndexHdr {
  u32 mxFrame;                    /* Index of last valid frame in the WAL */
  // ...
};

// usages of mxFrame

int WalIndexRecover(Wal *pWal) {
    pInfo->nBackfillAttempted = pWal->hdr.mxFrame;
    pInfo->aReadMark[i] = pWal->hdr.mxFrame;
    sqlite3_log(SQLITE_NOTICE_RECOVER_WAL,
          "recovered %d frames from WAL file %s",
          pWal->hdr.mxFrame, pWal->zWalName
      );
}

// Updates shared-memory structures so that next client to write to db
// (which may be this one) does so by writing frames into start of log file.
void walRestartHdr(Wal *wal, u32 salt1) {
    pWal->hdr.mxFrame = 0;
}

int walCheckpoint(...) {
    if( pInfo->nBackfill<pWal->hdr.mxFrame ){
        // ...
        /* Compute in mxSafeFrame the index of the last frame of the WAL that is
        ** safe to write into the database.  Frames beyond mxSafeFrame might
        ** overwrite database pages that are in use by active readers and thus
        ** cannot be backfilled from the WAL.
        */
        mxSafeFrame = pWal->hdr.mxFrame;
    }   
}

/* Attempt to start a read txn.
** Reader will use WAL frames up to and including pWal->hdr.mxFrame 
** if pWal->readLock>0.
** Or if pWal->readLock==0, then the reader will ignore the WAL
** completely and get all content directly from the database file.
*/
int walTryBeginRead(Wal *pWal, int *pChanged, int useWal, int cnt) {
    
}
```


Rust 
```rust
struct WalFile {
  shared: Arc<RwLock<WalFileShared>>,
  /// This is the index to the read_lock in WalFileShared that we are holding. 
  /// This lock contains the max frame for this connection.
  max_frame_read_lock_index: usize,
  /// Max and min frame allowed to lookup range=(min_frame..max_frame)
  max_frame: u64,
  min_frame: u64,
}

fn Wal.begin_read_tx() {
  self.maxframe = max_read_mark as u64;
}

/// Find latest frame containing a page.
fn Wal.find_frame(&self, page_id: u64) {
  //...
  for frame in frames.iter().rev() {
      if *frame <= self.max_frame {
          return Ok(Some(*frame));
      }
  }
  Ok(None)
}

fn Wal.get_max_frame() -> u64 {
  self.max_frame
}

// get_max_frame usages
Pager.allocate_page()
  PageCacheKey::new(page.get().id, Some(self.wal.borrow().get_max_frame()));
Pager.read_page()
  PageCacheKey::new(page_idx, Some(self.wal.borrow().get_max_frame()))
Pager.cacheflush()
  PageCacheKey::new(*page_id, Some(self.wal.borrow().get_max_frame()))
Pager.load_page()
Pager.put_loaded_page()
  PageCacheKey::new(id, Some(self.wal.borrow().get_max_frame()))

/// include the max_frame that a connection will read from so that if two
// connections have different max_frames, they might read different wal frames.
pub struct PageCacheKey {
  pgno: usize,
  max_frame: Option<u64>,
}

pub struct DumbLruPageCache {
  // different max_frame for the same page results in different key
  map: RefCell<HashMap<PageCacheKey, NonNull<PageCacheEntry>>>,
}

/// Part of WAL shared btw threads.
/// One difference between SQLite and limbo: never support multi process, 
/// meaning we don't need WAL's index file. 
/// So we can do stuff like this without shared memory.
struct WalFileShared {
  min_frame: u64,
  max_frame: u64,
  nbackfills: u64,
}
```


