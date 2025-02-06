
#[derive(Debug, Clone)]
pub enum Data {
    None,
    Boolean(bool),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    Float(f32),
    Double(f64),
    Varchar(String),
}

#[derive(Clone, Debug)]
pub enum DataType {
    None,
    Boolean,
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Float,
    Double,
    Varchar,
}

#[derive(Debug, Clone)]
#[repr(i32)]
pub enum TypeId {
    INVALID = 0,
    BOOLEAN = 1,
    TINYINT = 2,
    SMALLINT = 3,
    INT = 4,
    BIGINT = 5,
    FLOAT = 6,
    DOUBLE = 7,
    VARCHAR = 8,
}

// impl DataType {
//     #[inline]
//     pub fn get_size(&self) -> usize {
//         match self {
//             DataType::None => 0,
//             DataType::Boolean => 1,
//             DataType::SmallInt => 2,
//             DataType::TinyInt => 1,
//             DataType::Int => 4,
//             DataType::BigInt => 8,
//             DataType::Float => 4,
//             DataType::Double => 8,
//             Data::Varchar(s) => s.len()
//         }
//     }
// }

impl Data {
    #[inline]
    pub fn get_size(&self) -> usize {
        match self {
            Data::None => 0,
            Data::Boolean(_) => 1,
            Data::SmallInt(_) => 2,
            Data::TinyInt(_) => 1,
            Data::Int(_) => 4,
            Data::BigInt(_) => 8,
            Data::Float(_) => 4,
            Data::Double(_) => 8,
            Data::Varchar(s) => s.len()
        }
    }

    #[inline]
    pub fn get_data_type(&self) -> DataType {
        match self {
            Data::None => DataType::None,
            Data::Boolean(_) => DataType::Boolean,
            Data::SmallInt(_) => DataType::SmallInt,
            Data::TinyInt(_) => DataType::TinyInt,
            Data::Int(_) => DataType::Int,
            Data::BigInt(_) => DataType::BigInt,
            Data::Float(_) => DataType::Float,
            Data::Double(_) => DataType::Double,
            Data::Varchar(_) => DataType::Varchar,
        }
    }

    #[inline]
    pub fn to_vec(self) -> Vec<u8> {
        match self {
            Data::Varchar(v) => v.into_bytes(),
            Data::TinyInt(v) | Data::Int(v) | Data::BigInt(v) | Data::SmallInt(v) => v.to_be_bytes().to_vec(),
            Data::Boolean(v) => if v { vec![1] } else { vec![0] },
            Data::Float(v) | Data::Double(v) => v.to_be_bytes().to_vec(),
            Data::None => { vec![] }
        }
    }
}


impl TypeId {
    fn new(data: &[u8], type_id: TypeId) -> Data {
        unsafe {
            match type_id {
                TypeId::INVALID => Data::None,
                TypeId::BOOLEAN => Data::Boolean(if *data.get_unchecked(0) == 1 { false } else { true }),
                TypeId::TINYINT => Data::TinyInt(*(data.as_ptr() as *const i8)),
                TypeId::SMALLINT => Data::SmallInt(*(data.as_ptr() as *const i16)),
                TypeId::INT => Data::Int(*(data.as_ptr() as *const i32)),
                TypeId::BIGINT => Data::BigInt(*(data.as_ptr() as *const i64)),
                TypeId::FLOAT => Data::Float(*(data.as_ptr() as *const f32)),
                TypeId::DOUBLE => Data::Double(*(data.as_ptr() as *const f64)),
                TypeId::VARCHAR => Data::Varchar(String::from_utf8_unchecked(data.to_vec())),
            }
        }
    }
}