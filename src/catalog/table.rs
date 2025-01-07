use crate::catalog::column::Column;
use crate::catalog::schema::Schema;

#[derive(Clone, Debug)]
pub struct Table {
    /// database table name
    name: String,
    /// column
    column: Vec<Column>,
    /// The table ID
    table_id: u32,
    /// The table schema
    schema: Schema,
}