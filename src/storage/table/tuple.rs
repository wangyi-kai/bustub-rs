use serde_json::map::Values;
use crate::catalog::schema::Schema;
use crate::common::{*};
use crate::common::rid::RID;
use crate::types::data::Data;

pub struct Tuple {
    allocated: bool,
    size: u32,
    data: Vec<u8>,
    rid: RID,
}

impl Tuple {
    pub fn new(values: Vec<Data>, schema: Schema) -> Self {
        assert_eq!(values.len(), schema.get_column_size());
        let tuple_size = schema.get_length();
        for idx in schema.get_unlined_column().iter() {

        }
    }
}