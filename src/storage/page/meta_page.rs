use std::mem;
pub struct TableMeta {
    table_name: String,
    record_size: usize,
    used_space: usize,
}

impl TableMeta {
    pub fn to_u8(&self) -> &[u8] {
        let bytes = unsafe {
            let ptr = self as *const Self as *const u8;
            std::slice::from_raw_parts(ptr, mem::size_of(Self))
        };
        bytes
    }

    pub fn from_u8(&self, buf: &[u8]) -> Self {
        let meta = unsafe {
            std::ptr::read(buf.as_ptr() as *const Self)
        };
        meta
    }
}