use std::fs::File;

use anyhow::Result;
use datafusion_common::ExprSchema;

fn get_file_size(file: &File) -> Result<u64> {
    let metadata = file.metadata()?;
    Ok(metadata.len())
}
