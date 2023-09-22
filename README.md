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

# see what returns by sqlite
sqlite3 sample.db "select * from apples"
```

## TODO

### Read Path

Projection
- [x] select * from table1
- [ ] Implement Project by Column Name in ExecProjection: select col1 from table1
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
- [x] basic Logical Plan execution
- [x] physical plan
- [ ] ColumnValue and DataRecord
- [ ] Parsing database
- [ ] Parsing table
- [ ] replace hardcoded ExecApplesScan by actual sqlite table scan


### Write Path
to be implemented later


## Architecture

SQL string -> Logical Plan: datafusion-sql

## References
- https://github.com/datafuselabs
- [Apache OpenDAL](https://github.com/apache/incubator-opendal)
- [GrepTimeDB](https://github.com/GreptimeTeam/greptimedb)
