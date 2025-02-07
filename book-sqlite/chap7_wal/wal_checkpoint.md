# WAL Checkpoint

Contents
1. [checkpoint api](#checkpoint_api)
2. from pragma to checkpoint
3. checkpoint opcode
4. examples

Why need `checkpoint`?
- When the DBMS restarts a database upon a failure, the very first thing it needs to do is to recover the database. 
- The only recovery information available is the journal content that has survived the failure. 
- The DBMS may need to read all log records from the journal and process the records against the database. 
- Consequently, the recovery becomes a time consuming operation, especially when the journal contains a large number of log records. 
- To reduce recovery time at restarts, the DBMS checkpoints the database periodically. 

What is a `checkpoint`?
- A checkpoint is a kind of synchronization point between the database and the recovery subsystem. 
- A checkpoint establishes a relatively recent database state and journal content and these can be used as a basis for recovery at future restarts. 
- Checkpoints are taken solely to cut down restart processing time. 
- In essence, a checkpoint helps to eliminate some old log records from the journal 
- -> it helps speeding up restart processing upon system failures.

`checkpoint` in WAL journal mode
- Unlike the rollback journaling scheme , the wal journaling scheme needs checkpointing to keep the journal file size in check.
- SQLite automatically performs a checkpoint when the wal journal file reaches a threshold size of 1000 pages.
- Checkpoint operations are performed sequentially, mutually exclusively.
- On each call to checkpointing function, SQLite do following steps
  1. First , it flushes the wal journal file.
  2. Second, it transfers some valid page contents to the database file.
  3. Third, it flushes the database file ( only if the entire wal journal is copied into the database file).
  4. Fourth, the `salt-1` component of the wal file header is incremented and the salt-2 is randomized (to invalidate the current page log images in the wal journal).
  5. Fifth, update the wal-index.

checkpoint concurrency
- a checkpoint operation execution can run concurrently with read-transactions.
- the checkpoint stops on or before reading the `wal-mark` of any read-transaction (see below).
- checkpoint remembers how far it has checkpointed; the next checkpoint restarts from there.
- When the entire wal journal is checkpointed, the journal is rewind to prevent the journal file to grow without bound.



## checkpoint_api 

```c
// main.c
// checkpoint database zDb
int sqlite3_wal_checkpoint_v2(
  sqlite3 *db,                    /* Database handle */
  const char *zDb,                /* Name of attached database (or NULL) */
  int eMode,                      /* SQLITE_CHECKPOINT_* value */
  int *pnLog,                     /* OUT: Size of WAL log in frames */
  int *pnCkpt                     /* OUT: Total number of frames checkpointed */
){
  sqlite3_mutex_enter(db->mutex); // lock db
  rc = sqlite3Checkpoint(db, iDb, eMode, pnLog, pnCkpt);
  sqlite3_mutex_leave(db->mutex); // unlock db
}

// still in main.c
// Run a check point on db index iDb (there can be many dbs in sqlite storing in db->nDb)
int sqlite3Checkpoint(sqlite3 *db, int iDb, int eMode, int *pnLog, int *pnCkpt){
  assert( sqlite3_mutex_held(db->mutex) );
  rc = sqlite3BtreeCheckpoint(db->aDb[i].pBt, eMode, pnLog, pnCkpt);
}

// btree.c
// Run a checkpoint on the Btree passed as the first argument.
// Return SQLITE_LOCKED if this or any other connection has an open txn on shared-cache.
int sqlite3BtreeCheckpoint(Btree *p, int eMode, int *pnLog, int *pnCkpt){
  BtShared *pBt = p->pBt;
  sqlite3BtreeEnter(p); 
  rc = sqlite3PagerCheckpoint(pBt->pPager, p->db, eMode, pnLog, pnCkpt); 
}

// pager.c
// called when the user invokes "PRAGMA wal_checkpoint",
// "PRAGMA wal_blocking_checkpoint" or calls the sqlite3_wal_checkpoint()
// or wal_blocking_checkpoint() API functions.
int sqlite3PagerCheckpoint(
  Pager *pPager,                  /* Checkpoint on this pager */
  sqlite3 *db,                    /* Db handle used to check for interrupts */
  int eMode,                      /* Type of checkpoint */
  int *pnLog,                     /* OUT: Final number of frames in log */
  int *pnCkpt                     /* OUT: Final number of checkpointed frames */
){
  rc = wal.sqlite3WalCheckpoint(
    pPager->pWal, 
    db, 
    eMode,
    (eMode==SQLITE_CHECKPOINT_PASSIVE ? 0 : pPager->xBusyHandler),
    pPager->pBusyHandlerArg,
    pPager->walSyncFlags, 
    pPager->pageSize, 
    (u8 *)pPager->pTmpSpace,
    // output
    pnLog, 
    pnCkpt
    );
}

// wal.c
// Obtain a CHECKPOINT lock and then backfill as much information as
// we can from WAL into the database.
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
  // lock
  // read wal-index header
  // copy data from log to db file
  rc = walCheckpoint(pWal, db, eMode2, xBusy2, pBusyArg, sync_flags,zBuf);

  // set output
  if( pnLog ) *pnLog = (int)pWal->hdr.mxFrame;
  if( pnCkpt ) *pnCkpt = (int)(walCkptInfo(pWal)->nBackfill);
  // unlock ...
}

// still in wal.c - calling private function.
// - Copy as much content as we can from the WAL back into the database file.
// - This routine will never overwrite a database page that a concurrent reader might be using.
// - Fsync is also called on the database file if (and only if) the entire WAL content is copied into the database file. 
// - All I/O barrier operations (a.k.a fsyncs) occur in this routine 
// when SQLite is in WAL-mode in synchronous=NORMAL.
// - Callers must hold sufficient locks to ensure no other checkpoint is running.
static int walCheckpoint(
  Wal *pWal,                      /* Wal connection */
  sqlite3 *db,                    /* Check for interrupts on this handle */
  int eMode,                      /* One of PASSIVE, FULL or RESTART */
  int (*xBusy)(void*),            /* Function to call when busy */
  void *pBusyArg,                 /* Context argument for xBusyHandler */
  int sync_flags,                 /* Flags for OsSync() (or 0) */
  u8 *zBuf                        /* Temporary buffer to use */
){
  // Compute in mxSafeFrame the index of the last frame of the WAL that is
  // safe to write into the database.  Frames beyond mxSafeFrame might
  // overwrite database pages that are in use by active readers and thus
  // cannot be backfilled from the WAL.

  // allocate iterator
  rc = walIteratorInit(pWal, pInfo->nBackfill, &pIter);

  // sync WAL to disk
  rc = os.sqlite3OsSync(pWal->pWalFd, CKPT_SYNC_FLAGS(sync_flags));

  // Iterate through the contents of the WAL, copying data to the db file
  while( rc==SQLITE_OK && 0==walIteratorNext(pIter, &iDbpage, &iFrame) ){
    // iFrame: u32, wal frame contains data for iDbpage
    // szPage: int, database page size

    rc = sqlite3OsRead(pWal->pWalFd, zBuf, szPage, iOffset);
    rc = sqlite3OsWrite(pWal->pDbFd, zBuf, szPage, iOffset);
  }

  // Release the reader lock held while backfilling.
  walUnlockExclusive(pWal, WAL_READ_LOCK(0), 1);
}

// the call chain stops here. 
// the above routine walCheckpoint persisted data to disk.
```

## from pragma to c apis

https://sqlite.org/pragma.html#pragma_wal_checkpoint

- checkpoint runs on `db: sqlite3` which is the database connection current Vdbe program is holding.
- the equivalient in Rust is `Connection` struct
- need to add `checkpoint` method to `Connection`
- it is available, it calls `self.pager.clear_page_cache()`

```c
// case OP_Checkpoint bytecode execution
rc = main.sqlite3Checkpoint(db, pOp->p1, pOp->p2, &aRes[1], &aRes[2]);

// vdbe.c executing a program (sqlite3_step)
int sqlite3VdbeExec(Vdbe *p) {
  sqlite3 *db = p->db; // the db
}

// vdbe.h
struct Vdbe {
  sqlite3 *db;            /* The database connection that owns this statement */
}
```


```c

// parser

// add bytecode

// bytecode execution vdbe.c
// Opcode: CHECKPOINT P1 P2 P3 * *
case OP_Checkpoint: {
  int i;                          /* Loop counter */
  int aRes[3];                    /* Results */
  Mem *pMem;                      /* Write results here */

  assert( vdbePointer->readOnly==0 );
  aRes[0] = 0;
  aRes[1] = aRes[2] = -1;
  assert( pOp->p2==SQLITE_CHECKPOINT_PASSIVE
       || pOp->p2==SQLITE_CHECKPOINT_FULL
       || pOp->p2==SQLITE_CHECKPOINT_RESTART
       || pOp->p2==SQLITE_CHECKPOINT_TRUNCATE
  );

  // db=db - the database
  // iDb=pOp->p1 database index
  // eMode=pOp->p2 - enum WAL mode: one of SQLITE_CHECKPOINT_PASSIVE, FULL, RESTART or TRUNCATE
  // int *pnLog=&aRes[1] - size of WAL log in frames
  // int *pnCkpt=&aRes[2] - total number of frames checkpointed
  rc = main.sqlite3Checkpoint(db, pOp->p1, pOp->p2, &aRes[1], &aRes[2]);
  if( rc ){
    if( rc!=SQLITE_BUSY ) goto abort_due_to_error;
    rc = SQLITE_OK;
    aRes[0] = 1;
  }
  for(i=0, pMem = &aMem[pOp->p3]; i<3; i++, pMem++){
    sqlite3VdbeMemSetInt64(pMem, (i64)aRes[i]);
  }   
  break;
};  

// main.c
// this is the 2nd call in the api section
int sqlite3Checkpoint(sqlite3 *db, int iDb, int eMode, int *pnLog, int *pnCkpt){
  // ...
}

// call to btree module, the other follows
```


## checkpoint opcode

### where is opcode CHECKPOINT stored and processed?

Opcode: CHECKPOINT P1 P2 P3 * *
- https://www.sqlite.org/opcode.html
- P1: int iDb. Checkpoint database P1. This is a no-op if P1 is not currently in WAL mode. 
- P2: int. enum eMode, one of SQLITE_CHECKPOINT_PASSIVE, FULL, RESTART, or TRUNCATE. 
- Write 1 or 0 into mem[P3] if the checkpoint returns SQLITE_BUSY or not, respectively. 
- Write the number of pages in the WAL after the checkpoint into mem[P3+1] and the number of pages in the WAL that have been checkpointed after the checkpoint completes into mem[P3+2]. 
- However on an error, mem[P3+1] and mem[P3+2] are initialized to -1.

``` 
SQLite version 3.41.1 2023-03-10 12:13:52
Enter ".help" for usage hints.
sqlite> explain pragma wal_checkpoint;
addr  opcode         p1    p2    p3    p4             p5  comment
----  -------------  ----  ----  ----  -------------  --  -------------
0     Init           0     5     0                    0   Start at 5
1     Expire         1     1     0                    0
2     Checkpoint     12    0     1                    0
3     ResultRow      1     3     0                    0   output=r[1..3]
4     Halt           0     0     0                    0
5     Goto           0     1     0                    0
```


## Examples

Test that doing a checkpoint while there is a txn lock returns SQLITE_BUSY

```shell
sqlite3 sqlite3.db

# pragma on empty db should be 0,0,0
pragma wal_checkpoint;
0|0|0

create table t1 (id int primary key, name text);
insert into t1 (id, name) values (1, 'one'), (2, 'two'), (3, 'three');
select * from t1;
1|one
2|two
3|three

# db file is still 4K, data sits only in WAL
-rw-r--r--    1 sonmh  1275159865    4K Jan 17  09:00 sqlite3.db
-rw-r--r--    1 sonmh  1275159865    32K Jan 17 09:00 sqlite3.db-shm
-rw-r--r--    1 sonmh  1275159865    20K Jan 17 09:06 sqlite3.db-wal

# 1st col is 
#   0 if sqlite3_wal_checkpoint_v2() returns SQLITE_OK
#   1 if it returns SQLITE_BUSY. A RESTART or FULL or TRUNCATE checkpoint was blocked from completing 
#   (e.g. ongoing txn from another thread/ process).
# 2nd col: # pages written to wal file.
# 3rd col: # pages moved to db file successfully.
pragma wal_checkpoint;
0|5|5

# db file is now 12K, data applied from WAL to db
-rw-r--r--    1 sonmh  1275159865    12K Jan 17 09:08 sqlite3.db
-rw-r--r--    1 sonmh  1275159865    32K Jan 17 09:00 sqlite3.db-shm
-rw-r--r--    1 sonmh  1275159865    20K Jan 17 09:06 sqlite3.db-wal

# checkpoint again, output the same
pragma wal_checkpoint;
0|5|5

sqlite> begin transaction;
sqlite> insert into t1 (id, name) values (4, 'four');
sqlite> update t1 set name='two1' where id=2;

# checkpoint on client2
sqlite> pragma wal_checkpoint;
0|5|5

# client1
sqlite> commit;
sqlite> select * from t1;
1|one
2|two1
3|three
4|four

# client2
sqlite> pragma wal_checkpoint;
0|2|2
```

## checkpoint result

1. `pnLog` number of pages in wal
2. `pnCkpt` number of pages done moving to db file

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

struct WalIndexHdr {
  u32 mxFrame;                    /* Index of last valid frame in the WAL */
  // ...
};

struct Wal {
  WalIndexHdr hdr;           /* Wal-index header for current transaction */
  u32 minFrame;              /* Ignore wal frames before this one */
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
















