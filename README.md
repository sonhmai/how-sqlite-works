# rust-sqlite
Rewriting SQLite in Rust for Learning and for Fun.

This was inspired by
- [CodeCrafters SQLite programming challenge](https://app.codecrafters.io/courses/sqlite), please pay them a visit.
- [SQLite detailed documentation](https://www.sqlite.org/fileformat.html).
- [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion).


Getting Started
```
# run tests
cargo test
# showing warnings and stdout
cargo test -- --nocapture

# execute program against a sqlite database
cargo run -- sql sample.db "select name from apples"
# suppress warnings
RUSTFLAGS=-Awarnings cargo run -- sql sample.db "select name from apples"

# see what returns by sqlite
sqlite3 sample.db "select * from apples"
```


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
Buffer Pool (custom)
Concurrency Control (custom)
Recovery (custom)
-----Storage Layer-----
Disk Manager
File Storage (custom following SQLite database file format)
```


- Disk Manager
  - logical abstraction over physical file system and disk access
  - provides interfaces of physical disk operations: reads, writes, flushes, etc.


### Known Limitations

Schema
- sqlparser-rs and datafusion seems not having knowledge re primary key and auto-increment.
Parsing DDL which has `id integer primary key autoincrement` lost knowledge of `primary key autoincrement`.


Data types
- Arrow supports Utf8 only. Sqlite has Text in (UTF-8, UTF-16BE or UTF-16LE) so
only utf8 is supported.


## References

### Readings
1. Paper - Architecture of a Database System (2007). Overview of important components to relational database systems.
2. Book - SQLite Database System Design and Implementation, Sibsankar Haldar (2016).
3. Article - [Series: What would SQLite look like if written in Rust?](https://dev.to/thepolyglotprogrammer/what-would-sqlite-look-like-if-written-in-rust-part-2-4g66)


### Projects
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
- [rqlite, A lightweight, distributed relational database built on SQLite in Golang](https://github.com/rqlite/rqlite)
