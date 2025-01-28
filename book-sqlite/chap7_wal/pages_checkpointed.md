# Number of pages checkpointed pnCkpt

After checkpointing, SQLite returns `pnCkpt` number of pages checkpointed as a result.

It is one of the two fields that checkpoint returns
1. `pnLog` number of pages in wal
2. `pnCkpt` number of pages done moving to db file -> this one we are talking about

To rewrite these to Rust, we are looking at 
- how is `pnCkpt` value stored and changed
- what are the usages of this field

Conclusion: nbackfills in WalFileShared is equivalent to nBackfill. 
We can return this as checkpoint result.

## SQLite C code

`pnCkpt` is accessed through the field `nBackfill` of `WalCkptInfo` struct.

Reads
1. `walTryBeginRead()` uses nBackfill to select a particular WAL_READ_LOCK() that strives to let the checkpoint process do as much work as possible.
2. `if( pInfo->nBackfill<pWal->hdr.mxFrame )`
3. `assert( pInfo->nBackfill==pWal->hdr.mxFrame );`
4. `if( !useWal && AtomicLoad(&pInfo->nBackfill)==pWal->hdr.mxFrame`
5. `pWal->minFrame = AtomicLoad(&pInfo->nBackfill)+1`;
6. `assert( pInfo->nBackfill==pWal->hdr.mxFrame )`;
7. walRestartLog `if( pInfo->nBackfill>0 )`
8. read for checkpoint result `if( pnCkpt ) *pnCkpt = (int)(walCkptInfo(pWal)->nBackfill);`


Writes of nBackfill
1. `walIndexRecover()` reset checkpoint header `pInfo->nBackfill = 0;`
2. `walRestartHdr()` reset to 0 `AtomicStore(&pInfo->nBackfill, 0);`
3. `walCheckpoint()` in `AtomicStore(&pInfo->nBackfill, mxSafeFrame);`




```c
// wal.c
if( pnCkpt ) *pnCkpt = (int)(walCkptInfo(pWal)->nBackfill);

struct WalCkptInfo {
    // this is the field  
    u32 nBackfill;                  /* Number of WAL frames backfilled into DB */

    u32 aReadMark[WAL_NREADER];     /* Reader marks */
    u8 aLock[SQLITE_SHM_NLOCK];     /* Reserved space for locks */
    u32 nBackfillAttempted;         /* WAL frames perhaps written, or maybe not */
    u32 notUsed0;                   /* Available for future enhancements */
};
```

## mxSafeFrame

- `mxSafeFrame` is max frame in WAL that can be backfilled safely to db.
- frames beyond this index might overwrite database pages that are currently being read by active readers, which could lead to data inconsistency.

```c
// wal.c
// static int walCheckpoint()

/* Compute in mxSafeFrame the index of the last frame of the WAL that is
** safe to write into the database.  Frames beyond mxSafeFrame might
** overwrite database pages that are in use by active readers and thus
** cannot be backfilled from the WAL.
*/
mxSafeFrame = pWal->hdr.mxFrame; // init to highest frame index in Wal
mxPage = pWal->hdr.nPage;

// loop thru readers, for each one
// - get read mark atomically (last frame reader has processed)
// - process only if mxSafeFrame (wal last frame idx) is bigger than read mark
for(i=1; i<WAL_NREADER; i++){
    u32 y = AtomicLoad(pInfo->aReadMark+i); SEH_INJECT_FAULT;

    if( mxSafeFrame>y ){
        assert( y<=pWal->hdr.mxFrame );
        rc = walBusyLock(pWal, xBusy, pBusyArg, WAL_READ_LOCK(i), 1);

        if( rc==SQLITE_OK ){
            u32 iMark = (i==1 ? mxSafeFrame : READMARK_NOT_USED);
            AtomicStore(pInfo->aReadMark+i, iMark); SEH_INJECT_FAULT;
            walUnlockExclusive(pWal, WAL_READ_LOCK(i), 1);
        // if cannot lock the wal, a reader is using it.
        // we reduce the max safe frame to y.
        } else if( rc==SQLITE_BUSY ){
            mxSafeFrame = y;
            xBusy = 0;

        } else{
            goto walcheckpoint_out;
        }
    }
}
```

## Rust

```rust
/// WalFileShared is the part of a WAL that will be shared between threads. 
/// A wal has information that needs to be communicated between threads 
/// so this struct does the job.
pub struct WalFileShared {
    wal_header: Arc<RwLock<WalHeader>>,
    min_frame: u64,
    max_frame: u64,

    nbackfills: u64,
}
```

Reads
1. Wal.begin_read_tx(): 256 `self.min_frame = shared.nbackfills + 1;`
2. Wal.begin_read_tx(): 296 `self.min_frame = shared.nbackfills + 1;`


Writes of `nbackfills`:
1. Init: WalFileShared.open_shared() with `nbackfills: 0`
2. Wal.checkpoint(): `shared.nbackfills = 0;` if backfilled everything
3. Wal.checkpoint(): `shared.nbackfills = self.ongoing_checkpoint.max_frame;` if not backfilled everything