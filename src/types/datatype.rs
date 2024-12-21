use std::fmt::{Display, Formatter};

/// Base Data Type, refer MySql
#[derive(Debug, Clone)]
pub enum DataType {
    /// 4 bytes
    Boolean,
    /// 1 byte
    TinyInt,
    /// 2 bytes
    SmallInt,
    /// 4 bytes
    Int,
    /// 8 bytes
    BigInt,
    /// 4 bytes
    Float,
    /// 8 bytes
    Double,
    /// fix-length string 1 byte
    Char(usize),
    /// unfix-length string
    Varhar(usize),
    /// binary form long text data
    Blob(usize),
    /// (precision, scale)
    Decimal(u32, u32),
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Boolean => write!(f, "BOOLEAN"),
            DataType::TinyInt => write!(f, "TINYINT"),
            DataType::SmallInt => write!(f, "SMALLINT"),
            DataType::Int => write!(f, "INT"),
            DataType::BigInt => write!(f, "BIGINT"),
            DataType::Float => write!(f, "FLOAT"),
            DataType::Double => write!(f, "DOUBLE"),
            DataType::Char(len) => write!(f, "CHAR({})", len),
            DataType::Varchar(len) => write!(f, "VARCHAR({})", len),
            DataType::Blob(len) => write!(f, "BLOB({})", len),
            DataType::Decimal(a, b) => write!(f, "DECIMAL({},{})", a, b),
        }
    }
}

impl DataType {
    pub fn get_size(&self) -> usize {
        match self {
            DataType::Boolean | DataType::Int | DataType::Float => 4,
            DataType::BigInt | DataType::Double => 8,
            DataType::TinyInt => 1,
            DataType::SmallInt => 2,
            DataType::Char(len) => *len,
            DataType::Varchar(len) => *len,
            DataType::Blob(len) => *len,
            DataType::Decimal(a, b) => {
                let groups = (*a as usize + 8) / 9;
                let mut bytes = groups * 9;
                if *b > 0 {
                    bytes += 1;
                }
                bytes
            }
        }
    }
}

