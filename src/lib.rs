mod dxlib_wrapper;
mod my_file;
pub use dxlib_wrapper::*;

#[cfg(test)]
mod tests {
    use crate::dxlib_wrapper::*;
    use dx_common::dxlib::*;
    use dx_fps::*;
    use dx_keyboard::*;
    use dx_window::*;
    const TEST_MIN_WINDOW_WIDTH: i32 = 640;
    const TEST_MIN_WINDOW_HEIGHT: i32 = 480;
    const TEST_MAX_WINDOW_WIDTH: i32 = 1280;
    const TEST_MAX_WINDOW_HEIGHT: i32 = 800;
    const TEST_WINDOW_TITLE: &str = "test-window";
    #[test]
    fn test() {
        unsafe {
            let mut ref_win = DxWindow::new();
            let mut key = DxKeyBoard::new();
            let mut fps = DxFps::new();
            let color = dx_GetColor(255, 255, 255);
            let window = ref_win.create_window(
                DxWindow::videomode(TEST_MAX_WINDOW_WIDTH, TEST_MAX_WINDOW_HEIGHT, 32),
                TEST_WINDOW_TITLE,
            );
            while window.is_open() {
                key.update();
                dx_ClearDrawScreen();
                fps.draw();
                dx_DrawString(
                    0,
                    30,
                    &format!(
                        "key:press:{} key:release:{}",
                        key.get_press_cnt(KEY_INPUT_1),
                        key.get_release_cnt(KEY_INPUT_1)
                    ),
                    color,
                );
                fps.wait();
                dx_ScreenFlip();
            }
        }
    }
}
