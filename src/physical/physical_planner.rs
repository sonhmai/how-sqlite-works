use anyhow::bail;
use datafusion_expr::LogicalPlan;
use log::{error, info};
use std::cell::RefCell;
use std::rc::Rc;

use crate::model::column_value::ColumnValue;
use crate::model::database::Database;
use crate::physical::expression::col_by_index::PhysicalColByIndex;
use crate::physical::expression::literal::PhysicalLiteral;
use crate::physical::expression::physical_expr::PhysicalExpr;
use crate::physical::plan::exec::Exec;
use crate::physical::plan::exec_apples_scan::ExecApplesScan;
use crate::physical::plan::exec_dummy::ExecDummy;
use crate::physical::plan::join::ExecJoinHash;
use crate::physical::plan::exec_projection::ExecProjection;
use crate::physical::plan::exec_scan::ExecScan;

pub struct PhysicalPlanner {
    pub database: Rc<RefCell<Database>>,
}

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
        info!("executing logical plan \n{logical_plan:?}");

        match logical_plan {
            LogicalPlan::TableScan(table_scan) => {
                info!(
                    "Scanning table {} projection {:?}",
                    table_scan.table_name, table_scan.projection
                );
                // TODO root page number should not be hardcoded but looked up in db meta
                let table_page_number = 2; // hard-coded for sample.db, table apples

                Box::new(ExecScan::new(
                    table_scan.table_name.to_string(),
                    table_page_number,
                    self.database.clone(),
                ))
            }
            LogicalPlan::Projection(logical_proj) => {
                let physical_expressions = logical_proj
                    .expr
                    .iter()
                    .map(|logical_expr|
                        // knowing that logical plan is Projection having only 1 input -> access idx 0
                        create_physical_expr(&logical_expr, logical_plan.inputs()[0],
                        ).expect("cannot parse physical expr"))
                    .collect();
                // * to defer the smart ptr input: Arc<datafusion LogicalPlan>,
                // then take a reference with &
                let input_physical_plan = self.plan(&*logical_proj.input);

                Box::new(ExecProjection {
                    input: input_physical_plan,
                    expressions: physical_expressions,
                })
            }

            LogicalPlan::Join(join) => {
                let left_physical = self.plan(&join.left);
                let right_physical = self.plan(&join.right);

                todo!()
            }

            _ => {
                error!("error executing plan {logical_plan:#?}");
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
        datafusion_expr::Expr::Literal(scalar) => {
            let column_value = ColumnValue::One;
            Ok(Box::new(PhysicalLiteral {
                value: column_value,
            }))
        }
        _ => bail!("cannot create physical expr from {logical_expr}"),
    }
}

#[test]
fn test_create_case_expr() {}
