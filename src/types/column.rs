use crate::types::datatype::DataType;

#[derive(Debug, Clone)]
pub struct Column {
    /// field name
    col_name: String,
    /// field type
    col_type: DataType,
    /// the size of the fixed length column
    fixed_length: u32,
    /// the length of the variable length column
    variable_length: u32,
    /// Column offset in the tuple
    col_offset: u32,
}