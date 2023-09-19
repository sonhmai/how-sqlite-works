use datafusion_expr::LogicalPlan;

use crate::model::data_record::DataRecord;

pub struct Executor {}

impl Executor {
    pub fn execute(&self, logical_plan: LogicalPlan) -> Vec<DataRecord> {
        println!("executing logical plan {logical_plan:#?}");

        match logical_plan {
            LogicalPlan::TableScan(table_scan) => {
                println!(
                    "Scanning table {} projection {:?}",
                    table_scan.table_name, table_scan.projection
                );
                vec![]
            }
            LogicalPlan::Projection(projection) => {
                println!("Projecting expressions {:#?}", projection.expr);
                vec![]
            }
            _ => {
                println!("error executing plan {logical_plan:#?}");
                vec![]
            }
        }
    }
}
