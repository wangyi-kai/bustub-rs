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
    pub fn from_values(values: Vec<Data>, schema: &Schema) -> Self {
        assert_eq!(values.len(), schema.get_column_size());
        let mut tuple_size = schema.get_length() as usize;
        let unlined_cols = schema.get_unlined_columns();
        unlined_cols.into_iter().map(|idx| {
            tuple_size += values.get(idx as usize).unwrap().get_size() + 4;
        });
        let size = tuple_size;
        let mut data = Vec::with_capacity(size);
        let col_cnt = schema.get_column_count();
        let offset = schema.get_length();

        let idx = 0;
        while idx < col_cnt {
            let column = schema.get_column(idx);
            if !column.is_inlined() {
                if let Some(col_data) = values.get(idx) {
                    data.push(col_data.clone().to_vec());
                }
            }

        }


        Self {
            allocated: true,
            size: size as u32,
            data,
            rid: RID::default(),
        }
    }
}