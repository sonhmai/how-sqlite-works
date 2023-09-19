use std::collections::HashMap;
use std::sync::Arc;

use arrow_schema::{DataType, Field, Schema};
use datafusion_common::{DataFusionError, plan_err, Result};
use datafusion_common::config::ConfigOptions;
use datafusion_expr::{AggregateUDF, ScalarUDF, TableSource, WindowUDF};
use datafusion_expr::builder::LogicalTableSource;
use datafusion_sql::planner::ContextProvider;
use datafusion_sql::TableReference;

///
/// Here the schema is hardcoded. It should be parsed from the db file
pub struct SchemaProvider {
    tables: HashMap<String, Arc<dyn TableSource>>,
}

fn create_table_source(fields: Vec<Field>) -> Arc<dyn TableSource> {
    Arc::new(LogicalTableSource::new(Arc::new(
        Schema::new_with_metadata(fields, HashMap::new())
    )))
}

impl SchemaProvider {
    pub fn new() -> SchemaProvider {
        let mut tables = HashMap::new();
        // inserting the tables existed in sample.db
        tables.insert(
            "apples".to_string(),
            create_table_source(vec![
                Field::new("id", DataType::Int32, false),
                Field::new("name", DataType::Utf8, false),
                Field::new("color", DataType::Utf8, false),
            ]),
        );
        SchemaProvider {
            tables
        }
    }
}

impl ContextProvider for SchemaProvider {
    fn get_table_provider(&self, name: TableReference) -> Result<Arc<dyn TableSource>> {
        match self.tables.get(name.table()) {
            Some(tableSource) => Ok(tableSource.clone()),
            _ => plan_err!("Table not found: {}", name.table()),
        }
    }

    fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>> {
        todo!()
    }

    fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>> {
        todo!()
    }

    fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>> {
        todo!()
    }

    fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType> {
        todo!()
    }

    fn options(&self) -> &ConfigOptions {
        todo!()
    }
}

