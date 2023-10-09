
use crate::dx_common::dxlib::*;
use crate::dx_resouce::*;

#[derive(Debug, Clone)]
pub struct DxMusicData {
    music_path: String,
    music_handle: i32,
}
impl DxMusicData {
    pub fn new(path: &str) -> DxMusicData {
        return DxMusicData {
            music_path: String::from(path),
            music_handle: 0,
        };
    }
    fn new_with_params() {}
}
#[derive(Debug, Clone)]
pub struct DxMusic {
    data: DxMusicData,
}

impl DxResouce for DxMusic {
    type Config = DxMusicData;
    type GetVal = i32;
    fn create(&mut self, config: &Self::Config) -> Result<&mut Self, String> {
        self.data = config.clone();
        let path = self.data.music_path.clone();
        let handle = unsafe { dx_LoadMusicMem(&path) };
        if handle == -1 {
            return Err("ミュージックハンドルの生成に失敗しました".to_string());
        } else {
            self.data.music_handle = handle;
            return Ok(self);
        }
    }
    fn get(&self) -> Result<Self::GetVal, String> {
        if self.data.music_handle == -1 {
            return Err("ミュージックハンドルが無効です".to_string());
        } else {
            return Ok(self.data.music_handle);
        }
    }
    fn delete(&mut self) -> Result<&mut Self, String> {
        let res = unsafe { dx_DeleteMusicMem(self.data.music_handle) };
        if res == -1 {
            return Err("ミュージックハンドルの削除に失敗しました".to_string());
        } else {
            return Ok(self);
        }
    }
}

impl DxMusic {
    pub fn new() -> DxMusic {
        return DxMusic {
            data: DxMusicData {
                music_handle: 0,
                music_path: String::new(),
            },
        };
    }
}

