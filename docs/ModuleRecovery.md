# Recovery

Covers database logging and recovery.

How to avoid?
- Partially written records
    - to handle case of db crashing halfway thru appending a record to redo log.
    - use checksum to detect and ignore corrupted parts of the log.


