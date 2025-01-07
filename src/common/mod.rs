pub mod rid;
pub mod error;

pub const INVALID_PAGE_ID: i32 = -1;
pub const INVALID_TXN_ID: i64 = -1;
pub const META_PAGE: i32 = 0;
pub const PAGE_SIZE: u32 = 4 * 1024;
pub const BUFFER_POOL_SIZE: u32 = 10;
pub const LRUK_REPLACER_K: u32 = 10;
type frame_id_t = i32;
type page_id_t = i32;
type txn_id_t = i32;
type lsn_t = i32;
type slot_offset_t = usize;