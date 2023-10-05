use dxlib::*;
use dxlib_ffi::*;
use std::mem::size_of;
use std::ops::Drop;
use winapi::um::wingdi::{AddFontResourceExA, RemoveFontResourceExA, FR_PRIVATE};

pub struct DxFontData {
    font_path: String,
    size: i32,
    thick: i32,
}
impl DxFontData {
    pub fn new() -> DxFontData {
        return DxFontData {
            font_path: String::new(),
            size: 0,
            thick: 0,
        };
    }
}

pub struct DxFont {
    data: DxFontData,
}
impl DxFont {
    pub fn new() -> DxFont {
        return DxFont {
            data: DxFontData {
                font_path: String::new(),
                size: 0,
                thick: 0,
            },
        };
    }
    pub fn add_resouce_data(&mut self, path: &str) -> Result<(), String> {
        let result = unsafe {
            AddFontResourceExA(path.to_cstring().as_ptr(), FR_PRIVATE, std::ptr::null_mut())
        };
        if result == 0 {
            return Err("フォントリソースの追加に失敗しました".to_string());
        }
        return Ok(());
    }
    pub fn create_font(
        &mut self,
        path: &str,
        size: i32,
        thick: i32,
        font_type: i32,
    ) -> Result<i32, String> {
        unsafe {
            let res = self.add_resouce_data(path);
            match res {
                Ok(_) => {}
                Err(val) => {
                    return Err(val);
                }
            }
            let handle = dx_CreateFontToHandle(path, size, thick, font_type);
            return Ok(handle);
        }
    }
    pub fn delete_resouce_data(&self, path: &str) -> Result<(), String> {
        // フォントリソースを削除する
        let result = unsafe {
            RemoveFontResourceExA(path.to_cstring().as_ptr(), FR_PRIVATE, std::ptr::null_mut())
        };
        if result == 0 {
            return Err("フォントリソースの削除に失敗しました".to_string());
        }
        return Ok(());
    }
}
impl Drop for DxFont {
    fn drop(&mut self) {
        /* let res = self.delete_resouce_data();
        match res {
            Ok(val) => {
                println!("Ok({:?})", val);
            }
            Err(val) => {
                println!("Err({:?})", val);
            }
        }
        */
    }
}
