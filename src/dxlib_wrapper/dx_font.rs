use std::mem::size_of;
use winapi::um::wingdi::{AddFontResourceExA, RemoveFontResourceExA, FR_PRIVATE};
pub struct DxFontData {
    font_path:String,
}
impl DxFontData {
    fn new() -> DxFontData {
        return DxFontData {};
    }
}

pub struct DxFont {
    data: DxFontData,
}
impl DxFont {
    fn new() -> DxFont {
        return DxFont {
            data: DxFontData {},
        };
    }
}
