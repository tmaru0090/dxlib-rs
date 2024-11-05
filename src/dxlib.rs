use crate::common::error::DxError;
use crate::ffi::dxlib::*;

use anyhow::anyhow;
use anyhow::Result as Res;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
pub fn get_drag_file_num() -> Res<i32> {
    let res = unsafe { dx_GetDragFileNum() };
    if res == -1 {
        return Err(anyhow!(DxError::new("", res)));
    } else {
        return Ok(res);
    }
}
pub fn set_font_size(size: i32) -> Res<i32> {
    let res = unsafe { dx_SetFontSize(size) };
    if res == -1 {
        return Err(anyhow!(DxError::new("", res)));
    } else {
        return Ok(res);
    }
}
pub fn change_font(font_name: &str) -> Res<i32> {
    let res = unsafe { dx_ChangeFont(font_name) };
    if res == -1 {
        return Err(anyhow!(DxError::new("", res)));
    } else {
        return Ok(res);
    }
}
pub fn set_out_application_log_valid_flag(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetOutApplicationLogValidFlag(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("", res)));
    } else {
        return Ok(res);
    }
}
pub fn get_drag_file_path(file_path_buffer: &mut String) -> Res<i32> {
    // バッファのサイズを指定
    let mut buf = vec![0_u8; 1000]; // 1000バイトのバッファを作成
    let buf_ptr = buf.as_mut_ptr() as *mut c_char; // C用のポインタに変換

    // C関数を呼び出してファイルパスを取得
    let res = unsafe {
        dx_GetDragFilePath(buf_ptr) // C関数を呼び出す
    };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ドロップアンドドロップされたファイルはありません",
            res
        )));
    }
    // Cの文字列からRustのStringに変換
    let path_cstr = unsafe { CStr::from_ptr(buf_ptr) }; // Cの文字列を取得
    let path_string = path_cstr.to_string_lossy().into_owned(); // RustのStringに変換
                                                                // 引数のcurrent_pathを更新
    *file_path_buffer = path_string; // 引数のStringを更新
    Ok(res)
}
pub fn set_drag_file_valid_flag(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetDragFileValidFlag(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("", res)));
    } else {
        return Ok(res);
    }
}
pub fn create_font_to_handle(font_name: &str, size: i32, thick: i32, font_type: i32) -> Res<i32> {
    let res = unsafe { dx_CreateFontToHandle(font_name, size, thick, font_type) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "フォントハンドルの生成に失敗しました",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn draw_string_to_handle(
    x: i32,
    y: i32,
    string: &str,
    color: u32,
    font_handle: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawStringToHandle(x, y, string, color, font_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "フォントつきの文字列の描画に失敗しました",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ウィンドウサイズを変更する
pub fn set_window_size(width: i32, height: i32) -> Res<i32> {
    let res = unsafe { dx_SetWindowSize(width, height) };
    if res == -1 {
        return Err(anyhow!(DxError::new(" Erorr", res)));
    } else {
        return Ok(res);
    }
}
/// ウィンドウのアクティブ状態を変更する
pub fn set_always_run_flag(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetAlwaysRunFlag(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(" Erorr", res)));
    } else {
        return Ok(res);
    }
}

pub fn clear_draw_screen() -> Res<i32> {
    let res = unsafe { dx_ClearDrawScreen() };
    if res == -1 {
        return Err(anyhow!(DxError::new(" Erorr", res)));
    } else {
        return Ok(res);
    }
}
pub fn set_graph_mode(size_x: i32, size_y: i32, color_bit_num: i32, refresh_rate: i32) -> Res<i32> {
    let res = unsafe { dx_SetGraphMode(size_x, size_y, color_bit_num, refresh_rate) };
    if res == -1 {
        return Err(anyhow!(DxError::new("Erorr", res)));
    } else {
        return Ok(res);
    }
}

pub fn screen_flip() -> Res<i32> {
    let res = unsafe { dx_ScreenFlip() };
    if res == -1 {
        return Err(anyhow!(DxError::new("Erorr", res)));
    } else {
        return Ok(res);
    }
}
pub fn set_draw_screen(draw_screen: i32) -> Res<i32> {
    let res = unsafe { dx_SetDrawScreen(draw_screen) };
    if res == -1 {
        return Err(anyhow!(DxError::new("Error", res)));
    } else {
        return Ok(res);
    }
}

pub fn dxlib_init() -> Res<i32> {
    let res = unsafe { dx_DxLib_Init() };
    if res == -1 {
        return Err(anyhow!(DxError::new("DxLibの初期化に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn dxlib_end() -> Res<i32> {
    let res = unsafe { dx_DxLib_End() };
    if res == -1 {
        return Err(anyhow!(DxError::new("DxLibの終了に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn process_message() -> Res<i32> {
    let res = unsafe { dx_ProcessMessage() };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "エラーが発生したか、ウィンドウが閉じられました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn draw_string(x: i32, y: i32, string: &str, color: u32) -> Res<i32> {
    let res = unsafe { dx_DrawString(x, y, string, color) };
    if res == -1 {
        return Err(anyhow!(DxError::new("文字列の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn get_color(red: i32, green: i32, blue: i32) -> Option<u32> {
    if red < 0 || green < 0 || blue < 0 || red > 255 || green > 255 || blue > 255 {
        return None;
    }
    Some(unsafe { dx_GetColor(red, green, blue) })
}
/// ウィンドウモードの変更
pub fn change_window_mode(flag: i32) -> Res<i32> {
    let res = unsafe { dx_ChangeWindowMode(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ウィンドウモードの変更に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

pub fn set_use_charcode_format(char_code_format: i32) -> Res<i32> {
    let res = unsafe { dx_SetUseCharCodeFormat(char_code_format) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "文字コードの変更に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
// グラフィックデータ制御関数

/// 画像ファイルのメモリへの読み込み、及び動画ファイルのロード
pub fn load_graph(file_name: &str) -> Res<i32> {
    let res = unsafe { dx_LoadGraph(file_name) };
    if res == -1 {
        return Err(anyhow!(DxError::new("画像の読み込みに失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
/// メモリに読み込んだグラフィックの描画
pub fn draw_graph(x: i32, y: i32, gr_handle: i32, trans_flag: i32) -> Res<i32> {
    let res = unsafe { dx_DrawGraph(x, y, gr_handle, trans_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みの画像の描画に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// メモリに読み込んだグラフィックの拡大縮小描画
pub fn draw_extend_graph(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawExtendGraph(x1, y1, x2, y2, gr_handle, trans_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("画像の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

/// メモリに読み込んだグラフィックの回転描画
pub fn draw_rota_graph(
    x: i32,
    y: i32,
    ext_rate: f64,
    angle: f64,
    gr_handle: i32,
    trans_flag: i32,
    turn_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawRotaGraph(x, y, ext_rate, angle, gr_handle, trans_flag, turn_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("画像の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

/// ロード済みのグラフィックのLR反転描画
pub fn draw_turn_graph(x: i32, y: i32, gr_handle: i32, trans_flag: i32) -> Res<i32> {
    let res = unsafe { dx_DrawTurnGraph(x, y, gr_handle, trans_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みの画像の描画に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// ロード済みのグラフィックのサイズの取得
pub fn get_graph_size(gr_handle: i32, size_xbuf: &mut i32, size_ybuf: &mut i32) -> Res<i32> {
    let res = unsafe { dx_GetGraphSize(gr_handle, size_xbuf, size_ybuf) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みの画像のサイズの取得に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

// 図形描画関数

/// 線を描画
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, color: Color) -> Res<i32> {
    let res = unsafe { dx_DrawLine(x1, y1, x2, y2, color) };
    if res == -1 {
        return Err(anyhow!(DxError::new("線の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
/// 線を描画(アンチエイリアス効果付き)
pub fn draw_line_aa(x1: f32, y1: f32, x2: f32, y2: f32, color: Color) -> Res<i32> {
    let res = unsafe { dx_DrawLineAA(x1, y1, x2, y2, color) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "アンチエイリアス付き線の描画に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// 四角を描画
pub fn draw_box(x1: i32, y1: i32, x2: i32, y2: i32, color: Color, fill_flag: i32) -> Res<i32> {
    let res = unsafe { dx_DrawBox(x1, y1, x2, y2, color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("四角形の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
/// 四角を描画(アンチエイリアス効果付き)
pub fn draw_box_aa(x1: f32, y1: f32, x2: f32, y2: f32, color: Color, fill_flag: i32) -> Res<i32> {
    let res = unsafe { dx_DrawBoxAA(x1, y1, x2, y2, color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "アンチエイリアス付き四角形の描画に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// 点の描画
pub fn draw_pixel(x: i32, y: i32, color: u32) -> Res<i32> {
    let res = unsafe { dx_DrawPixel(x, y, color) };
    if res == -1 {
        return Err(anyhow!(DxError::new("点の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
/// 指定点の色を取得
pub fn get_pixel(x: i32, y: i32) -> Res<i32> {
    let res = unsafe { dx_GetPixel(x, y) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "指定座標のピクセルの取得に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

// 円の描画
pub fn draw_circle(x: i32, y: i32, r: i32, color: u32, fill_flag: i32) -> Res<i32> {
    let res = unsafe { dx_DrawCircle(x, y, r, color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("円の描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

//３Ｄ図形描画関係関数

pub fn draw_line_3d(pos1: VECTOR, pos2: VECTOR, color: u32) -> Res<i32> {
    let res = unsafe { dx_DrawLine3D(pos1, pos2, color) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn draw_triangle_3d(
    pos1: VECTOR,
    pos2: VECTOR,
    pos3: VECTOR,
    color: u32,
    fill_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawTriangle3D(pos1, pos2, pos3, color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

pub fn draw_sphere_3d(
    center_pos: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawSphere3D(center_pos, r, div_num, spc_color, spc_color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

pub fn draw_capsule_3d(
    pos1: VECTOR,
    pos2: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawCapsule3D(pos1, pos2, r, div_num, dif_color, spc_color, fill_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

pub fn draw_cone_3d(
    top_pos: VECTOR,
    bottom_pos: VECTOR,
    r: f32,
    div_num: i32,
    dif_color: u32,
    spc_color: u32,
    fill_flag: i32,
) -> Res<i32> {
    let res = unsafe {
        dx_DrawCone3D(
            top_pos, bottom_pos, r, div_num, dif_color, spc_color, fill_flag,
        )
    };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

pub fn draw_bill_board_3d(
    pos: VECTOR,
    cx: f32,
    cy: f32,
    size: f32,
    angle: f32,
    gr_handle: i32,
    trans_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawBillboard3D(pos, cx, cy, size, angle, gr_handle, trans_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn draw_polygon_indexed_3d(
    vertex: *mut VERTEX3D,
    vertex_num: i32,
    indices: *mut u16,
    polygon_num: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Res<i32> {
    let res = unsafe {
        dx_DrawPolygonIndexed3D(
            vertex,
            vertex_num,
            indices,
            polygon_num,
            gr_handle,
            trans_flag,
        )
    };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn dx_draw_polygon_3d(
    vertex: *mut VERTEX3D,
    polygon_num: i32,
    gr_handle: i32,
    trans_flag: i32,
) -> Res<i32> {
    let res = unsafe { dx_DrawPolygon3D(vertex, polygon_num, gr_handle, trans_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new("3d描画に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

pub fn set_material_use_vert_dif_color(use_flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetMaterialUseVertDifColor(use_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "頂点カラー使用フラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_material_use_vert_spc_color(use_flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetMaterialUseVertSpcColor(use_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "スペシャルカラー使用フラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_material_param(material: MATERIALPARAM) -> Res<i32> {
    let res = unsafe { dx_SetMaterialParam(material) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "マテリアルの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

// Zバッファを使うかどうかのフラグ
pub fn set_use_zbuffer_3d(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetUseZBuffer3D(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "Zバッファ使用フラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
// Zバッファへの書き込みするかどうかのフラグ
pub fn set_write_zbuffer_3d(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetWriteZBuffer3D(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "Zバッファ書き込みフラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_use_back_culling(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetUseBackCulling(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "バックカリング使用フラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_texture_address_mode_uv(mode_u: i32, mode_v: i32) -> Res<i32> {
    let res = unsafe { dx_SetTextureAddressModeUV(mode_u, mode_v) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "テクスチャのアドレスモード変更に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_enable(flag: i32) -> Res<i32> {
    let res = unsafe { dx_SetFogEnable(flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "フォグ使用フラグの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_color(red: i32, green: i32, blue: i32) -> Res<i32> {
    let res = unsafe { dx_SetFogColor(red, green, blue) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "フォグの色の変更に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn set_fog_start_end(start: f32, end: f32) -> Res<i32> {
    let res = unsafe { dx_SetFogStartEnd(start, end) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "フォグの開始距離または終了距離の設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn get_colorf(red: f32, green: f32, blue: f32, alpha: f32) -> Option<COLOR_F> {
    if red < 0.0 || green < 0.0 || blue < 0.0 || red > 255.0 || green > 255.0 || blue > 255.0 {
        return None;
    }
    let res = unsafe { dx_GetColorF(red, green, blue, alpha) };
    return Some(res);
}
pub fn get_coloru8(red: i32, green: i32, blue: i32, alpha: i32) -> Option<COLOR_U8> {
    if red < 0 || green < 0 || blue < 0 || red > 255 || green > 255 || blue > 255 {
        return None;
    }

    let res = unsafe { dx_GetColorU8(red, green, blue, alpha) };

    return Some(res);
}

/// ミリ秒単位の精度を持つカウンタの現在値を得る
pub fn get_now_count() -> i32 {
    let res = unsafe { dx_GetNowCount() };
    return res;
}
/// GetNowCountの高精度バージョン
pub fn get_now_hiperformance_count() -> i64 {
    let res = unsafe { dx_GetNowHiPerformanceCount() };
    return res;
}

/// 指定の時間だけ処理をとめる
pub fn wait_timer(wait_time: i32) -> Res<i32> {
    let res = unsafe { dx_WaitTimer(wait_time) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "タイマーの待機処理に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ディスプレイの垂直同期信号を指定回数待つ
//pub fn dx_WaitVSync(async_num:i32) -> i32;
/// キーの入力待ち
pub fn wait_key() -> i32 {
    let res = unsafe { dx_WaitKey() };
    return res;
}

// 音楽再生関数

/// メモリに読み込んだサウンドファイルの読み込み
pub fn load_sound_mem(file_name: &str) -> Res<i32> {
    let res = unsafe { dx_LoadSoundMem(file_name) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "サウンドの読み込みに失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// メモリに読み込んだサウンドファイルの再生
pub fn play_sound_mem(music_handle: i32, play_type: i32, top_position_flag: i32) -> Res<i32> {
    let res = unsafe { dx_PlaySoundMem(music_handle, play_type, top_position_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みのサウンドの再生に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// メモリに読み込んだサウンドファイルの削除
pub fn delete_sound_mem(music_handle: i32) -> Res<i32> {
    let res = unsafe { dx_DeleteSoundMem(music_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みのサウンドの削除に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// メモリに読み込んだ音楽ファイルの読み込み
pub fn load_music_mem(file_name: &str) -> Res<i32> {
    let res = unsafe { dx_LoadMusicMem(file_name) };
    if res == -1 {
        return Err(anyhow!(DxError::new("音楽の読み込みに失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
/// メモリに読み込んだ音楽ファイルの再生
pub fn play_music_mem(music_handle: i32, play_type: i32, top_position_flag: i32) -> Res<i32> {
    let res = unsafe { dx_PlayMusicMem(music_handle, play_type, top_position_flag) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みの音楽の再生に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// メモリに読み込んだ音楽ファイルの削除
pub fn delete_music_mem(music_handle: i32) -> Res<i32> {
    let res = unsafe { dx_DeleteMusicMem(music_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みの音楽の削除に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルを演奏(再生)する
pub fn play_music(file_name: &str, play_type: i32) -> Res<i32> {
    let res = unsafe { dx_PlayMusic(file_name, play_type) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "MIDIファイルまたはMP3ファイルの再生に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルが演奏(再生)中かの情報を取得する
pub fn check_music() -> Res<i32> {
    let res = unsafe { dx_CheckMusic() };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "MIDIファイルまたはMP3ファイルの再生中の情報取得に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)停止
pub fn stop_music() -> Res<i32> {
    let res = unsafe { dx_StopMusic() };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "MIDIファイルまたはMP3ファイルの停止に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// ＭＩＤＩ又はＭＰ３ファイルの演奏(再生)の音量を設定する
pub fn set_volume_music(volume: i32) -> Res<i32> {
    let res = unsafe { dx_SetVolumeMusic(volume) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "MIDIファイルまたはMP3ファイルの音量設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
/// サウンドハンドルの再生位置をバイト単位で取得する
pub fn get_sound_current_position(sound_handle: i32) -> Res<i32> {
    let res = unsafe { dx_GetSoundCurrentPosition(sound_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "音声ファイルの再生位置の取得に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// サウンドハンドルの再生位置をミリ秒単位で取得する
/// (無圧縮wav,Ogg以外の形式の場合は正しい値が返ってこない場合があります)
pub fn get_sound_current_time(sound_handle: i32) -> Res<i32> {
    let res = unsafe { dx_GetSoundCurrentTime(sound_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "音声ファイルの再生位置の取得に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}

/// 現在時刻を取得する
pub fn get_date_time(data_buf: &mut DATEDATA) -> Res<i32> {
    let res = unsafe { dx_GetDateTime(data_buf) };
    if res == -1 {
        return Err(anyhow!(DxError::new("現在時刻の取得に失敗しました。", res)));
    } else {
        return Ok(res);
    }
}

/// 乱数を取得する
pub fn get_rand(rand_max: i32) -> i32 {
    let res = unsafe { dx_GetRand(rand_max) };
    return res;
}
/// 乱数の初期値を設定する
pub fn s_rand(seed: i32) -> i32 {
    let res = unsafe { dx_SRand(seed) };
    return res;
}

pub fn set_main_window_text(window_text: &str) -> Res<i32> {
    let res = unsafe { dx_SetMainWindowText(window_text) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ウィンドウタイトルの設定に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn mv1_load_model(file_name: &str) -> Res<i32> {
    let res = unsafe { dx_MV1LoadModel(file_name) };
    if res == -1 {
        return Err(anyhow!(DxError::new("モデルのロードに失敗しました。", res)));
    } else {
        return Ok(res);
    }
}
pub fn mv1_delete_model(m_handle: i32) -> Res<i32> {
    let res = unsafe { dx_MV1DeleteModel(m_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みのモデルの削除に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
pub fn mv1_draw_model(m_handle: i32) -> Res<i32> {
    let res = unsafe { dx_MV1DrawModel(m_handle) };
    if res == -1 {
        return Err(anyhow!(DxError::new(
            "ロード済みのモデルの描画に失敗しました。",
            res
        )));
    } else {
        return Ok(res);
    }
}
