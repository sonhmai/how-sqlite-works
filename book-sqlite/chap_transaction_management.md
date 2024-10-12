# Chapter Transaction Management

Contents
1. how SQLite achieves ACID (Atomicity, Consistency, Isolation, Durability)
2. how SQLite uses locking
3. implementing transaction manager (concurrency manager) `Pager` in sqlite
4. implementing recovery manager


Write path
1. write log record containing recovery info (e.g. old and new values of items) to transaction log/ journal.
2. DBMS persists log record to disk before changing item in db.
3. when transaction aborted or there is a crash, db uses persisted log to move db to a consistent state.
   - either rollback/ undo operations of uncommitted transactions
   - or roll-forward/ redo operations of committed transactions that has not been reflected in db files


### Write-Ahead Logging

There are and cons of using WAL instead of rollback journal. 
Some of from SQLite website is depicted below. For more see https://sqlite.org/wal.html

Pros
1. WAL is significantly faster in most scenarios.
2. WAL provides more concurrency as readers do not block writers and a writer does not block readers. Reading and writing can proceed concurrently.
3. Disk I/O operations tends to be more sequential using WAL.
4. ...

Cons
1. All processes using a database must be on the same host computer; WAL does not work over a network filesystem.
2. not possible to change the page_size after entering WAL mode.
3. ...

### Implement a transaction

Pager (transaction manager)
- Pager is transaction manager in sqlite, ensures ACID
- manages locks on db files + log records in journal files.
- decides on mode of locks and time of acquiring and releasing locks.
- follows strict two phase locking protocol to produce serializable transactions' execution.
- determines content of log records, writes them to journal file.

#### Read path

#### Write path