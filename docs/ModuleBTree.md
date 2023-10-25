# BTree Module
This page explains the implementation of BTree module and compare the differences
of it and related modules to the SQLite implementation.


Important
- Rust-sqlite does not use the VM Layer that SQLite has.


Sources
- Subsankar Haldar, SQLite Database System Design and Implementation


## This implementation
- should BtCursor return, traverse at page or page's cell level?

Mapping equivalent components this implementation -> SQLite:
- Database -> sqlite3 *db // the database connection
- BufferPool -> Pager
- Page -> MemPage
- do we need BtShared?
- BtCursor -> BtCursor
- ExecScan -> VM full scan func


## What SQLite does

Control data structures

To operate on a specific tree in db
- VM open tree by creating cursor on tree `sqlite3BtreeCursor`


### Btree struct: a Btree handle
- How does Btree, BtCursor, sqlite3BtreeCursor connection to Table?
  - There is not a direct link.
  - Btree and BtCursor operate at lower level, dealing with B-Tree structures.
  - Table structure is used at a higher level to represent schema of a table.
  - When a table is accessed as part of a SQL statement, SQLite will use the information 
  in the Table structure to determine how to access the table. This may involve calling 
  `sqlite3BtreeCursor` to open a cursor on the table's B-Tree structure, 
  but this is done indirectly, not directly from the `Table` structure.


``` 
/* A database connection contains a pointer to an instance of
** this object for every database file that it has open.  This structure
** is opaque to the database connection.  The database connection cannot
** see the internals of this structure and only deals with pointers to
** this structure.
**
** VM opens a db file -> BTree object. VM uses obj as handle to modify the file.
** it summarizes everything VM needs to know about the file. 
** BTree obj == db connection for for VM.
*/
struct Btree {
  sqlite3 *db;       /* The database connection holding this btree */
  BtShared *pBt;     /* Sharable content of this btree */
  u8 inTrans;        /* TRANS_NONE, TRANS_READ or TRANS_WRITE */
  u8 sharable;       /* True if we can share pBt with another db */
  u8 locked;         /* True if db currently has pBt locked */
  u8 hasIncrblobCur; /* True if there are one or more Incrblob cursors */
  int wantToLock;    /* Number of nested calls to sqlite3BtreeEnter() */
  int nBackup;       /* Number of backup operations reading this btree */
  u32 iBDataVersion; /* Combines with pBt->pPager->iDataVersion */
  Btree *pNext;      /* List of other sharable Btrees from the same db */
  Btree *pPrev;      /* Back pointer of the same list */
#ifdef SQLITE_DEBUG
  u64 nSeek;         /* Calls to sqlite3BtreeMovetoUnpacked() */
#endif
#ifndef SQLITE_OMIT_SHARED_CACHE
  BtLock lock;       /* Object used to lock page 1 */
#endif
};
```


### BtShared struct
```   
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
```


### BtCursor
```
// Cursor acts as a logical pointer to particular entry in a tree.
// For each open tree, tree module creates an object of BtCursor type 
// that is used as a handle to read, insert, or delete tuples from the tree.
struct BtCursor {
  u8 eState;                /* One of the CURSOR_XXX constants (see below) */
  u8 curFlags;              /* zero or more BTCF_* flags defined below */
  u8 curPagerFlags;         /* Flags to send to sqlite3PagerGet() */
  u8 hints;                 /* As configured by CursorSetHints() */
  int skipNext;    /* Prev() is noop if negative. Next() is noop if positive.
                   ** Error code if eState==CURSOR_FAULT */
  Btree *pBtree;            /* The Btree to which this cursor belongs */
  Pgno *aOverflow;          /* Cache of overflow page locations */
  void *pKey;               /* Saved key that was cursor last known position */
  /* All fields above are zeroed when the cursor is allocated.  See
  ** sqlite3BtreeCursorZero().  Fields that follow must be manually
  ** initialized. */
#define BTCURSOR_FIRST_UNINIT pBt   /* Name of first uninitialized field */
  BtShared *pBt;            /* The BtShared this cursor points to */
  BtCursor *pNext;          /* Forms a linked list of all cursors */
  CellInfo info;            /* A parse of the cell we are pointing at */
  i64 nKey;                 /* Size of pKey, or last integer key */
  Pgno pgnoRoot;            /* The root page of this tree */
  i8 iPage;                 /* Index of current page in apPage */
  u8 curIntKey;             /* Value of apPage[0]->intKey */
  u16 ix;                   /* Current index for apPage[iPage] */
  u16 aiIdx[BTCURSOR_MAX_DEPTH-1];     /* Current index in apPage[i] */
  struct KeyInfo *pKeyInfo;            /* Arg passed to comparison function */
  MemPage *pPage;                        /* Current page */
  MemPage *apPage[BTCURSOR_MAX_DEPTH-1]; /* Stack of parents of current page */
};
```


