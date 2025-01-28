# Transaction Commit

``` 
Transaction Management
    Normal processing
        Read operation
        Write operation
        Cache flush
        Commit operation -> we're here
        Statement operations 
        Setting up savepoints
        Releasing savepoints
```

we take into account only the single database file in WAL scenario.
In WAL mode, we still need to differentiate between write and read transactions because WAL allows for multiple readers and only one writer concurrently.

Steps
1. tree module is ready to commit a transaction
2. it calls `sqlite3PagerCommitPhaseOne` function first
3. then `sqlite3PagerCommitPhaseTwo` function

Committing read txn
- releasing shared lock from db file
- returns NO_LOCK state
- does not purge page cache

Committing write txn
- todo

```c 
int sqlite3PagerCommitPhaseOne(
  Pager *pPager,                  /* Pager object */
  const char *zSuper,            /* If not NULL, the super-journal name */
  int noSync                      /* True to omit the xSync on the db file */
){
    if( pagerUseWal(pPager) ){
      PgHdr *pPageOne = 0;
      pList = sqlite3PcacheDirtyList(pPager->pPCache);
      if( pList==0 ){
        /* Must have at least one page for the WAL commit flag.*/
        rc = sqlite3PagerGet(pPager, 1, &pPageOne, 0);
        pList = pPageOne;
        pList->pDirty = 0;
      }
      assert( rc==SQLITE_OK );
      if( ALWAYS(pList) ){
        rc = pagerWalFrames(pPager, pList, pPager->dbSize, 1);
      }
      sqlite3PagerUnref(pPageOne);
      if( rc==SQLITE_OK ){
        sqlite3PcacheCleanAll(pPager->pPCache);
      }
    }
}
```