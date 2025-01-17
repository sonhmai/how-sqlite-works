# WAL Checkpoint

## checkpoint api 

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
  rc = sqlite3WalCheckpoint(pPager->pWal, db, eMode,
        (eMode==SQLITE_CHECKPOINT_PASSIVE ? 0 : pPager->xBusyHandler),
        pPager->pBusyHandlerArg,
        pPager->walSyncFlags, pPager->pageSize, (u8 *)pPager->pTmpSpace,
        pnLog, pnCkpt
    );
}

// still in pager.c
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
  int *pnLog,                     /* OUT: Number of frames in WAL */
  int *pnCkpt                     /* OUT: Number of backfilled frames in WAL */
){
  // lock
  // read wal-index header
  // copy data from log to db file
  rc = walCheckpoint(pWal, db, eMode2, xBusy2, pBusyArg, sync_flags,zBuf);
  // unlock
}

// wal.c
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

## path from pragma to c apis

```c

// parser

// add bytecode

// bytecode execution vdbe.c
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

opcode: CHECKPOINT
- https://www.sqlite.org/opcode.html
- Checkpoint database P1. This is a no-op if P1 is not currently in WAL mode. 
- Parameter P2 is one of SQLITE_CHECKPOINT_PASSIVE, FULL, RESTART, or TRUNCATE. 
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

```
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



















