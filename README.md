# rust-sqlite

Rewriting SQLite in Rust for learning and for fun, then writing a book I wished I had on the journey.

## 1. Getting Started

Run the book on localhost
```
cargo install mdbook # install mdbook to serve the book if not available
mdbook serve
# go to localhost:3000 on browser to read the book
```

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
  wal # Write Ahead Logging for atomicity, recovery, etc.
  storage # module handling physical storage to file on disk
  util 
    presentation.rs # how sqlite present returned result to cli stdout (rows)
    varint.rs # varint encode and decode
tests # integration tests and test resources
```


```
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

Testing
```
# run tests
cargo test
# showing warnings and stdout
cargo test -- --nocapture
# run all tests with prefix test_move_to_right and show print output
cargo test move_to_right -- --nocapture
```


Sample.db schema
- apples: id integer primary key, name text, color text
- oranges: id integer primary key, name text, description text


## 3. Known Limitations

Schema
- sqlparser-rs and datafusion seems not having knowledge re primary key and auto-increment.
Parsing DDL which has `id integer primary key autoincrement` lost knowledge of `primary key autoincrement`.


Data types
- Arrow supports Utf8 only. Sqlite has Text in (UTF-8, UTF-16BE or UTF-16LE) so
only utf8 is supported.


## 4. References

This was inspired by these references
- [CodeCrafters SQLite programming challenge](https://app.codecrafters.io/courses/sqlite), please pay them a visit.
- [SQLite detailed documentation](https://www.sqlite.org/fileformat.html).

### Readings
1. Paper - Architecture of a Database System (2007). Overview of important components to relational database systems.
2. Book - SQLite Database System Design and Implementation, Sibsankar Haldar (2016).
3. Article - [Series: What would SQLite look like if written in Rust?](https://dev.to/thepolyglotprogrammer/what-would-sqlite-look-like-if-written-in-rust-part-2-4g66)


### Projects
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
- [rqlite, A lightweight, distributed relational database built on SQLite in Golang](https://github.com/rqlite/rqlite)
- [Limbo, an in-process OLTP with async io database management system, compatible with SQLite.](https://github.com/penberg/limbo)
- [Apache Arrow DataFusion](https://github.com/apache/arrow-datafusion).
- [Let's Build a Simple Database](https://cstack.github.io/db_tutorial/)
