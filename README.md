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
File Storage (custom following SQLite database file format)
```


## References

### Readings
1. Paper - Architecture of a Database System (2007). Overview of important components to relational database systems.
2. Book - SQLite Database System Design and Implementation, Sibsankar Haldar (2016).


### Projects
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
- [rqlite, A lightweight, distributed relational database built on SQLite in Golang](https://github.com/rqlite/rqlite)