### How is a table scan executed?

Step 1: BtCursor is open on a Btree with `int sqlite3BtreeCursor(...)`.
This cursor is used to traverse rows in a table or entries in an index.
```
int sqlite3BtreeCursor(
  Btree *p,                                   /* The btree containing the table to open */
  Pgno iTable,                                /* Root page of table to open */
  int wrFlag,                                 /* 1 to write. 0 read-only */
  struct KeyInfo *pKeyInfo,                   /* First arg to xCompare() */
  BtCursor *pCur                              /* Allocated space to write new cursor here */
){
  if( p->sharable ){
    return btreeCursorWithLock(p, iTable, wrFlag, pKeyInfo, pCur);
  }else{
    return btreeCursor(p, iTable, wrFlag, pKeyInfo, pCur);
  }
}
```

Step 2: Moving to the first row
- The cursor is moved to the first row in the table using the `sqlite3BtreeFirst()` function.

``` 
/* Move the cursor to the first entry in the table.  Return SQLITE_OK
** on success.  Set *pRes to 0 if the cursor actually points to something
** or set *pRes to 1 if the table is empty.
*/
int sqlite3BtreeFirst(BtCursor *pCur, int *pRes){
  int rc; // return code

  assert( cursorOwnsBtShared(pCur) );
  assert( sqlite3_mutex_held(pCur->pBtree->db->mutex) );
  rc = moveToRoot(pCur);
  if( rc==SQLITE_OK ){
    assert( pCur->pPage->nCell>0 );
    *pRes = 0;
    rc = moveToLeftmost(pCur);
  }else if( rc==SQLITE_EMPTY ){
    assert( pCur->pgnoRoot==0 || (pCur->pPage!=0 && pCur->pPage->nCell==0) );
    *pRes = 1;
    rc = SQLITE_OK;
  }
  return rc;
}

/*
** Move the cursor down to the left-most leaf entry beneath the
** entry to which it is currently pointing.
**
** The left-most leaf is the one with the smallest key - the first
** in ascending order.
*/
static int moveToLeftmost(BtCursor *pCur){
  // why static? static = internal linkage = can only be called within same file, not from other files.
  Pgno pgno;
  int rc = SQLITE_OK;
  MemPage *pPage;

  assert( cursorOwnsBtShared(pCur) );
  assert( pCur->eState==CURSOR_VALID );
  
  // Traversing down the B-Tree, moving the cursor to the leftmost child of each page, until it reaches a leaf page.
  // continue as long as the previous operation was successful (rc == SQLITE_OK) 
  // and the current page is not a leaf page. 
  while( rc==SQLITE_OK && !(pPage = pCur->pPage)->leaf ){
    // check if the current cell index (pCur->ix) is less than the total number of cells 
    // in the page (pPage->nCell) to ensure that the cursor is within the bounds of the page.
    assert( pCur->ix < pPage->nCell );
    // get page number of child page that cursor should move to next
    pgno = get4byte(findCell(pPage, pCur->ix));
    // move cursor to the child page
    rc = moveToChild(pCur, pgno);
  }
  return rc;
}
```

Step 3: Iterating over the rows
- The rows in the table are iterated over using a loop. 
- Inside the loop, the sqlite3BtreeNext() function is used to move the cursor to the next row.


Step 4: Processing each row

Step 5: Closing the cursor