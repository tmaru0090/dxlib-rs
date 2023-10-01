pub struct DxFontData {}
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
