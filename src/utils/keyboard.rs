use crate::common::error::DxError;
use crate::ffi::dxlib::*;
use anyhow::anyhow;
use anyhow::Result as Res;

#[derive(Clone, Debug)]
pub struct KeyBoard {
    key_state: [i32; 256],
    key_buf: [i8; 256],
    press_cnt: [i32; 256],
    release_cnt: [i32; 256],
}
impl KeyBoard {
    pub fn new() -> KeyBoard {
        return KeyBoard {
            key_state: [0; 256],
            key_buf: [0; 256],
            press_cnt: [0; 256],
            release_cnt: [0; 256],
        };
    }
    pub fn update(&mut self) -> Res<()> {
        unsafe {
            // キーの状態を取得
            let res = dx_GetHitKeyStateAll(self.key_buf.as_mut_ptr());
            if res == -1 {
                return Err(anyhow!(DxError::new(
                    "関数 GetHitKeyStateAllでエラーが発生しました",
                    res,
                )));
            }
            // キーの状態を更新
            for i in 0..256 {
                if self.key_buf[i] != 0 {
                    self.press_cnt[i] += 1;
                    if self.release_cnt[i] > 0 {
                        self.release_cnt[i] = 0;
                    }
                } else {
                    self.press_cnt[i] = 0;
                    self.release_cnt[i] += 1;
                    if self.release_cnt[i] < 0 {
                        self.release_cnt[i] = 0;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn wait_until_key_pressed(&self, key_code: i32) {
        // 指定されたキーコードが1以上になるまで待機
        if self.get_press_cnt(key_code) != TRUE {
            // ウェイトを入れて、無駄な処理負荷を軽減します
            //thread::sleep(Duration::from_millis(10));
            unsafe {
                dx_WaitTimer(100);
            }
        }
    }
    pub fn is_available_code(&self, key_code: i32) -> bool {
        if 0 >= key_code && key_code > 256 {
            return false;
        }
        return true;
    }
    pub fn get_release_cnt(&self, key_code: i32) -> i32 {
        if !self.is_available_code(key_code) {
            return -1;
        }
        return self.release_cnt[key_code as usize];
    }
    pub fn get_press_cnt(&self, key_code: i32) -> i32 {
        if !self.is_available_code(key_code) {
            return -1;
        }
        return self.press_cnt[key_code as usize];
    }
    pub fn get_key_state(&self, key_code: i32) -> i32 {
        if !self.is_available_code(key_code) {
            return -1;
        }
        return self.key_state[key_code as usize];
    }
}
