use crate::physical::exec::Exec;
use crate::physical::exec_apples_scan::ExecApplesScan;
use crate::physical::exec_dummy::ExecDummy;
use crate::physical::exec_projection::ExecProjection;
use crate::physical::exec_scan::ExecScan;
use datafusion_expr::LogicalPlan;

pub struct PhysicalPlanner {}

impl PhysicalPlanner {
    ///
    /// Box puts a type on heap instead of stack.
    /// What:
    ///     Box is a smart pointer, a reference just like & in &str.
    ///     A reference pointer has fixed size.
    /// When:
    ///     use for types the compiler does not know the size. Example Exec trait here
    ///     can be many types so we don't know the size at compile time.
    ///
    pub fn plan(&self, logical_plan: LogicalPlan) -> Box<dyn Exec> {
        println!("executing logical plan \n{logical_plan:?}");

        match logical_plan {
            LogicalPlan::TableScan(table_scan) => {
                println!(
                    "Scanning table {} projection {:?}",
                    table_scan.table_name, table_scan.projection
                );
                Box::new(ExecScan {})
            }
            LogicalPlan::Projection(logical_proj) => Box::new(ExecProjection {
                input: Box::new(ExecApplesScan {}),
                expressions: logical_proj.expr,
            }),
            _ => {
                println!("error executing plan {logical_plan:#?}");
                // TODO make return type Result with error
                Box::new(ExecDummy {})
            }
        }
    }
}
