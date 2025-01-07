mod page;
mod meta_page;

const PAGE_SIZE: usize = 1024 * 4;
const INVALID_PAGE_ID: i32 = -1;
const PAGE_HEADER_SIZE: usize = 20;