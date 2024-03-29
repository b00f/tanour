use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum TestMsg {
    #[n(0)]
    Mul {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(1)]
    Div {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(2)]
    WriteData {
        #[n(0)]
        offset: u32,
        #[n(1)]
        data: Vec<u8>,
    },

    #[n(3)]
    ReadData {
        #[n(0)]
        offset: u32,
        #[n(1)]
        length: u32,
    },
}
