# Vector extension

Terms
- recall
- ANN: Approximate Nearest Neighbors
- GSNG: Generalized Sparse Neighborhood Graph
- LM-DiskANN: a variant of ANN designed for low memory consumption by keeping only a small part of the vector index in memory

Algo: 
- targeted graph-based algorithms which aim to build a graph in a way that its traversal can find approximate nearest neighbors quickly by visiting only small subgraph of the whole dataset.
- implemented the LM-DiskANN algorithm (Pan, 2023), a low-memory footprint variant of the DiskANN vector search algorithm (Subramanya, 2019 and Singh, 2021), and integrated it into the SQLite indexing system.

### tutorial

```sql
-- create table with F32_BLOB vector column
CREATE TABLE movies (
 title TEXT,
 year INT,
 embedding F32_BLOB(3)
);
-- insert data
INSERT INTO movies VALUES ('Raiders of the Lost Ark', 1981, vector('[1,2,3]'));
INSERT INTO movies VALUES ('Indiana Jones and the Temple of Doom', 1984, vector('[1,2,4]'));
INSERT INTO movies VALUES ('Indiana Jones and the Last Crusade', 1989, vector('[1,2,5]'));
INSERT INTO movies VALUES ('Indiana Jones and the Kingdom of the Crystal Skull', 2008, vector('[5,6,7]'));
INSERT INTO movies VALUES ('Indiana Jones and the Dial of Destiny', 2023, vector('[5,6,8]'));
-- find 3 exact nearest neighbors
SELECT title, year FROM movies ORDER BY vector_distance_cos(embedding, vector('[5,6,7]')) LIMIT 3;
-- creates a DiskANN index movies_idx on the embedding vector column of the movies table, 
-- using the cosine distance function (1 - cosine similarity) to determine nearest neighbors.
create index movies_idx on movies(libsql_vector_idx(embedding, 'metric=cosine'));
```

insert opcode
```
libsql> EXPLAIN INSERT INTO movies VALUES ('Raiders of the Lost Ark', 1981, vector('[1,2,3]'));

addr  opcode         p1    p2    p3    p4                       p5  authors comment
----  -------------  ----  ----  ----  -----------------------  --  -------------
...
2     OpenVectorIdx  1     5     0     k(2,,)                   0   (!) new opcode - opens vector index movies_idx
...
13    PureFunc       2     11    6     libsql_vector_idx(-1)    32  prepare data (libsql_vector_idx under the hood just returns its first argument)
14    IntCopy        1     7     0                              0   prepare rowid
15    MakeRecord     6     2     5                              0   prepare record for vector index
...
17    IdxInsert      1     5     6     2                        16  insert record (vector, rowid)
...
```

sqlite code gen
- emit a special opcode `OpenVectorIdx` when a vector index in a column is detected.
- instruction opens a vector index, which adds a special-case to the OP_IdxInsert instruction, which is used to update an index.
- when you insert a row in a table with a vector column, SQLite core calls into the DiskANN code to update the vector index.

### api
the (internal, non-API) interface between this module and the
rest of the SQLite system:

```
diskAnnCreateIndex()     Create new index and fill default values for diskann parameters (if some of them are omitted)
diskAnnDropIndex()       Delete existing index
diskAnnClearIndex()      Truncate existing index
diskAnnOpenIndex()       Open index for operations (allocate all necessary internal structures)
diskAnnCloseIndex()      Close index and free associated resources
diskAnnSearch()          Search K nearest neighbours to the query vector in an opened index
diskAnnInsert()          Insert single new(!) vector in an opened index
diskAnnDelete()          Delete row by key from an opened index
```

### blobspot

## Refs
- https://turso.tech/blog/approximate-nearest-neighbor-search-with-diskann-in-libsql
- https://github.com/asg017/sqlite-vec/issues/25
- https://ai.plainenglish.io/the-space-complexity-of-vector-search-indexes-in-libsql-3fadb0cdee96

## Quotes

DiskANN is suitable for fragmented BTree format of sqlite
- Brute force search works great in LanceDB because it uses a contiguous columnar format. 
- In SQLite there is an upper bound to performance because the data is inherently fragmented. 
- By using DiskANN, you can actually leverage this fragmentation and use the B-tree to do the lookups for nearest neighbors. 
- It seems uniquely suited to SQLite's storage engine.
