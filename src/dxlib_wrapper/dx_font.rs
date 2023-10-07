use dxlib::*;
use dxlib_ffi::*;
use std::collections::HashMap;
use std::mem::size_of;
use std::ops::Drop;
use winapi::um::wingdi::{AddFontResourceExA, RemoveFontResourceExA, FR_PRIVATE};
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

pub struct DxFontData {
    font_handle: HashMap<String, i32>,
    key_handle: HashMap<String, i32>,
    font_path: String,
    font_size: i32,
    font_thick: i32,
    font_type: i32,
}

impl DxFontData {
    pub fn new() -> DxFontData {
        return DxFontData {
            font_handle: HashMap::new(),
            key_handle: HashMap::new(),
            font_path: String::new(),
            font_size: 0,
            font_thick: 0,
            font_type: 0,
        };
    }
}

impl HashMgr for DxFont {
    // Path
    type Key = String;
    // Handle
    type Val = i32;
    // Handle
    type GetVal = i32;
    // ハンドルの取得
    fn get(&self, key: Self::Key) -> Option<Self::GetVal> {
        //let path = self.hash_path.get(key).unwrap();
        return Some(0);
    }
    // フォントハンドルをパスをキーとして追加
    fn add(&mut self, key: Self::Key, val: Self::Val) -> &DxFont {
        self.data.font_handle.insert(key.to_string(), val);
        return self;
    }
    // 指定のキーのハンドルを削除
    fn remove(&mut self, key: Self::Key) -> &DxFont {
        return self;
    }
}
pub struct DxFont {
    data: DxFontData,
}

impl DxFont {
    pub fn new() -> DxFont {
        return {
            DxFont {
                data: DxFontData::new(),
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
    // 生成したハンドルに別名をつけて、キーとして登録
    pub fn add(&mut self, key: &str) -> Result<&DxFont, String> {
        let handle = self.data.font_handle.get(&self.data.font_path).unwrap();
        self.data.key_handle.insert(key.to_string(),*handle);
        return Ok(self);
    }
    // 新しくフォントハンドルを作成
    pub fn create_font(
        &mut self,
        path: &str,
        name: &str,
        size: i32,
        thick: i32,
        font_type: i32,
    ) -> Result<&DxFont, String> {
        unsafe {
            self.data.font_path = path.to_string();
            self.data.font_size = size;
            self.data.font_thick = thick;
            let res = self.add_resouce_data(path);
            match res {
                Ok(_) => {}
                Err(val) => {
                    println!("Err({:?})", val);
                    return Err(val);
                }
            }
            let  handle = dx_CreateFontToHandle(name, size, thick, font_type);
            HashMgr::add(self,path.to_string(),handle);

        }

        return Ok(self);
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
        /*
        let res = self.delete_resouce_data(&self.data.font_path);
        match res {
            Ok(_) => {
                let res = unsafe { dx_DeleteFontToHandle(self.data.font_handle) };
                if res != -1 {
                    println!("CreateFontToHandle:フォントハンドルは正常に削除されました");
                }else{
                    println!("CreateFontToHandle:フォントハンドルの削除に失敗しました");
                }
            }
            Err(val) => {
                println!("Err({:?})", val);
            }
        }
        */
    }
}
