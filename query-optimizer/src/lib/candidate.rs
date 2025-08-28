
/// One possible physical plan with estimated cost and output cardinality.
#[derive(Clone, Debug)]
pub struct Candidate {
    pub(crate) mask: u64,
    /// Estimated total cost of this plan, according to cost model.
    pub(crate) cost: f64,
    /// Estimated cardinality - number of output rows.
    pub(crate) cardinality: f64,
    /// Plan name.
    pub(crate) plan: String,
}