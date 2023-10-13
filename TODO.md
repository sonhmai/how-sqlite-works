# TODO

## Query Planning and Processing (Read Path)

Scan
- [x] Implement DbHeader
- [ ] SqliteContextProvider: parse db header from file for tables and schemas
  - [x] Create DiskManager for centralizing physical ops instead of using BufferPool.
  - [x] Centralize init process in Database.
  - [ ] Implement DbMeta to be able to parse DbHeader and schema objects
    - [x] Page cell_ptrs()
    - [x] DbMeta parses leaf_table_cells for first page
    - [x] Implement SchemaObject::parse(&LeafTableCell)
    - [ ] Add parsing Columns from sql statement to SchemaObject::parse
  - [ ] From schema objects we can get table (name, cols, data types) for SqliteContextProvider
  - [ ] Implement this conversion in SqliteContextProvider::new, 
  potentially we need to convert sqlite type to arrow_schema types.
- [ ] ExecScan: implement Physical TableScan

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


## Query Optimizer
TODO, not prioritized yet.


## Buffer Pool
- Buffer Pool is a in-memory cache of pages from the database file on disk.
- All access methods (read and write) MUST go through the buffer pool 
and not the database file directly.


- [ ] `SqliteContextProvider` and `DbMeta`
  when SQLite starts, use buffer pool for parsing db header and metadata from first page.
- [ ] maintain dirty-flag for each page, set if page is modified.
- [ ] `Buffer Replacement Policy` implement LRU policy for page eviction when buffer is full.
  - maintain timestamp of when page was last accessed.
  - when buffer is full, evict page with oldest timestamp.
  - store timestamp in a data structure that allows efficient sorting and retrieving smallest.


## Write Path
TODO


## Concurrency Control
TODO


## Recovery 
- [ ] implement LogRecord
- [ ] LogManager
- [ ] CheckpointManager
- [ ] LogRecovery: reads log file from disk, redo and undo.
