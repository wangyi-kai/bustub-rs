use super::column::{*};

#[derive(Clone, Debug)]
pub struct Schema {
    /// Fixed-length column size
    length: u32,
    /// All the columns in the schema, inlined and uninlined.
    columns: Vec<Column>,
    /// True if all the columns are inlined, false otherwise
    tuple_is_inlined: bool,
    /// Indices of all unlined columns.
    unlined_columns_: Vec<u32>,
}

impl Schema {
    #[inline]
    pub fn get_columns(&self) -> Vec<Column> {
        self.columns.clone()
    }

    #[inline]
    pub fn get_column(&self, idx: usize) -> Column {
        unsafe {
            *self.columns.get_unchecked(idx).clone()
        }
    }

    #[inline]
    pub fn get_column_size(&self) -> usize {
        self.columns.len()
    }

    #[inline]
    pub fn get_unlined_column(&self) -> Vec<u32> {
        self.unlined_columns_.clone()
    }

    #[inline]
    pub fn get_unlined_column_count(&self) -> usize {
        self.unlined_columns_.len()
    }

    #[inline]
    pub fn get_col_idx(&self, col_name: String) -> usize {
        let idx = self.get_idx(col_name);
        if idx >= 0 {
            return idx as usize;
        }
        panic!("Column does not exist");
    }

    #[inline]
    fn get_idx(&self, name: String) -> i32 {
        for i in 0..self.columns.len() {
            if self.columns.get(i).unwrap().get_name() == name {
                return (i as i32)
            }
        }
        -1
    }
}

