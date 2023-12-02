use std::collections::HashMap;
use std::sync::Arc;

use arrow_schema::{DataType, Field, Schema};
use datafusion_common::config::ConfigOptions;
use datafusion_common::{plan_err, DataFusionError, Result};
use datafusion_expr::builder::LogicalTableSource;
use datafusion_expr::{AggregateUDF, ScalarUDF, TableSource, WindowUDF};
use datafusion_sql::planner::ContextProvider;
use datafusion_sql::TableReference;

use crate::model::database::Database;

/// SqliteContextProvider is an extension of datafusion ContextProvider
/// for providing Catalog, Table, Schema, UDFs, etc. of sqlite and custom ones.
///
/// https://arrow.apache.org/datafusion/library-user-guide/catalogs.html
///
/// Here the schema is hardcoded. It should be parsed from the db file.
pub struct SqliteContextProvider {
    tables: HashMap<String, Arc<dyn TableSource>>,
    options: ConfigOptions,
}

fn create_table(fields: Vec<Field>) -> Arc<dyn TableSource> {
    Arc::new(LogicalTableSource::new(Arc::new(
        Schema::new_with_metadata(fields, HashMap::new()),
    )))
}

fn create_table_source(fields: Vec<Field>) -> Arc<dyn TableSource> {
    Arc::new(LogicalTableSource::new(Arc::new(
        Schema::new_with_metadata(fields, HashMap::new()),
    )))
}

impl SqliteContextProvider {
    // Create ContextProvider from reading first page of db file for
    // db metadata (db info and schema objects
    pub fn new_for_db(database: &Database) -> SqliteContextProvider {
        let mut tables = HashMap::new();

        for schema_obj in &database.db_meta.schema_objects {
            let table_source = create_table_source(schema_obj.columns.clone());
            tables.insert(schema_obj.tbl_name.clone(), table_source);
        }

        SqliteContextProvider {
            tables,
            options: Default::default(),
        }
    }
}

impl ContextProvider for SqliteContextProvider {
    fn get_table_source(&self, name: TableReference) -> Result<Arc<dyn TableSource>> {
        match self.tables.get(name.table()) {
            Some(table_source) => Ok(table_source.clone()),
            _ => plan_err!("Table not found: {}", name.table()),
        }
    }

    fn get_table_provider(&self, name: TableReference) -> Result<Arc<dyn TableSource>> {
        match self.tables.get(name.table()) {
            Some(table_source) => Ok(table_source.clone()),
            _ => plan_err!("Table not found: {}", name.table()),
        }
    }

    fn get_function_meta(&self, name: &str) -> Option<Arc<ScalarUDF>> {
        None // we don't add any ScalarUDF
    }

    fn get_aggregate_meta(&self, name: &str) -> Option<Arc<AggregateUDF>> {
        None
    }

    fn get_window_meta(&self, name: &str) -> Option<Arc<WindowUDF>> {
        None
    }

    fn get_variable_type(&self, variable_names: &[String]) -> Option<DataType> {
        None
    }

    fn options(&self) -> &ConfigOptions {
        &self.options
    }
}
