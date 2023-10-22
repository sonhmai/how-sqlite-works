use std::cell::RefCell;
use std::rc::Rc;

use crate::btree::bt_cursor::BtCursor;
use crate::model::data_record::DataRecord;
use crate::model::database::Database;
use crate::physical::plan::exec::Exec;

#[derive(Debug)]
pub struct ExecScan {
    pub table_name: String,
    pub table_page_number: u32,
    pub database: Rc<RefCell<Database>>,
    bt_cursor: BtCursor,
}

impl ExecScan {
    pub fn new(
        table_name: String,
        table_page_number: u32,
        database: Rc<RefCell<Database>>
    ) -> Self {
        let bt_cursor = BtCursor::new(database.clone(), table_page_number);
        ExecScan {
            table_name,
            table_page_number,
            database,
            bt_cursor,
        }
    }
}

impl Exec for ExecScan {
    fn execute(&mut self) -> Vec<DataRecord> {
        let mut records = Vec::new();

        while let Some(record) = self.bt_cursor.move_to_next() {
            records.push(record);
        }

        records
    }
}
