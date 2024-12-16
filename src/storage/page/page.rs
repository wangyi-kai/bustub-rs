
pub struct Slot {
    offset: u16,
    length: u16,
}

impl Slot {
    pub fn to_u8(&mut self) -> &[u8] {
        let bytes = unsafe {
            let ptr = self as *const Self as *const u8;
            std::slice::from_raw_parts(ptr, std::mem::size_of(Self))
        };
        bytes
    }

    pub fn from_u8(&mut self, bytes: &[u8]) -> Self {
        let slot: Slot = unsafe {
            std::ptr::read(bytes.as_ptr() as *const Self);
        };
        slot
    }
}
pub struct PageHeader {
    page_id: u32,
    lsn: u32,
    prev_page_id: u32,
    next_page_id: u32,
    free_space_offset: u32,
}

impl PageHeader {
    const SIZE: usize = std::mem::size_of(Self);
    #[inline]
    pub fn to_u8(&mut self) -> &[u8] {
        let bytes = unsafe {
            let ptr = self as *const Self as *const u8;
            std::slice::from_raw_parts(ptr, std::mem::size_of(self))
        };
        bytes
    }

    #[inline]
    pub fn from_u8(&mut self, bytes: &[u8]) -> Self {
        let header = unsafe {
            std::ptr::read(bytes.as_ptr() as *const Self)
        };
        // let self = unsafe {
        //     let header = bytes.as_ptr() as *const PageHeader;
        //     *header
        // };
        header
    }
}

pub struct SlotData {

}

pub struct Page {
    header: PageHeader,
    slot: Vec<Slot>,
    data: Vec<u8>,
}

