
#[derive(Debug)]
pub struct IndexDef {
    pub table: String,
    pub name: String,
    pub cols: Vec<String>,
}

pub fn order_satisfied_by_index(required: &[String], index: &IndexDef) -> bool {
    // Accept prefix match: ORDER BY (a) or (a,b) satisfied by index (a,b,...) .
    if required.is_empty() { 
        return false; 
    }
    if required.len() > index.cols.len() { 
        return false; 
    }
    required.iter().zip(index.cols.iter()).all(|(r, i)| r == i)
}