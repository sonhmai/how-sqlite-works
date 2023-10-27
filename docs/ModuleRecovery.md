# Recovery

Covers 
- database logging, recovery
- ensuring atomicity, durability in ACID.


How to avoid?
- Partially written records
    - to handle case of db crashing halfway thru appending a record to redo log.
    - use checksum to detect and ignore corrupted parts of the log.


2x Transactional Safety Mechanisms
1. Rollback Journal Mode
2. Write Ahead Logging (WAL) Journaling Mode


Rollback Journal 
- copies old version of changed pages to another file.
- directly overwrites db file with new content.
- when transaction rolls back -> copies them back into main db file.
- cons
  - limit concurrency due to locking mechanism and transaction handling way.
  No other transactions can access the db when
    - write transaction holds exclusive lock on entire db. 
    - when transaction is committed, rollback journal needs to be synced to disk before
    releasing lock. 


WAL
- for a transaction, writes new version of changed page to another file.
- leaves original page in main db file.
- pros
  - support higher concurrency than `Rollback Journal`
    - allowing reads transaction to happen concurrently with write.
    - not locking the entire db for each write.


## References
- [How SQLite helps you do ACID](https://fly.io/blog/sqlite-internals-rollback-journal/)
- [How SQLite Scales Read Concurrency](https://fly.io/blog/sqlite-internals-wal/)
