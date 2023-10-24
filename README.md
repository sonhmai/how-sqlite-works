# rust-sqlite

Rewriting SQLite in Rust for Learning and for Fun.

This was inspired by
- [CodeCrafters SQLite programming challenge](https://app.codecrafters.io/courses/sqlite), please pay them a visit.
- [SQLite detailed documentation](https://www.sqlite.org/fileformat.html).
- [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion).
- [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/)


## 1. Getting Started


``` 
docs # detailed doc for implementation and design records, step by step guidelines, module walkthrough, etc.
readings # related more comprehensive readings like books and articles
src # source code with unit tests, for a more detailed module description, look at Architecture section
  bin # binary cli entry point with main function
  access # access layer
  concurrency # handling concurrency control: transactions for example
  logical # things with logical layer like logical plan, not much here as we use arrow-datafusion for this
  model # main domain model of sqlite database like TableLeafCell that mapped to sqlite3 doc concepts
  physical # things related to physical planning and execution
  recovery # handles recovery for failure and 
  storage # module handling physical storage to file on disk
  util 
    presentation.rs # how sqlite present returned result to cli stdout (rows)
    varint.rs # varint encode and decode
tests # integration tests and test resources
```


```
# run tests
cargo test
# showing warnings and stdout
cargo test -- --nocapture

# execute program against a sqlite database
# this table has 4 rows
cargo run -- sql tests/resources/sample.db "select name from apples"
# this table has 6895 rows and span > 1 db page
cargo run -- sql tests/resources/superheroes.db "select * from superheroes"

# suppress warnings
RUSTFLAGS=-Awarnings cargo run -- sql sample.db "select name from apples"

# see what returns by sqlite
sqlite3 sample.db "select * from apples"

# output >= debug logs
export RUST_LOG=debug
sqlite3 sample.db "select * from apples"
RUST_LOG=debug sqlite3 sample.db "select * from apples"  # this also works
```

Sample.db schema
- apples: id integer primary key, name text, color text
- oranges: id integer primary key, name text, description text


## 2. Architecture
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
Recovery (custom)
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
- Recovery
  - managing recovery process and ensure consistency in case of failures/ crashes.
  - uses techniques like logging (e.g. Write Ahead Logging) and periodic checkpoints.


Storage Layer
- Disk Manager
  - logical abstraction over physical file system and disk access
  - provides interfaces of physical disk operations: reads, writes, flushes, etc.


Physical File: actual file in sqlite3 file format.


Sequence Diagram: SQL String to returned result

TODO


## 3. Known Limitations

Schema
- sqlparser-rs and datafusion seems not having knowledge re primary key and auto-increment.
Parsing DDL which has `id integer primary key autoincrement` lost knowledge of `primary key autoincrement`.


Data types
- Arrow supports Utf8 only. Sqlite has Text in (UTF-8, UTF-16BE or UTF-16LE) so
only utf8 is supported.


## 4. References

### Readings
1. Paper - Architecture of a Database System (2007). Overview of important components to relational database systems.
2. Book - SQLite Database System Design and Implementation, Sibsankar Haldar (2016).
3. Article - [Series: What would SQLite look like if written in Rust?](https://dev.to/thepolyglotprogrammer/what-would-sqlite-look-like-if-written-in-rust-part-2-4g66)


### Projects
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
- [rqlite, A lightweight, distributed relational database built on SQLite in Golang](https://github.com/rqlite/rqlite)
