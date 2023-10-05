use crate::dx_common::dxlib::*;
#[derive(Clone)]
pub struct DxKeyBoard {
    key_state: [i32; 256],
    key_buf: [i8; 256],
    press_cnt: i32,
    release_cnt: i32,
}
impl DxKeyBoard {
    pub fn new() -> Box<DxKeyBoard> {
        return Box::new(DxKeyBoard {
            key_state: [0; 256],
            key_buf: [0; 256],
            press_cnt: 0,
            release_cnt: 0,
        });
    }
    pub fn update(&mut self) {
        unsafe {
            // キーの状態を取得
            dx_GetHitKeyStateAll(self.key_buf.as_mut_ptr());
            // キーの状態を更新
            for i in 0..256 {
                if self.key_buf[i] == 1 {
                    self.key_state[i] += 1;
                    self.press_cnt += 1;
                    if self.release_cnt > 0 {
                        self.release_cnt = 0;
                    }
                } else {
                    self.key_state[i] = 0;
                    self.release_cnt += 1;
                    if self.press_cnt > 0 {
                        self.press_cnt = 0;
                    }
                }
            }
        }
    }
    pub fn get_key_state(&self, key_code: i32) -> (i32, i32, i32) {
        return (
            self.key_state[key_code as usize],
            self.press_cnt,
            self.release_cnt,
        );
    }
}
