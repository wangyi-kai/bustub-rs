use crate::catalog::column::Column;

#[derive(Clone, Debug)]
pub struct Schema {
    /// Fixed-length column size
    length: u32,
    /// All the columns in the schema, inlined and unlined.
    columns: Vec<Column>,
    /// True if all the columns are inlined, false otherwise
    tuple_is_inlined: bool,
    /// Indices of all unlined columns.
    unlined_columns: Vec<u32>,
}

impl Schema {
    pub fn new(columns: Vec<Column>) -> Self {
        let mut cur_offset = 0;
        let mut cols = Vec::new();
        let mut length = 0;
        let mut is_inlined = false;
        let mut unlined_columns = Vec::new();
        for (i, mut column) in columns.into_iter().enumerate() {
            if !column.is_inlined() {
                unlined_columns.push(i as u32);
            }
            column.set_col_offset(cur_offset);
            cur_offset += column.get_fixed_length();
            cols.push(column.clone());
        }
        length = cur_offset;
        Self {
            length,
            columns: cols,
            tuple_is_inlined: is_inlined,
            unlined_columns
        }
    }

    pub fn from_schema(from: &Schema, attrs: Vec<u32>) -> Self {
        let mut cols = Vec::with_capacity(attrs.len());
        for idx in attrs.iter() {
            cols.push(from.get_column(*idx as usize));
        }
        Self::new(cols)
    }

    #[inline]
    pub fn get_columns(&self) -> Vec<Column> {
        self.columns.clone()
    }

    #[inline]
    pub fn get_length(&self) -> u32 {
        self.length
    }

    #[inline]
    pub fn get_column(&self, idx: usize) -> Column {
        unsafe {
            *self.columns.get_unchecked(idx).clone()
        }
    }

    #[inline]
    pub fn get_column_count(&self) -> usize {
        self.columns.len()
    }

    #[inline]
    pub fn get_unlined_columns(&self) -> Vec<u32> {
        self.unlined_columns.clone()
    }

    #[inline]
    pub fn get_unlined_column_count(&self) -> usize {
        self.unlined_columns.len()
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

