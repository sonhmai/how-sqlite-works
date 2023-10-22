# BTree Module
This page explains the implementation of BTree module and compare the differences
of it and related modules to the SQLite implementation.


Important
- Rust-sqlite does not use the VM Layer that SQLite has.


Sources
- Subsankar Haldar, SQLite Database System Design and Implementation


## This implementation
- should BtCursor return, traverse at page or page's cell level?


## What SQLite does

Control data structures

To operate on a specific tree in db
- VM open tree by creating cursor on tree `sqlite3BtreeCursor`


``` 
struct BTree
    


struct MemPage
    pBt // pointer to BtShared obj to which this page belongs to.
    pDbPage // pointer to page header PgHdr that holds this page.
    pgno // page number for this page.
    aData // pointer back to the start of the in-cache page image.
    intKey // for B+tree
    leaf // true if leaf flag is set
    hasData // true if this page stores data
    nCell
    nFree
    // ... other vars
    
// Cursor acts as a logical pointer to particular entry in a tree.
// For each open tr ee, th e tr ee module creates an object of BtCursor type 
// that is used as a handle to read, insert, or delete tuples from the tree.
struct BtCursor
    pBt // ptr to BtShared owning this cursor.
    pBtree // ptr to Btree obj to which cursor belongs to.
    pgnoRoot // rootpage of tree the cursor represents.
    apPage[] //  pages from tree root down to current page.
    aiIdx[] // current indexes in apPage array.
    iPage // index of current page in apPage[]
    wrFlag // true if we can write to this cursor
    eState // current state of cursor
    ...
```