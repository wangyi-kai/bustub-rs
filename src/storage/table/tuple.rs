use crate::common::{*};
use crate::common::rid::RID;

pub struct Tuple {
    allocated: bool,
    size: u32,
    data: Vec<u8>,
    rid: RID,
}

impl Tuple {
    pub fn new() -> Self {

    }
}