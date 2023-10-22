# BTree Module
This page explains the implementation of BTree module and compare the differences
of it and related modules to the SQLite implementation.


Important
- Rust-sqlite does not use the VM Layer that SQLite has.


Sources
- Subsankar Haldar, SQLite Database System Design and Implementation


## This implementation
- should BtCursor return, traverse at page or page's cell level?

idea? (this implementation -> SQLite)
- Database -> BTree
- Page -> MemPage
- do we need BtShared?
- BtCursor -> BtCursor
- ExecScan -> VM full scan func


## What SQLite does

Control data structures

To operate on a specific tree in db
- VM open tree by creating cursor on tree `sqlite3BtreeCursor`


``` 
// VM opens a db file -> BTree object. VM uses obj as handle to modify the file.
// it summarizes everything VM needs to know about the file. 
// BTree obj == db connection for for VM.
struct BTree
    db // ptr to lib connection holding this Btree object
    pBt // ptr to BtShared obj via which the tree module accesses pages of db file (which has Pager)
    inTrans // indicates whether a transaction is in progress
    // other control vars...
    
    
// represents state of a single db file
struct BtShared
    pPager // ptr to a Pager that manages this db and journal file
    pCursor // a list of open cursors on trees of db
    pageSize // total number of bytes on each page
    nTransaction // number of open (read and write) transactions
    inTransaction // transactional state
    pSchema // ptr to schema cache of schema objects
    db // ptr to lib connection that currently using this object
    pPage1 // pointer to in-mem copy of MemPage object for db Page 1
    mutex // access synchronizer
    // ,,, other control vars
    

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
// For each open tree, tree module creates an object of BtCursor type 
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