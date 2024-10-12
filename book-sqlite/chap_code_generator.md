# Physical Execution Module

The physical execution module is equivalent to the Virtual Machine (VM) in SQLite.

It executes the logical plan parsed from the SQL query and is the actor that understands
and manipulates database objects (table, indexes, tuples, records, etc.).

It talks to the Access Module to do the above things. Requirements for the Access Module
- provide API to cluster and organize tuples of a relation and separate them from those of other relations.
- provide API to store, retrieve, manipulate tuples efficiently.


## 1. What SQLite does

SQLite implements SQL functions using `callbacks` to C language routines.
- built-in SQL functions (count, coalesce, etc.) are in `func.c`
- date and time conversion are in `date.c`

Join Logic
- SQLite does loop-joins only, not merge-joins. 
    - Outer loop: leftmost table in FROM clause
    - Inner loop: rightmost table
- For example `select * from t1, t2 where some-condition` -> pseudo code
    - open read transaction on db
    - check db schema version to make sure schema has not changed after bytecode has been generated
    - open 2 read cursors: T1 and T2
    - for each record in T1, do:
        - for each record in T2, do:
            - if where condition eval to true:
                - compute all cols for current row of the result
                - invoke default callback fn for current row of result
    - close both cursors