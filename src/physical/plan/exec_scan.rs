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
    records: Vec<DataRecord>,
}

impl ExecScan {
    pub fn new(
        table_name: String,
        table_page_number: u32,
        database: Rc<RefCell<Database>>,
    ) -> Self {
        let bt_cursor = BtCursor::new(database.clone(), table_page_number);
        ExecScan {
            table_name,
            table_page_number,
            database,
            bt_cursor,
            records: vec![],
        }
    }
}

impl Exec for ExecScan {
    fn execute(&mut self) -> &[DataRecord] {
        let mut records = Vec::new();

        while self.bt_cursor.move_to_next().is_ok() {
            // TODO - parse record from Btree, not hardcode
            let payload = vec![1];
            let record = DataRecord::parse_from(1, &payload);

            records.push(record);
        }
        
        self.records = records;
        &self.records
    }
    
    fn schema(&self) -> arrow_schema::SchemaRef {
        todo!()
    }
}
