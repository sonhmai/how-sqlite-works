#[derive(Debug, Clone)]
enum LogRecordType {
    BEGIN,
    COMMIT,
    ABORT,
}

#[derive(Debug)]
struct LogRecord {
    lsn: i64,
    prev_lsn: i64,
    log_record_type: LogRecordType,
    size: i32,
}

impl LogRecord {}
