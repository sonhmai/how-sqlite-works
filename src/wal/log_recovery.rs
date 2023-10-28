use datafusion_expr::to_hex;
use crate::access::buffer_pool::BufferPool;
use crate::storage::disk_manager::DiskManager;

/// LogRecovery reads log file from disk, redo and undo.
pub struct LogRecovery {
    disk_manager: DiskManager,
    buffer_pool: BufferPool,
}

impl LogRecovery {

    /// redo on page level
    pub fn redo(&self) {
        // Read log file from the beginning to end
        //   - prefetch log records into log buffer to reduce IO
        //   - compare page LSN with log_record's sequence number
        //   - build active_txn table and lsn_mapping table
        todo!()
    }

    /// undo on page level
    pub fn undo(&self) {
        // iterate thru transaction map and undo each operation
        todo!()
    }
}