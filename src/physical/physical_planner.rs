use anyhow::bail;
use datafusion_expr::LogicalPlan;

use crate::physical::plan::exec::Exec;
use crate::physical::plan::exec_apples_scan::ExecApplesScan;
use crate::physical::plan::exec_dummy::ExecDummy;
use crate::physical::plan::exec_projection::ExecProjection;
use crate::physical::plan::exec_scan::ExecScan;
use crate::physical::expression::col_by_index::PhysicalColByIndex;
use crate::physical::expression::physical_expr::PhysicalExpr;

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
    pub fn plan(&self, logical_plan: &LogicalPlan) -> Box<dyn Exec> {
        println!("executing logical plan \n{logical_plan:?}");

        match logical_plan {
            LogicalPlan::TableScan(table_scan) => {
                println!(
                    "Scanning table {} projection {:?}",
                    table_scan.table_name, table_scan.projection
                );
                Box::new(ExecScan {})
            }
            LogicalPlan::Projection(logical_proj) => {
                let physical_expressions = logical_proj
                    .expr
                    .iter()
                    .map(|logical_expr|
                        // knowing that logical plan is Projection having only 1 input -> access idx 0
                        create_physical_expr(&logical_expr, logical_plan.inputs()[0]
                        ).expect("cannot parse physical expr"))
                    .collect();
                Box::new(ExecProjection {
                    input: Box::new(ExecApplesScan {}),
                    expressions: physical_expressions,
                })
            }
            _ => {
                println!("error executing plan {logical_plan:#?}");
                // TODO make return type Result with error
                Box::new(ExecDummy {})
            }
        }
    }
}

pub fn create_physical_expr(
    logical_expr: &datafusion_expr::Expr,
    input: &LogicalPlan,
) -> anyhow::Result<Box<dyn PhysicalExpr>> {
    match logical_expr {
        datafusion_expr::Expr::Column(col) => {
            let schema = input.schema();
            let col_index = schema.index_of_column(&col)?;
            Ok(Box::new(PhysicalColByIndex { col_index }))
        }
        _ => bail!("cannot create physical expr from {logical_expr}")
    }
}

#[test]
fn test_create_case_expr() {

}
