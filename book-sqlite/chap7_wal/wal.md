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
2. Roll-forward by Write Ahead Logging (WAL) Journal Mode


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


WAL (Roll-forward Journal)
- for a transaction, writes new version of changed page to another file.
- leaves original page in main db file.
- pros
  - support higher concurrency than `Rollback Journal`
    - allowing reads transaction to happen concurrently with write.
    - not locking the entire db for each write.


RollForward vs RollBack
- The term "roll forward" in the context of databases refers to the process of applying changes recorded 
in a log to the actual database to bring it to a more recent state.
- When a transaction is executed, all changes (inserts, updates, deletes) are first recorded in a log. 
If a system failure occurs, these logs can be used to "roll forward" all the changes that were 
not yet applied to the database at the time of the failure.
- The process is called "roll forward" because it moves the state of the database "forward in time" 
to a more recent state by applying the changes from the log. 
This is in contrast to a "rollback" operation, which reverts the database to a previous state by undoing changes.
- In essence, the name "roll forward" is a metaphor that describes the action of 
advancing the state of the database by applying changes from a log.


## References
- [How SQLite helps you do ACID](https://fly.io/blog/sqlite-internals-rollback-journal/)
- [How SQLite Scales Read Concurrency](https://fly.io/blog/sqlite-internals-wal/)
- [Forensic examination of SQLite Write Ahead Log (WAL) files](https://sqliteforensictoolkit.com/forensic-examination-of-sqlite-write-ahead-log-wal-files/)
