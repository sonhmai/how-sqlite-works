use std::fs::File;
use std::io::Read;

use anyhow::Result;

use crate::access::buffer_pool::BufferPool;
use crate::model::db_header::DbHeader;
use crate::model::db_meta::DbMeta;
use crate::storage::disk_manager::DiskManager;

const MAGIC_HEADER: [u8; 16] = *b"SQLite format 3\0";
const ROOT_PAGE_OFFSET: u8 = 100;
const NUM_CELLS_OFFSET: u8 = 3;

/// A sqlite3 database (1 db file)
#[derive(Debug)]
pub struct Database {
    pub db_meta: DbMeta,
    pub buffer_pool: BufferPool,
}

impl Database {
    /// create a Database instance from file path
    pub fn new(file_path: &str) -> Result<Self> {
        println!("Creating Database from {file_path}");

        // To get page_size we need to parse the first 100 bytes before
        // constructing BufferPool and DiskManager as they need those info.
        // Hence, DbMeta has the exception of access physical file directly,
        // not through BufferPool.
        let mut file = File::open(file_path)?;
        let mut buf: Vec<u8> = vec![];
        // TODO read only what we need not the whole file into mem.
        file.read_to_end(&mut buf)?;
        let db_meta = DbMeta::parse(buf.as_slice())?;

        let page_size = db_meta.db_header.page_size;
        let disk_manager = DiskManager::new(
            file_path, page_size as usize
        )?;
        // TODO does Database need ref to DiskManager? why?
        //  If yes, how to have both database and buffer pool refs 1 obj DiskManager?
        // ownership of disk_manager is moved to BufferPool
        let buffer_pool = BufferPool::new(disk_manager);

        Ok(Database {
            db_meta,
            buffer_pool,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use crate::model::database::Database;

    #[test]
    fn test_database() {
        let db_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/resources/sample.db");
        let db = Database::new(db_path.as_path().to_str().unwrap()).unwrap();

        assert_eq!(db.db_meta.db_header.page_size, 4096);
    }
}
