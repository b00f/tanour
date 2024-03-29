#[derive(Debug)]
pub struct Page {
    pub offset: usize,
    pub length: usize,
    pub data: Vec<u8>,
    pub updated: bool,
}
impl Page {
    pub fn new(offset: usize, length: usize, data: Vec<u8>) -> Self {
        Page {
            offset,
            length,
            data,
            updated: false,
        }
    }
}
