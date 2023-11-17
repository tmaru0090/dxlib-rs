extern crate alloc;
use crate::dx_common::dxlib::*;
use crate::my_file::MyFile;
use alloc::rc::Rc;
//ビデオモード時のデータ
pub struct DxVideoModeData {
    width: i32,    //クライアントサイズのwidth
    height: i32,   //クライアントサイズのheight
    colorbit: i32, //カラービット
}
//ウィンドウプロパティ
pub struct DxWindowData {}
pub enum DxWindowStyle {}
pub struct DxWindow {
    videomode_data: DxVideoModeData,
    title: String,
    window_dest_flag: bool,
}

//ウィンドウクラス
impl DxWindow {
    //DxLib_Init()を呼出し後の初期化処理(デフォルト値を使う場合のみ)
    pub fn initialize(&self) {
        unsafe {
            dx_SetDrawScreen(DX_SCREEN_BACK);
            dx_SetUse3DFlag(TRUE);
            dx_SetAlwaysRunFlag(TRUE);
        }
    }
    //DxLibの終了処理
    pub fn finalize(&self) {
        unsafe {}
    }
    //新しくウィンドウ情報を空で初期化し、スマートポインタ返す
    pub fn new() -> Box<DxWindow> {
        Box::new(DxWindow {
            videomode_data: DxVideoModeData {
                width: 0,
                height: 0,
                colorbit: 0,
            },
            title: String::new(),
            window_dest_flag: false,
        })
    }

    //新しくウィンドウを作成
    pub fn create_window(&mut self, videomode: DxVideoModeData, title: &str) -> &mut DxWindow {
        unsafe {
            //Dxlib_Initを呼ぶ前に必要な処理
            {
                dx_SetWaitVSyncFlag(TRUE);
                dx_SetUseCharCodeFormat(DX_CHARCODEFORMAT_UTF8);
                dx_SetMainWindowText(title);
                dx_SetGraphMode(videomode.width, videomode.height, videomode.colorbit, 120);
                dx_ChangeWindowMode(TRUE);
            }
            let dxinit = self.dx_init();
            match dxinit {
                Ok(val) => {
                    MyFile::new("./game.GAME_LOG").file_write_all(val);
                }
                Err(err) => {
                    MyFile::new("./game.GAME_LOG").file_write_all(err);
                }
            }
            self.initialize();
        }
        self
    }
    // DxLib_End関数のエラーハンドリング
    pub fn dx_end(&self) -> Result<String, String> {
        unsafe {
            if dx_DxLib_End() == -1 {
                Err("DxLib_End: Error Please! open Log.txt immediately\n".to_string())
            } else {
                Ok("DxLib_End: Success!\n".to_string())
            }
        }
    }
    // DxLib_End関数のエラーハンドリング
    pub fn dx_init(&self) -> Result<String, String> {
        unsafe {
            if dx_DxLib_Init() == -1 {
                Err("DxLib_Init: Error Please! open Log.txt immediately\n".to_string())
            } else {
                Ok("DxLib_Init: Success!\n".to_string())
            }
        }
    }

    pub fn is_dest(&self) -> bool {
        return self.window_dest_flag;
    }
    //ウィンドウが開いてるかどうか判定する
    pub fn is_open(&mut self) -> bool {
        unsafe {
            if dx_ProcessMessage() == 0 {
                return true;
            }
            return false;
        }
    }
    pub fn videomode(width: i32, height: i32, color_bit: i32) -> DxVideoModeData {
        DxVideoModeData {
            width: width,
            height: height,
            colorbit: color_bit,
        }
    }
    pub fn draw_and_update<F>(&self, mut callback_update: &mut F, mut callback_draw: &mut F)
    where
        F: FnMut() -> (),
    {
        unsafe {
            dx_ClearDrawScreen();
            callback_update();
            callback_draw();
            dx_ScreenFlip();
        }
    }
}
impl Drop for DxWindow {
    fn drop(&mut self) {
        self.finalize();
        unsafe {
            let dxend = self.dx_end();
            match dxend {
                // 失敗したときの処理
                Ok(val) => {
                    MyFile::new("./game.GAME_LOG").file_write_append(val);
                }
                Err(err) => {
                    MyFile::new("./game.GAME_LOG").file_write_append(err);
                }
            }
        }
    }
}
