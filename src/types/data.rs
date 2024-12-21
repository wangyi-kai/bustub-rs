
#[derive(Debug, Clone)]
pub enum Data {
    Boolean(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Double(f64),
    Varchar(String),
}

impl Data {
    fn new(data: &[u8], type_id: usize) -> Data {
        unsafe {
            match type_id {
                0 => Data::BigInt(*(data.as_ptr() as *const i64)),
                _ => {}
            }
        }
    }
}