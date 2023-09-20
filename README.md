# rust-sqlite
Rewriting SQLite in Rust for Learning and Fun

```
# run tests
cargo test
# showing warnings and stdout
cargo test -- --nocapture

# execute program against a sqlite database
cargo run -- sql sample.db "select name from apples"
# suppress warnings
RUSTFLAGS=-Awarnings cargo run -- sql sample.db "select name from apples"
```

## TODO

### Read Path

Projection
- [ ] select * from table1
- [ ] select col1 from table1
- [ ] select col1, col2 from table1

Selection
- [ ] Where `select col1 from table1 where col2='value'`
- [ ] Where `IN`

Aggregation
- [ ] Count: `select name, count(1) from apples group by name;`
- [ ] Max: `select name, max(color) from apples group by name;`
- [ ] Average

Component
- [x] basic SQL to Logical Plan
- [ ] basic Logical Plan execution
- [ ] execute Logical Plan output by `datafusion-sql`


### Write Path
to be implemented later


## Architecture

SQL string -> Logical Plan: datafusion-sql

## References
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
