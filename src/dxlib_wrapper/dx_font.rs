use crate::dx_resouce::*;
use dxlib::*;
use dxlib_ffi::*;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Drop;
use winapi::um::wingdi::{AddFontResourceExA, RemoveFontResourceExA, FR_PRIVATE};
/*
pub trait HashMgr {
    type Key;
    type Val;
    type GetVal;
    fn get(&self, key: Self::Key) -> Option<Self::GetVal>;
    fn add(&mut self, key: Self::Key, val: Self::Val) -> &Self;
    fn remove(&mut self, key: Self::Key) -> &Self;
}

/*
pub trait HandleMgr{
    type MaxNum;
    type Handle;

    fn get(&mut self,key:Self::Key)->Self;
    fn add(&mut self,key:Self::Key,val:Self::Handle)->Self;
    fn remove(&mut self,key:Self::Key)->Self;
}
*/
*/
#[derive(Debug, Clone)]
pub struct DxFontData {
    font_name: String,
    font_handle: i32,
    font_path: String,
    font_size: i32,
    font_thick: i32,
    font_type: i32,
}

impl DxFontData {
    pub fn new() -> DxFontData {
        return DxFontData {
            font_name: String::new(),
            font_handle: 0,
            font_path: String::new(),
            font_size: 0,
            font_thick: 0,
            font_type: 0,
        };
    }
    pub fn new_with_params(
        font_path: &str,
        font_name: &str,
        font_size: i32,
        font_thick: i32,
        font_type: i32,
    ) -> DxFontData {
        return DxFontData {
            font_path: String::from(font_path),
            font_name: String::from(font_name),
            font_size: font_size,
            font_thick: font_thick,
            font_type: font_type,
            font_handle: 0,
        };
    }
}
impl DxResouce for DxFont {
    type Config = DxFontData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, String> {
        unsafe {
            self.data = config.clone();
            let name = self.data.font_name.clone();
            let path = self.data.font_path.clone();
            let size = self.data.font_size;
            let thick = self.data.font_thick;
            let font_type = self.data.font_type;
            let res = self.add_resouce_data(&path);
            match res {
                Ok(_) => {
                    let handle = dx_CreateFontToHandle(&name, size, thick, font_type);
                    self.data.font_handle = handle;
                    if handle == -1 {
                        return Err("フォントハンドルの生成に失敗しました".to_string());
                    }
                }
                Err(val) => {
                    return Err(val);
                }
            }
        }

        return Ok(self);
    }
    fn get(&self) -> Result<Self::GetVal, String> {
        if self.data.font_handle != -1 {
            return Ok(self.data.font_handle);
        } else {
            return Err("".to_string());
        }
    }
    fn delete(&mut self) -> Result<&mut Self, String> {
        let path = self.data.font_path.clone();
        let res = self.remove_resouce_data(&path);
        match res {
            Ok(_) => {
                let res = unsafe { dx_DeleteFontToHandle(self.data.font_handle) };
                if res == -1 {
                    return Err("フォントハンドルの削除に失敗しました".to_string());
                }
            }
            Err(val) => {
                return Err(val);
            }
        }
        return Ok(self);
    }
}
/*
impl HashMgr for DxFont {
    // Path
    type Key = String;
    // Handle
    type Val = i32;
    // Handle
    type GetVal = i32;
    // ハンドルの取得
    fn get(&self, key: Self::Key) -> Option<Self::GetVal> {
        return Some(*self.data.key_handle.get(&key).unwrap());
    }
    // フォントハンドルをパスをキーとして追加
    fn add(&mut self, key: Self::Key, val: Self::Val) -> &DxFont {
        self.data.font_handle.insert(key.to_string(), val);
        return self;
    }
    // 指定のキーのハンドルを削除
    fn remove(&mut self, key: Self::Key) -> &DxFont {
        if self.remove_resouce_data(&self.data.font_path).is_ok(){
            unsafe{
                dx_DeleteFontToHandle(*self.data.key_handle.get(&key).unwrap());
            }
        }

        self.data.key_handle.remove(&key);
        return self;
    }
}
*/

pub struct DxFont {
    data: DxFontData,
}

impl DxFont {
    pub fn new() -> DxFont {
        return {
            DxFont {
                data: DxFontData {
                    font_name: String::new(),
                    font_handle: 0,
                    font_path: String::new(),
                    font_size: 0,
                    font_thick: 0,
                    font_type: 0,
                },
            }
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

    // 新しくフォントハンドルを作成
    pub fn create_font(
        &mut self,
        name: &str,
        size: i32,
        thick: i32,
        font_type: i32,
    ) -> Result<&mut DxFont, String> {
        unsafe {
            let mut path = self.data.font_path.clone();
            self.data.font_size = size;
            self.data.font_thick = thick;
            let res = self.add_resouce_data(&path);
            match res {
                Ok(_) => {
                    let handle = dx_CreateFontToHandle(name, size, thick, font_type);
                    self.data.font_handle = handle;
                    if handle == -1 {
                        return Err("フォントハンドルの生成に失敗しました".to_string());
                    }
                }
                Err(val) => {
                    return Err(val);
                }
            }
        }

        return Ok(self);
    }
    pub fn get(&self) -> i32 {
        return self.data.font_handle;
    }
    pub fn remove_resouce_data(&self, path: &str) -> Result<(), String> {
        // フォントリソースを削除する
        let result = unsafe {
            RemoveFontResourceExA(path.to_cstring().as_ptr(), FR_PRIVATE, std::ptr::null_mut())
        };
        if result == 0 {
            return Err("フォントリソースの削除に失敗しました".to_string());
        }
        return Ok(());
    }
    pub fn delete_font(&mut self) -> Result<(), String> {
        let mut path = self.data.font_path.clone();
        let res = self.remove_resouce_data(&path);
        match res {
            Ok(_) => {
                let res = unsafe { dx_DeleteFontToHandle(self.data.font_handle) };
                if res == -1 {
                    return Err("フォントハンドルの削除に失敗しました".to_string());
                }
            }
            Err(val) => {
                return Err(val);
            }
        }
        return Ok(());
    }
    /*
    pub fn get_handle(&self) -> Result<i32, String> {
        if self.data.font_handle != -1 {
            return Ok(self.data.font_handle);
        } else {
            return Err("ハンドルが無効です".to_string());
        }
    }
    */
}
impl Drop for DxFont {
    fn drop(&mut self) {
        let _ = self.delete_font();
    }
}
