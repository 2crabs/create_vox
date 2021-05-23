pub(crate) fn u32_to_array(a: u32) -> [u8; 4] {
    a.to_ne_bytes()
}

pub(crate) fn u16_to_array(a: u16) -> [u8; 2] {
    a.to_ne_bytes()
}
