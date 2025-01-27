use crate::single::*;

#[allow(non_camel_case_types)]
pub enum EventKind {
    DISPATCH_SEND_ROW,
    DISPATCH_RECEIVED_ROW([Single; 6]),
    DISPATCH_SEND_PIECES(Vec<Single>),
    DISPATCH_RECEIVED_PIECES(Vec<(Single, u8, u8)>),
    DISPATCH_SEND_POWER,
}
