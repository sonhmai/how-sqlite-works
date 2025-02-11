/*

Checkpoint algo https://www.sqlite.org/fileformat2.html#checkpoint_algorithm
- WAL flushed to disk using VFS xSync
- valid content of WAL is transferred to db file
- db file flushed to disk using another VFS xSync
- xSync operations serve as write barriers - all writes launched before the xSync
must complete before any write that launches after the xSync begins.

 */
