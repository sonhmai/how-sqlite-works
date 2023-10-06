# TODO

## Read Path

Scan
- [x] Implement DbHeader
- [ ] How to parse tables and schemas from db file first page.
- [ ] SqliteContextProvider: parse db header from file for tables and schemas
  - [ ] Implement DbMeta to be able to parse DbHeader and schema objects
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


## Write Path
not analyzed yet
