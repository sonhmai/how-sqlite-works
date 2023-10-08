# rust-sqlite
Rewriting SQLite in Rust for Learning and for Fun.

This was inspired by
- [CodeCrafters SQLite programming challenge](https://app.codecrafters.io/courses/sqlite), please pay them a visit.
- [SQLite detailed documentation](https://www.sqlite.org/fileformat.html).
- [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion).


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

SQL string -> Logical Plan: datafusion-sql

## References
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
