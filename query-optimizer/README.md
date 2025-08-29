

### Does sqlite uses logical plan and physical plan separately?
- No. SQLite does not keep a clean separation between logical and physical plan.
- It does a hybrid plan, both logical and physical planning, in `query planner` layer.
  - logical rewriting (flattening subqueries, pushing down predicates, etc.) directly on parsed tree.
  - physical access method choices (index vs table scan, join order, join algo, etc.).

``` 
SQLString --Parser--> AST --QueryPlanner--> VDBE bytecode

QueryPlanner
- semantic analysis
- logical rewrites
- physical details
- emitting bytecode
```

### Which module doing what?
- Parsing / SELECT struct: parse.y, select.c, expr.c 
- Logical rewrites (flattening, push-down): select.c, whereexpr.c 
- Planning (join order, index choice): where.c (WhereLoop, WherePath, cost estimates)
- Loop codegen: wherecode.c 
- VM & opcodes: vdbe.c, vdbeaux.c, vdbeInt.h 
- Stats: analyze.c, sqlite_stat1/stat4

### How SQLite does join algo selection?

Pseudocode
```
function plan_query(tables, predicates, orderby, limit):
    whereTerms = extract_where_terms(predicates) // eq join terms, ranges, etc.
    loopsByTable = {} // candidate WhereLoops per table
    
    // build candidate loops
    for T in tables:
        candidates = []
        if exists usable index I on T matching whereTerms[T]:
            candidates.append(loop_using_index_range(T, I, whereTerms[T]))
            if index_is_covering_for_needed_cols(I):
                mark_as_covering(candidates.last) // no need to read this table as the index covers it
                
        if equality_join_to_PK_exists(T, whereTerms):
            candidates.append(loop_using_rowid_lookup(T, whereTerms))
            
        if automatic_index_helpful(T, whereTerms):
            candidates.append(loop_using_automatic_index(T, whereTerms))  // ephemeral btree
            
        candidates.append(loop_full_scan(T))  // fallback
        
        // estimate cost/cardinality for each candidate
        for L in candidates:
            L.cost, L.rows = estimate_cost_and_rows(L, stats)
        loopsByTable[T] = candidates
        
        // Dynamic-programming join-order search (where.c: WherePath/WhereBestIndex)
        bestPath = DP_choose_join_order(loopsByTable, whereTerms, orderby)
        
        // Try to satisfy ORDER BY / LIMIT with an index on the chosen outer
        if orderby_satisfied_by(bestPath):
            adjust_cost_for_no_sort(bestPath)
            enable_early_limit(bestPath, limit)

        // Generate VDBE (wherecode.c/select.c): nested loops in chosen order
        return codegen_nested_loops(bestPath)
```

`loop_using_index_range(T, I, whereTerms[T])`
- Scan index I over a possible narrow key range instead of a full table scan.
- Examples
  - Index `CREATE INDEX idx1 ON orders(customer_id, order_date);`
    - WHERE `customer_id=? AND order_date>=?` -> usable (prefix eq on customer_id + range on order_date)
    - WHERE `order_date >= ?` alone -> not for search (no leftmost customer_id eq)
    - WHERE `customer_id IN (?,?,?)` -> usable, planner estimates fan-out for IN list
  - Partial Index `CREATE INDEX idx_vietnam ON customer(country) WHERE country='Vietnam';`
    - query WHERE must has `country='Vietnam'` for `idx_hanoi` to be usable
    - if country is a parameter or broader condition (>, <, range, etc.), planner cannot prove that index
    can be used and will skip.

### An example of join selection

Setup
```sql 
CREATE TABLE customer(
  id INTEGER PRIMARY KEY,
  name TEXT,
  city TEXT
);
CREATE TABLE account(
  id INTEGER PRIMARY KEY,
  cust_id INTEGER,
  balance REAL
);

CREATE INDEX idx_account_balance ON account(balance DESC);   -- to satisfy WHERE and ORDER BY
CREATE INDEX idx_account_custid  ON account(cust_id);        -- for the join
CREATE INDEX idx_customer_city   ON customer(country);

SELECT c.name, a.balance
FROM account AS a
JOIN customer AS c ON a.cust_id = c.id
WHERE a.balance > 1000 AND c.country = 'Vietnam'
ORDER BY a.balance DESC
LIMIT 10;
```

### Where does SQLite store table stats to be used in optimizer/ planner?

### How SQLite does predicate push-down?
TODO

### Refs
- [SQLite: Past, Present, and Future](https://www.vldb.org/pvldb/vol15/p3535-gaffney.pdf)