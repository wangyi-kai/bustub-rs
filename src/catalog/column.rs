use std::cmp::PartialEq;
use crate::types::data::{*};

#[derive(Debug, Clone)]
pub struct Column {
    /// field name
    col_name: String,
    /// field type
    col_type: TypeId,
    /// the size of the fixed length column
    fixed_length: usize,
    /// the length of the variable length column
    variable_length: usize,
    /// Column offset in the tuple
    col_offset: u32,
}

impl Column {
    pub fn new_with_fix(name: String, data_type: TypeId) -> Self {
        let length = data_type.get_size();
        Self {
            col_name: name,
            col_type: data_type,
            fixed_length: length,
            variable_length: 0,
            col_offset: 0,
        }
    }

    pub fn new_with_unfix(name: String, data_type: TypeId, length: usize) -> Self {
        Self {
            col_name: name,
            col_type: data_type,
            fixed_length: 0,
            variable_length: length,
            col_offset: 0,
        }
    }

    #[inline]
    pub fn get_name(&self) -> String {
        self.col_name.clone()
    }

    #[inline]
    pub fn get_type(&self) -> TypeId {
        self.col_type.clone()
    }

    #[inline]
    pub fn get_fixed_length(&self) -> u32 {
        self.fixed_length as u32
    }

    #[inline]
    pub fn get_variable_length(&self) -> usize {
        self.variable_length.clone()
    }

    #[inline]
    pub fn get_offset(&self) -> u32 {
        self.col_offset
    }

    #[inline]
    pub fn is_inlined(&self) -> bool {
        match self.col_type {
            TypeId::VARCHAR => {
                true
            }
            _ => {
                false
            }
        }
    }

    #[inline]
    pub fn get_length(&self) -> usize {
        if self.is_inlined() {
            return self.fixed_length
        }
        self.variable_length
    }

    #[inline]
    pub fn set_col_offset(&mut self, offset: u32) {
        self.col_offset = offset
    }
}