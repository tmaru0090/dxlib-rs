mod dxlib_wrapper;
mod my_file;
pub use dxlib_wrapper::*;

#[cfg(test)]
mod tests {
    use crate::dxlib_wrapper::{
        dx_common::dxlib::*, dx_error::*, dx_event::*, dx_font::*, dx_fps::*, dx_input::*,
        dx_keyboard::*, dx_log::*, dx_math::*, dx_music::*, dx_sound::*, dx_window::*,
    };
    const TEST_MIN_WINDOW_WIDTH: i32 = 640;
    const TEST_MIN_WINDOW_HEIGHT: i32 = 480;
    const TEST_MAX_WINDOW_WIDTH: i32 = 1280;
    const TEST_MAX_WINDOW_HEIGHT: i32 = 800;
    const TEST_WINDOW_TITLE: &str = "test-window";
    #[test]
    fn test() -> Result<(), DxErrorType> {
        unsafe {
            let mut key = DxKeyBoard::new();
            let mut fps = DxFps::new();
            let color = dx_GetColor(255, 255, 255);
            let mut rwindow = DxWindow::new();
            let window = rwindow.create_window(
                DxWindow::videomode(TEST_MAX_WINDOW_WIDTH, TEST_MAX_WINDOW_HEIGHT, 32),
                TEST_WINDOW_TITLE,
                DxWindowStyle::default(),
            )?;
            while window.is_open() {
                key.update()?;
                dx_ClearDrawScreen();
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
                fps.draw(dx_GetColor(255, 0, 255));
                fps.wait()?;
                dx_ScreenFlip();
            }
        }
        Ok(())
    }
}
