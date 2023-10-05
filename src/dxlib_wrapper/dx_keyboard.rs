use crate::dx_common::dxlib::*;
#[derive(Clone)]
pub struct DxKeyBoard {
    key_state: [i32; 256],
    key_buf: [i8; 256],
    press_cnt: [i32;256],
    release_cnt:[i32;256],
}
impl DxKeyBoard {
    pub fn new() -> Box<DxKeyBoard> {
        return Box::new(DxKeyBoard {
            key_state: [0; 256],
            key_buf: [0; 256],
            press_cnt: [0;256],
            release_cnt: [0;256],
        });
    }
    pub fn update(&mut self) {
        unsafe {
            // キーの状態を取得
            dx_GetHitKeyStateAll(self.key_buf.as_mut_ptr());
            // キーの状態を更新
            for i in 0..256 {
                //self.key_state[i] += 1;
                if self.key_buf[i] != 0 {
                    self.press_cnt[i] += 1;
                    if self.release_cnt[i] > 0 {
                        self.release_cnt[i] = 0;
                    }
                } else {
                    self.release_cnt[i] += 1;
                    if self.press_cnt[i] > 0 {
                        self.press_cnt[i] = 0;
                    }
                }
            }
        }
    }
    pub fn get_press_cnt(&self,key_code:i32)->i32{
        return self.press_cnt[key_code as usize];
    }
    pub fn get_key_state(&self, key_code: i32) -> i32 {
        return self.key_state[key_code as usize];
    }
}
