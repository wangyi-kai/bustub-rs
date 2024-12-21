use std::fmt;
use std::fmt::Formatter;

use super::{*};

#[derive(Debug, Clone, Copy)]
pub struct RID {
    page_id: i32,
    slot_num: u32,
}

impl PartialEq for RID {
    fn eq(&self, other: &Self) -> bool {
        self.page_id == other.page_id && self.slot_num == other.slot_num
    }
}

impl fmt::Display for RID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "page_id: {}\n slot_num: {}", self.page_id, self.slot_num)
    }
}

impl Default for RID {
    fn default() -> Self {
        Self {
            page_id: INVALID_PAGE_ID,
            slot_num: 0,
        }
    }
}

impl RID {
    pub fn new(page_id: i32, slot_num: u32) -> Self {
        Self {
            page_id,
            slot_num,
        }
    }
    #[inline(always)]
    pub fn get_page_id(&self) -> i32 {
        self.page_id
    }

    #[inline(always)]
    pub fn get_slot_num(&self) -> u32 {
        self.slot_num
    }

    #[inline(always)]
    pub fn set(&mut self, page_id: i32, slot_num: u32) {
        self.page_id = page_id;
        self.slot_num = slot_num;
    }
}