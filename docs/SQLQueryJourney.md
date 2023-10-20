# SQL Query Journey

Let's walkthrough a journey from beginning to end of a sample SQL query
going through the code modules.

Stepping through code can be achieved with test `tests/test_scan_commands/cli_sql_scan_table_single_page`

- User using CLI `bin/rsql.rs` to execute a SQL Query `SELECT * FROM apples;` 
on database file `tests\resources\sample.db`.
- main function creates a `Database`, it should
   - open database file and reads entire sqlite_schema table to have db metadata info (db objects and schema, etc.)
- `SqliteContextProvider` needed for `arrow-datafusion` logical planning is created and fed to `SqlToRel`
which is LogicalPlanner of datafusion.
- sql string is fed to `arrow-datafusion` used as LogicalPlanner to produce a LogicalPlan `TableScan(apples)`.
- LogicalPlan is fed to PhysicalPlanner to produce PhysicalPlan `ExecScan` for execution.
- main calls `execute` on physical plan `ExecScan`
- PhysicalPlan call modules like BTree for table scan.
- BTree Table Scan returns `Vec<DataRecord>` to ExecScan.
- `Vec<DataRecord>` is shown to stdout in sqlite cli format using a presentation util function.
