# Chapter 1


## Architecture
Differences to SQLite official implementation:
- Database Frontend (Tokenizer, Parser) is replaced by DataFusion.
- Virtual Machine is replaced by using DataFusion and a custom Physical Layer for query processing and execution.


Layers
```
SQL String
-----Logical Layer-----
Query Planning: Tokenizer, Parser (datafusion)
Logical Plan (datafusion)
-----Physical Layer-----
Physical Planner (custom)
Physical Plan (custom)
-----Access Layer-----
BTree Module (custom)
Buffer Pool (custom)
Concurrency Control (custom)
WAL Write Ahead Logging (custom)
-----Storage Layer-----
Disk Manager
-----Physical File------
File (e.g. on disk) following SQLite database file format
```


Logical Layer
- This layer is responsible for interpreting the SQL string and converting it into a logical plan
  that represents the operations to be performed on the data.
- It involves some important steps
    - Tokenizer: SQL string is tokenined into tokens.
    - Parser: Tokens are used to build Abstract Syntax Tree (AST).
    - Query Planning: AST is transformed into Logical Plan.
- [DataFusion](https://github.com/apache/arrow-datafusion) library is used for this layer.


Physical Layer
- Physical Planner: takes the `LogicalPlan` of `arrow-datafusion` and transform it to an executable physical plan called `Exec`.
- why physical planning of `arrow-datafusion` is not used?
    - custom-built in order to custom this layer to have SQLite functionalities.
      For example, physical plan of a table scan will scan the table in the database file in SQLite format.


Access Layer
- The access layer is responsible for managing how data is accessed and manipulated.
  This includes managing data structures like B-Trees, handling concurrency to ensure data integrity,
  and handling recovery and consistency in case of system failures.
- BTree Module
    - managing the B-Tree data structure used for storing and retrieving data.
- Buffer Pool
    - The Buffer Pool is a cache of data that resides in memory for faster access, for locking, transaction managemeng, etc.
    - When data is read from the disk, it is first loaded into the buffer pool.
    - It also handles the replacement policy when the buffer is full, typically using an LRU (Least Recently Used) policy.
- Concurrency Control
    - ensures that multiple concurrent operations (e.g. write) can happen and do not
      impact data integrity (data corruption, missing amount, etc.).
- Write Ahead Logging
    - providing Atomicity, Recovery, etc. for the db.
    - managing recovery process and ensure consistency in case of failures/ crashes.
    - uses techniques like logging (e.g. Write Ahead Logging) and periodic checkpoints.


Storage Layer
- Disk Manager
    - logical abstraction over physical file system and disk access
    - provides interfaces of physical disk operations: reads, writes, flushes, etc.


Physical File: actual file in sqlite3 file format.


Sequence Diagram: SQL String to returned result

TODO

