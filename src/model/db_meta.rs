const ROOT_PAGE_OFFSET: u8 = 100;
const NUM_CELL_OFFSET: u8 = 3;

pub struct DbMeta {
    page_size: u32,
    num_tables: u32,
}
